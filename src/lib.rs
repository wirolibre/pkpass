//! pkpass

use models::Manifest;
use openssl::{
	pkcs7::{Pkcs7, Pkcs7Flags},
	stack::Stack,
	x509::store::X509StoreBuilder,
};
use sign::Identity;
use std::{
	collections::HashMap,
	io::{self, Read, Seek, Write},
	str::FromStr,
};
use zip::{result::ZipError, write::SimpleFileOptions, ZipArchive};

use crate::{
	models::{AssetContent, AssetType},
	sign::wwdr,
};

pub mod models;
pub mod sign;

#[derive(Debug)]
pub struct PkPass {
	pub pass: models::Pass,
	pub assets: models::AssetTable,
}

impl PkPass {
	pub fn new(description: String, serial_number: String) -> Self {
		Self {
			pass: models::Pass {
				app_launch_url: None,
				associated_store_identifiers: None,
				authentication_token: None,
				background_color: None,
				foreground_color: None,
				barcode: None,
				barcodes: None,
				beacons: None,
				boarding_pass: None,
				coupon: None,
				description,
				event_ticket: None,
				expiration_date: None,
				format_version: 1,
				generic: None,
				grouping_identifier: None,
				label_color: None,
				locations: None,
				logo_text: None,
				max_distance: None,
				nfc: None,
				// TODO: ugly
				organization_name: String::new(),
				pass_type_identifier: String::new(),
				relevant_date: None,
				semantics: None,
				serial_number,
				sharing_prohibited: None,
				store_card: None,
				suppress_strip_shine: None,
				team_identifier: String::new(),
				user_info: None,
				voided: None,
				web_service_url: None,
			},
			assets: HashMap::default(),
		}
	}

	// TODO: should indeed valitate pass
	pub fn read<R: Read + Seek>(reader: R) -> io::Result<Self> {
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

		if let Some(sig) = signature {
			let stack = Stack::new()?;

			let mut store = X509StoreBuilder::new()?;
			store.add_cert(wwdr::g4())?;
			let store = store.build();

			// TODO: fking understand and properly validate sig
			let _ = sig.verify(&stack, &store, Some(&manifest), None, Pkcs7Flags::empty());
		}

		let manifest: models::Manifest = serde_json::from_slice(&manifest)?;

		// TODO: verify manifest based on sig
		let pass: models::Pass = match zip.by_name("pass.json") {
			Ok(file) => serde_json::from_reader(file)?,
			Err(ZipError::FileNotFound) => todo!(),
			Err(e) => return Err(e.into()),
		};

		let mut assets = HashMap::new();

		for item in 0..zip.len() {
			let mut item = zip.by_index(item)?;

			if matches!(item.name(), "pass.json" | "manifest.json" | "signature") {
				continue;
			}
			if !item.is_file() {
				continue;
			}

			let item_kind = AssetType::from_str(item.name()).unwrap();

			let mut data = vec![];
			item.read_to_end(&mut data)?;

			assert!(manifest.verify_file(item.name(), &data));

			assets.insert(item_kind, AssetContent::new(data));
		}

		Ok(Self { pass, assets })
	}

	pub fn write<W: Write + Seek>(&mut self, identity: Identity, writer: W) -> io::Result<()> {
		self.pass.team_identifier = identity.team_id;
		self.pass.pass_type_identifier = identity.pass_type_id;
		self.pass.organization_name = identity.organization_name;
		// ---ugly---

		let mut manifest = Manifest::default();

		let mut zip = zip::ZipWriter::new(writer);
		let options =
			SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

		let pass_data = serde_json::to_vec(&self.pass)?;
		manifest.add_file("pass.json", &pass_data);
		zip.start_file("pass.json", options)?;
		zip.write_all(&pass_data)?;

		for (asset_kind, asset_content) in &self.assets {
			let asset_path = asset_kind.to_string();
			manifest.add_file(&asset_path, asset_content);
			zip.start_file(asset_path, options)?;
			zip.write_all(asset_content)?;
		}

		let manifest_data = serde_json::to_vec(&manifest)?;
		zip.start_file("manifest.json", options)?;
		zip.write_all(&manifest_data)?;

		if let Some(pen) = identity.pen {
			let mut certs = Stack::new()?;
			certs.push(pen.root_certificate)?;

			let signature = Pkcs7::sign(
				&pen.signer_certificate,
				&pen.signer_private_key,
				&certs,
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
