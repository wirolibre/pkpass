use openssl::{
	nid::Nid,
	pkcs12::ParsedPkcs12_2,
	pkey::{PKey, Private},
	stack::Stack,
	x509::{X509NameEntryRef, X509},
};
use std::{fmt, io};

#[derive(Debug)]
pub struct Identity {
	pub(crate) pen: Option<SigningPen>,
	pub(crate) pass_type_id: String,
	pub(crate) team_id: String,
}

impl Identity {
	#[must_use]
	pub fn from_apple_pen(pen: SigningPen) -> Option<Self> {
		fn get_str(op: Option<&'_ X509NameEntryRef>) -> Option<String> {
			Some(op?.data().as_utf8().ok()?.to_string())
		}

		let name = pen.signer_certificate.subject_name();

		let pass_type_id = get_str(name.entries_by_nid(Nid::USERID).next())?;
		let team_id = get_str(name.entries_by_nid(Nid::ORGANIZATIONALUNITNAME).next())?;

		Some(Self {
			pen: Some(pen),
			pass_type_id,
			team_id,
		})
	}
}

pub struct SigningPen {
	pub(crate) signer_private_key: PKey<Private>,
	pub(crate) signer_certificate: X509,
	pub(crate) chain: Stack<X509>,
}

impl fmt::Debug for SigningPen {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("SigningPen").finish_non_exhaustive()
	}
}

impl SigningPen {
	#[must_use]
	pub fn new(
		signer_private_key: PKey<Private>,
		signer_certificate: X509,
		chain: Stack<X509>,
	) -> Self {
		Self {
			signer_private_key,
			signer_certificate,
			chain,
		}
	}

	#[must_use]
	pub fn from_pkcs12(pkcs12: ParsedPkcs12_2) -> io::Result<Self> {
		let invalid_input = |msg: &str| io::Error::new(io::ErrorKind::InvalidInput, msg);

		let signer_private_key = pkcs12
			.pkey
			.ok_or_else(|| invalid_input("archive has to contain a private key"))?;
		let signer_certificate = pkcs12
			.cert
			.ok_or_else(|| invalid_input("archive has to contain a certificate"))?;
		let chain = pkcs12
			.ca
			.ok_or_else(|| invalid_input("archive has to contain a chain of trust"))?;

		Ok(Self {
			signer_private_key,
			signer_certificate,
			chain,
		})
	}
}

/// *Worldwide Developer Relations* Apple certificates
pub mod wwdr {
	use openssl::x509::X509;

	const G4: &[u8; 1113] = include_bytes!("AppleWWDRCAG4.cer");

	#[must_use]
	pub fn g4() -> X509 {
		X509::from_der(G4)
			.unwrap_or_else(|_| unreachable!("bundled Apple WWDR G4 certificate is valid"))
	}

	#[test]
	fn g4_cert_valid() {
		let _ = g4();
	}
}
