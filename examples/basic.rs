use clap::error::Result;
use openssl::{pkey::PKey, x509::X509};
use pkpass::{
	models::{AssetContent, AssetType, EventTicket, Image, Version},
	sign::{Identity, SigningPen},
	PkPass,
};
use std::{fs, io};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let identity = get_identity()?;

	let mut pass = PkPass::new(
		"A custom pass to try out my library".into(),
		Uuid::new_v4().as_simple().to_string(),
	);

	pass.pass.event_ticket = Some(EventTicket {});

	pass.assets.insert(
		AssetType::Image(Image::Icon(Version::Standard)),
		AssetContent::new(include_bytes!("../icon.png").to_vec()),
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
	let cert = fs::read_to_string("certs/signerCert.pem")?;
	let cert_err = "Provided signer cert is not a valid cert in PEM format";
	let cert = X509::from_pem(cert.as_bytes())
		.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, cert_err))?;
	let key = fs::read_to_string("certs/signerKey.pem")?;
	let key_err = "Provided signer key is not a valid private key in PEM format";
	let key = PKey::private_key_from_pem(key.as_bytes())
		.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, key_err))?;

	let pen = SigningPen::new_apple(cert, key);
	let identity = Identity::from_apple_pen(pen).unwrap();

	Ok(identity)
}
