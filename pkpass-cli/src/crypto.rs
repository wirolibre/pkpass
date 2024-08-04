use openssl::{nid::Nid, pkcs12::Pkcs12, pkey::PKey, rsa::Rsa, stack::Stack, x509::X509};
use pkpass::sign::certificates;
use rcgen::CertificateParams;
use std::{fs, path::PathBuf};

use crate::Exec;

/// Multiple helpers to help generate a signing certificate for pkpasses
#[derive(clap::Subcommand)]
pub(crate) enum Command {
	// TODO: would be nice to generate Ed25516 keys but couldn't make Apple
	//       accept a CSR signed with such a key
	/// Generates a private key and writes a `PEM`-encoded version to disc.
	Key(KeyArgs),

	/// Create a certificate request file using the given private key.
	///
	/// Useful for submitting CSRs to Apple.
	Request(CertificateRequestArgs),

	/// Creates an unencrypted [PKCS#12] certificate bundle with everything needed to sign `pkpass` files.
	///
	/// [PKCS#12]: https://en.wikipedia.org/wiki/PKCS_12
	Bundle(BundleCertArgs),
}

impl Exec for Command {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		match self {
			Self::Key(args) => args.run(),
			Self::Request(args) => args.run(),
			Self::Bundle(args) => args.run(),
		}
	}
}

#[derive(clap::Args)]
pub(crate) struct KeyArgs {
	/// Private Key destination file (e.g. pkpass.key)
	#[clap(long, short)]
	output: PathBuf,
}

impl Exec for KeyArgs {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		let new_keypair: PKey<_> = Rsa::generate(2048)?.try_into()?;
		fs::write(&self.output, new_keypair.private_key_to_pem_pkcs8()?)?;
		println!("Wrote private key to `{}`", self.output.display());
		Ok(())
	}
}

#[derive(clap::Args)]
pub(crate) struct CertificateRequestArgs {
	/// Read the private key from its PEM-encoded form (e.g. pkpass.key)
	#[clap(long)]
	private_key: PathBuf,

	/// Certificate Signing Request PEM-encoded destination file (e.g. pkpass.csr)
	#[clap(long, short)]
	output: PathBuf,
}

impl Exec for CertificateRequestArgs {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		let content = fs::read_to_string(self.private_key)?;
		let keypair = rcgen::KeyPair::from_pem(&content)?;

		let params = CertificateParams::default();
		let csr = params.serialize_request(&keypair)?;

		fs::write(&self.output, csr.pem()?)?;
		println!(
			"Wrote certificate signing request to `{}`",
			self.output.display()
		);

		println!("Head over to https://developer.apple.com/account/resources/certificates/list to submit your certificate request.");

		Ok(())
	}
}

#[derive(clap::Args)]
pub(crate) struct BundleCertArgs {
	/// Read the private key from its PEM-encoded form (e.g. pkpass.key)
	#[clap(long)]
	private_key: PathBuf,
	/// Read the X509 certificate in its DER-encoded form (e.g. pass.cer)
	#[clap(long)]
	certificate: PathBuf,

	/// PKCS#12 archive DER-encoded destination file (e.g. pkpass.p12)
	#[clap(long, short)]
	output: PathBuf,
}

impl Exec for BundleCertArgs {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		let cert = fs::read(&self.private_key)?;
		let pkey = PKey::private_key_from_pem(&cert)?;

		let cert = fs::read(&self.certificate)?;
		let cert = X509::from_der(&cert)?;

		let mut chain = Stack::new()?;
		chain.push(certificates::apple_wwdr_g4())?;

		let p12 = {
			let mut p12 = Pkcs12::builder();
			p12.pkey(&pkey);
			p12.cert(&cert);
			p12.ca(chain);
			// Don't encrypt content so password is irrelevant
			p12.key_algorithm(Nid::UNDEF);
			p12.cert_algorithm(Nid::UNDEF);
			p12.build2("")?
		};

		fs::write(&self.output, p12.to_der()?)?;
		println!("Wrote pkcs#12 certs bundle to `{}`", self.output.display());

		Ok(())
	}
}
