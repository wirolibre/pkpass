#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("ManifestSignatureMismatch: `{0}` calculated signature didn't match the one in the manifest")]
	ManifestSignatureMismatch(String),

	// ---
	#[error("Zip: {0}")]
	Zip(#[from] zip::result::ZipError),

	#[error("Io: {0}")]
	Io(#[from] std::io::Error),

	#[error("OpenSsl: {0}")]
	OpenSsl(#[from] openssl::error::ErrorStack),

	#[error("Json: {0}")]
	Json(#[from] serde_json::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
