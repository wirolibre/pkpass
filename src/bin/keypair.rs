use openssl::{pkey::PKey, x509::X509};
use rcgen::{CertificateParams, DistinguishedName, DnType};
use std::{
	fs,
	io::{self, stdin, Read},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	todo!();

	let keypair = PKey::generate_ed25519()?;
	let pem = String::from_utf8(keypair.private_key_to_pem_pkcs8()?)?;

	fs::write("pkpass.key", &pem)?;
	println!("Wrote private key to `pkpass.key`");

	let keypair = rcgen::KeyPair::from_pem(&pem)?;

	let mut dn = DistinguishedName::new();
	dn.push(DnType::OrganizationName, "Wiro Logiciel Libre");
	dn.push(DnType::CommonName, "wirolibre.xyz");
	dn.push(DnType::LocalityName, "Montreuil");
	dn.push(DnType::StateOrProvinceName, "Ile-De-France");
	dn.push(DnType::CountryName, "FR");

	let mut params = CertificateParams::default();
	params.distinguished_name = dn;

	fs::write("pkpass.csr", params.serialize_request(&keypair)?.pem()?)?;
	println!("Wrote certificate signing request to `pkpass.key`");

	println!("Head over to https://developer.apple.com/account/resources/certificates/list to submit your certificate request");

	let content = loop {
		println!("Waiting for Apple certificate... Type Enter when ready!");
		let mut buffer = [0u8; 1];
		// We just want to lock until we get some input to let the user the
		// time to retrive the cert though Apple Developper interface
		stdin().read_exact(&mut buffer)?;

		match fs::read("pass.cer") {
			Ok(c) => break c,
			Err(err) if err.kind() == io::ErrorKind::NotFound => {
				println!("`pass.cer` is not present, looping...");
				continue;
			}
			_ => panic!("smth else"),
		};
	};

	let cert = X509::from_der(&content)?;
	let pem = cert.to_pem()?;

	fs::write("pass.pem", pem)?;
	println!("Wrote `pass.pem`");

	Ok(())
}
