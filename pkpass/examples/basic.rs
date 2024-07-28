use openssl::pkcs12::Pkcs12;
use pkpass::{
	models::{
		fields::{EventTicket, PassFields, PassKind},
		manifest::{AssetContent, AssetType, Image, Version},
	},
	sign::{Identity, SigningPen},
	PkPass,
};
use std::{fs, io};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let identity = get_identity()?;

	let fields = PassFields::default();

	let mut pass = PkPass::new(
		"A custom pass to try out my library".into(),
		Uuid::new_v4().as_simple().to_string(),
		PassKind::EventTicket(EventTicket { fields }),
	);

	pass.assets.insert(
		AssetType::Image(Image::Icon(Version::Standard)),
		AssetContent::new(include_bytes!("assets/pinault-icon.png").to_vec()),
	);

	let file = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open("custom.pkpass")?;
	pass.write(identity, file)?;

	Ok(())
}

fn get_identity() -> io::Result<Identity> {
	let archive = fs::read("certs/pkpass.p12")?;
	let pkcs12_err = "Signing identity is not a valid PKCS#12 archive DER-encoded";
	let pkcs12 = Pkcs12::from_der(&archive)
		.map_err(|_err| io::Error::new(io::ErrorKind::InvalidData, pkcs12_err))?
		.parse2("")?;

	let pen = SigningPen::from_pkcs12(pkcs12)?;
	let identity = Identity::from_apple_pen(pen).unwrap();

	Ok(identity)
}
