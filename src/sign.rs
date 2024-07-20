use std::fmt;

use openssl::{
	nid::Nid,
	pkey::{PKey, Private},
	x509::{X509NameEntryRef, X509},
};

#[derive(Debug)]
pub struct Identity {
	pub(crate) organization_name: String,
	pub(crate) pass_type_id: String,
	pub(crate) team_id: String,
	pub(crate) pen: Option<SigningPen>,
}

impl Identity {
	#[must_use]
	pub fn from_apple_pen(pen: SigningPen) -> Option<Self> {
		fn get_str(op: Option<&'_ X509NameEntryRef>) -> Option<String> {
			Some(op?.data().as_utf8().ok()?.to_string())
		}

		let name = pen.signer_certificate.subject_name();

		let organization_name = get_str(name.entries_by_nid(Nid::ORGANIZATIONNAME).next())?;
		let pass_type_id = get_str(name.entries_by_nid(Nid::USERID).next())?;
		let team_id = get_str(name.entries_by_nid(Nid::ORGANIZATIONALUNITNAME).next())?;

		Some(Self {
			organization_name,
			pass_type_id,
			team_id,
			pen: Some(pen),
		})
	}
}

// TODO: use zeroize?
pub struct SigningPen {
	pub(crate) root_certificate: X509,
	pub(crate) signer_certificate: X509,
	pub(crate) signer_private_key: PKey<Private>,
}

impl fmt::Debug for SigningPen {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("SigningPen").finish_non_exhaustive()
	}
}

impl SigningPen {
	#[must_use]
	pub fn new_apple(signer_certificate: X509, signer_private_key: PKey<Private>) -> Self {
		Self {
			root_certificate: wwdr::g4(),
			signer_certificate,
			signer_private_key,
		}
	}

	#[must_use]
	pub fn new_custom(
		root_certificate: X509,
		signer_certificate: X509,
		signer_private_key: PKey<Private>,
	) -> Self {
		Self {
			root_certificate,
			signer_certificate,
			signer_private_key,
		}
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
