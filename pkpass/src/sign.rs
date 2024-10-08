use openssl::{
	pkcs12::ParsedPkcs12_2,
	pkey::{PKey, Private},
	stack::Stack,
	x509::X509,
};
use std::{fmt, io, str::FromStr};

#[derive(Debug)]
pub struct Identity {
	pub(crate) pass_type_id: String,
	pub(crate) team_id: String,
	pub(crate) pen: Option<SigningPen>,
}

impl Identity {
	#[must_use]
	pub const fn new_no_signature(pass_type_id: String, team_id: String) -> Self {
		Self {
			pass_type_id,
			team_id,
			pen: None,
		}
	}

	#[cfg(feature = "apple")]
	pub fn from_apple_pen(pen: SigningPen) -> io::Result<Self> {
		use openssl::nid::Nid;

		let name = pen.signer_certificate.subject_name();

		let get_entry = |nid: Nid| {
			let op = name.entries_by_nid(nid).next();
			Some(op?.data().as_utf8().ok()?.to_string())
		};

		let pass_type_id = get_entry(Nid::USERID).ok_or_else(|| {
			io::Error::new(
				io::ErrorKind::NotFound,
				"could not find user id on apple cert",
			)
		})?;
		let team_id = get_entry(Nid::ORGANIZATIONALUNITNAME).ok_or_else(|| {
			io::Error::new(
				io::ErrorKind::NotFound,
				"could not find organization unit name on apple cert",
			)
		})?;

		Ok(Self {
			pass_type_id,
			team_id,
			pen: Some(pen),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyMode {
	No,

	// TODO: rename to `Apple` when other exist?
	#[cfg(feature = "apple")]
	Yes,
}

impl Default for VerifyMode {
	fn default() -> Self {
		#[cfg(feature = "apple")]
		return Self::Yes;
		#[cfg(not(feature = "apple"))]
		return Self::No;
	}
}

impl FromStr for VerifyMode {
	type Err = std::io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"no" => Ok(Self::No),

			#[cfg(feature = "apple")]
			"yes" => Ok(Self::Yes),

			_ => Err(io::Error::new(io::ErrorKind::InvalidInput, "")),
		}
	}
}

impl fmt::Display for VerifyMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::No => write!(f, "no"),

			#[cfg(feature = "apple")]
			Self::Yes => write!(f, "yes"),
		}
	}
}

pub mod certificates {
	#[cfg(feature = "apple")]
	use openssl::x509::X509;

	#[cfg(feature = "apple")]
	const APPLE_ROOT: &[u8; 1215] = include_bytes!("AppleIncRootCertificate.cer");
	#[cfg(feature = "apple")]
	const APPLE_WWDR_G4: &[u8; 1113] = include_bytes!("AppleWWDRCAG4.cer");

	/// *Worldwide Developer Relations* Apple certificates
	#[must_use]
	#[cfg(feature = "apple")]
	pub fn apple_root() -> X509 {
		X509::from_der(APPLE_ROOT)
			.unwrap_or_else(|_| unreachable!("bundled Apple Root certificate is valid"))
	}

	/// *Worldwide Developer Relations* Apple certificates
	#[must_use]
	#[cfg(feature = "apple")]
	pub fn apple_wwdr_g4() -> X509 {
		X509::from_der(APPLE_WWDR_G4)
			.unwrap_or_else(|_| unreachable!("bundled Apple WWDR G4 certificate is valid"))
	}

	#[test]
	#[cfg(feature = "apple")]
	fn apple_root_cert_valid() {
		let _ = apple_root();
	}

	#[test]
	#[cfg(feature = "apple")]
	fn apple_wwdr_g4_cert_valid() {
		let _ = apple_wwdr_g4();
	}
}
