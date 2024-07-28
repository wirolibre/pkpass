use openssl::sha::Sha1;
use serde::{Deserialize, Serialize};
use std::{
	collections::{hash_map::Entry, HashMap},
	fmt,
	ops::Deref,
	str::FromStr,
};
use unic_langid::LanguageIdentifier;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Manifest {
	assets: HashMap<String, String>,
}

impl Manifest {
	pub(crate) fn add_file(&mut self, name: &str, data: &[u8]) {
		match self.assets.entry(name.to_owned()) {
			Entry::Occupied(_) => todo!(),
			Entry::Vacant(place) => {
				place.insert(sha1(data));
			}
		}
	}

	pub(crate) fn verify_file(&self, name: &str, data: &[u8]) -> bool {
		self.assets
			.get(name)
			.map_or(false, |sha| *sha == sha1(data))
	}
}

fn sha1(data: &[u8]) -> String {
	let mut hasher = Sha1::new();
	hasher.update(data);
	hex::encode(hasher.finish())
}

#[derive(PartialEq, Eq, Hash)]
pub enum AssetType {
	Image(Image),
	Localized(LanguageIdentifier, LocalizedAssetType),
}

impl fmt::Display for AssetType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Image(image) => write!(f, "{image}"),
			Self::Localized(lang, subasset) => write!(f, "{lang}.lproj/{subasset}"),
		}
	}
}

impl fmt::Debug for AssetType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Image(image) => write!(f, "Image({image:?})"),
			Self::Localized(lang, subasset) => write!(f, "Localized({lang}, {subasset:?})"),
		}
	}
}

impl FromStr for AssetType {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.split_once('/').map_or((s, None), |(a, b)| (a, Some(b))) {
			(one, None) => Ok(Self::Image(Image::from_str(one)?)),
			(one, Some(two)) => {
				let lang = one.strip_suffix(".lproj").unwrap();
				let lang = LanguageIdentifier::from_str(lang).unwrap();
				Ok(Self::Localized(lang, LocalizedAssetType::from_str(two)?))
			}
		}
	}
}

#[derive(PartialEq, Eq, Hash)]
pub enum LocalizedAssetType {
	Image(Image),
	/// `pass.strings`
	Strings,
}

impl fmt::Display for LocalizedAssetType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Image(image) => write!(f, "{image}"),
			Self::Strings => write!(f, "pass.strings"),
		}
	}
}

impl fmt::Debug for LocalizedAssetType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Image(image) => write!(f, "{image}"),
			Self::Strings => write!(f, "Strings"),
		}
	}
}

impl FromStr for LocalizedAssetType {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"pass.strings" => Ok(Self::Strings),
			s => Ok(Self::Image(Image::from_str(s).unwrap())),
		}
	}
}

#[derive(PartialEq, Eq, Hash)]
pub enum Image {
	/// The background image (background.png)
	Background(Version),
	/// The footer image (footer.png)
	Footer(Version),
	/// The icon (icon.png)
	Icon(Version),
	/// The logo image (logo.png)
	Logo(Version),
	/// The strip image (strip.png)
	Strip(Version),
	/// The thumbnail image (thumbnail.png)
	Thumbnail(Version),
}

impl fmt::Display for Image {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Background(v) => write!(f, "background{v}.png"),
			Self::Footer(v) => write!(f, "footer{v}.png"),
			Self::Icon(v) => write!(f, "icon{v}.png"),
			Self::Logo(v) => write!(f, "logo{v}.png"),
			Self::Strip(v) => write!(f, "strip{v}.png"),
			Self::Thumbnail(v) => write!(f, "thumbnail{v}.png"),
		}
	}
}

impl fmt::Debug for Image {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Background(v) => write!(f, "Background-{v:?}"),
			Self::Footer(v) => write!(f, "Footer-{v:?}"),
			Self::Icon(v) => write!(f, "Icon-{v:?}"),
			Self::Logo(v) => write!(f, "Logo-{v:?}"),
			Self::Strip(v) => write!(f, "Strip-{v:?}"),
			Self::Thumbnail(v) => write!(f, "Thumbnail-{v:?}"),
		}
	}
}

impl FromStr for Image {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let name = s.strip_suffix(".png").unwrap();
		let (name, version) = name.split_once('@').unwrap_or((name, ""));

		let version = match version {
			"" => Version::Standard,
			"2x" => Version::Size2X,
			"3x" => Version::Size3X,
			_ => return Err(()),
		};

		let image = match name {
			"background" => Self::Background(version),
			"footer" => Self::Footer(version),
			"icon" => Self::Icon(version),
			"logo" => Self::Logo(version),
			"strip" => Self::Strip(version),
			"thumbnail" => Self::Thumbnail(version),
			_ => return Err(()),
		};

		Ok(image)
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Version {
	Standard,
	Size2X,
	Size3X,
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Standard => write!(f, ""),
			Self::Size2X => write!(f, "@2x"),
			Self::Size3X => write!(f, "@3x"),
		}
	}
}

// From String
impl FromStr for Version {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"" => Ok(Self::Standard),
			"@2x" => Ok(Self::Size2X),
			"@3x" => Ok(Self::Size3X),
			_ => Err(()),
		}
	}
}

// hides content in debug print
pub struct AssetContent(Vec<u8>);

impl AssetContent {
	#[must_use]
	pub fn new(data: Vec<u8>) -> Self {
		Self(data)
	}
}

impl Deref for AssetContent {
	type Target = Vec<u8>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl fmt::Debug for AssetContent {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("AssetContent").finish()
	}
}

pub type AssetTable = HashMap<AssetType, AssetContent>;
