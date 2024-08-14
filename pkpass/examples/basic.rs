use openssl::pkcs12::Pkcs12;
use pkpass::{
	models::{EventTicket, Fields, PassKind},
	sign::{Identity, SigningPen},
	Pass, PassConfig,
};
use std::{fs, io};
use uuid::Uuid;

const ICON: &[u8; 314_069] = include_bytes!("assets/icon.png");

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let identity = get_identity()?;

	let fields = Fields::default();

	let mut pass = Pass::new(PassConfig {
		organization_name: "Acme Inc.".into(),
		description: "A custom pass to try out my library".into(),
		serial_number: Uuid::new_v4().as_simple().to_string(),
		kind: PassKind::EventTicket(EventTicket { fields }),
	});

	let Pass { assets, .. } = &mut pass;

	assets.images.icon.size_x1.replace(ICON.to_vec());

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
	let identity = Identity::from_apple_pen(pen)?;

	Ok(identity)
}
