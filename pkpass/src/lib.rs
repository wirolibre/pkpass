//! pkpass

use crate::{
	models::{spec, Assets, ColorTheme, Manifest, Metadata, PassKind},
	sign::{certificates, Identity, VerifyMode},
};
use openssl::{
	pkcs7::{Pkcs7, Pkcs7Flags},
	stack::Stack,
	x509::{store::X509StoreBuilder, X509PurposeId},
};
use std::{
	io::{Read, Seek, Write},
	mem,
};
use zip::{result::ZipError, write::SimpleFileOptions, ZipArchive};

mod error;
pub mod models;
pub mod sign;

pub use error::{Error, Result};

#[derive(Debug)]
pub struct Pass {
	pub metadata: Metadata,
	pub assets: Assets,
}

#[derive(Debug)]
pub struct PassConfig {
	pub organization_name: String,
	pub description: String,
	pub serial_number: String,
	pub kind: PassKind,
}

impl Pass {
	#[must_use]
	pub fn new(
		PassConfig {
			organization_name,
			description,
			serial_number,
			kind,
		}: PassConfig,
	) -> Self {
		Self {
			metadata: Metadata {
				format_version: 1,

				// TODO: not very clean, is filled later during signing
				pass_type_identifier: String::new(),
				team_identifier: String::new(),

				organization_name,
				description,
				serial_number,

				color_theme: ColorTheme::default(),
				kind,

				app_launch_url: None,
				associated_store_identifiers: Vec::default(),

				barcodes: Vec::default(),
				beacons: Vec::default(),
				expiration_date: None,
				grouping_identifier: None,
				locations: Vec::default(),
				logo_text: None,
				max_distance: None,
				nfc: None,
				relevant_date: None,
				semantics: None,
				sharing_prohibited: None,
				suppress_strip_shine: None,
				user_info: None,
				voided: None,

				web_service_url: None,
				authentication_token: None,
			},
			assets: Assets::default(),
		}
	}

	pub fn read<R: Read + Seek>(reader: R, verify: VerifyMode) -> Result<Self> {
		let mut zip = ZipArchive::new(reader)?;

		let signature = match zip.by_name("signature") {
			Ok(mut file) => {
				let mut signature = vec![];
				file.read_to_end(&mut signature)?;
				Some(Pkcs7::from_der(&signature)?)
			}
			Err(ZipError::FileNotFound) => None,
			Err(e) => return Err(e.into()),
		};

		// TODO: verify manifest based on sig
		let manifest = match zip.by_name("manifest.json") {
			Ok(mut file) => {
				let mut vec = vec![];
				file.read_to_end(&mut vec)?;
				vec
			}
			Err(ZipError::FileNotFound) => todo!(),
			Err(e) => return Err(e.into()),
		};

		if verify == VerifyMode::Yes {
			if let Some(sig) = signature {
				let stack = Stack::new()?;

				let store = {
					let mut store = X509StoreBuilder::new()?;
					store.add_cert(certificates::apple_root())?;
					store.add_cert(certificates::apple_wwdr_g4())?;
					store.set_purpose(X509PurposeId::ANY)?;
					store.build()
				};

				sig.verify(&stack, &store, Some(&manifest), None, Pkcs7Flags::empty())?;
			}
		}

		let manifest: Manifest = serde_json::from_slice(&manifest)?;

		let metadata: Metadata = match zip.by_name("pass.json") {
			Ok(file) => serde_json::from_reader::<_, spec::Metadata>(file)?.into(),
			Err(ZipError::FileNotFound) => todo!(),
			Err(e) => return Err(e.into()),
		};

		let mut assets = Assets::default();

		for item in 0..zip.len() {
			let mut item = zip.by_index(item)?;

			if matches!(item.name(), "pass.json" | "manifest.json" | "signature") {
				continue;
			}
			if !item.is_file() {
				continue;
			}

			let mut data = vec![];
			item.read_to_end(&mut data)?;

			// first check if asset is a valid one
			let asset = assets.get_mut(item.name())?;

			if !manifest.verify_file(item.name(), &data) {
				return Err(Error::ManifestSignatureMismatch(item.name().into()));
			}

			let _ = mem::replace(asset, data);
		}

		Ok(Self { metadata, assets })
	}

	pub fn write<W: Write + Seek>(&mut self, identity: Identity, writer: W) -> Result<()> {
		self.metadata.pass_type_identifier = identity.pass_type_id;
		self.metadata.team_identifier = identity.team_id;
		// ---ugly---

		let mut manifest = Manifest::default();

		let mut zip = zip::ZipWriter::new(writer);
		let options =
			SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

		let pass_data = serde_json::to_vec::<spec::Metadata>(&self.metadata.clone().into())?;
		manifest.add_file("pass.json", &pass_data);
		zip.start_file("pass.json", options)?;
		zip.write_all(&pass_data)?;

		for (asset_path, asset_content) in self.assets.paths() {
			manifest.add_file(&asset_path, asset_content);
			zip.start_file(asset_path, options)?;
			zip.write_all(asset_content)?;
		}

		let manifest_data = serde_json::to_vec::<Manifest>(&manifest)?;
		zip.start_file("manifest.json", options)?;
		zip.write_all(&manifest_data)?;

		if let Some(pen) = &identity.pen {
			let signature = Pkcs7::sign(
				&pen.signer_certificate,
				&pen.signer_private_key,
				&pen.chain,
				&manifest_data,
				Pkcs7Flags::DETACHED,
			)?;

			zip.start_file("signature", options)?;
			zip.write_all(&signature.to_der()?)?;
		}

		zip.finish()?;

		Ok(())
	}
}
