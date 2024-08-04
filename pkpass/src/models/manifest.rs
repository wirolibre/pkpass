use openssl::sha::Sha1;
use serde::{Deserialize, Serialize};
use std::{
	collections::{hash_map::Entry, HashMap},
	io,
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Version {
	Standard,
	Size2X,
	Size3X,
}

impl FromStr for Version {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"" => Ok(Self::Standard),
			"2x" => Ok(Self::Size2X),
			"3x" => Ok(Self::Size3X),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Default)]
pub struct Assets {
	pub images: ImageAssets,
	localized: HashMap<LanguageIdentifier, LocalizedAssets>,
}

impl Assets {
	pub fn get(&mut self, lang: LanguageIdentifier) -> &mut LocalizedAssets {
		self.localized.entry(lang).or_default()
	}

	pub(crate) fn get_mut(&mut self, path: &str) -> io::Result<&mut Vec<u8>> {
		// path can be escaped?
		match path.split_once('/') {
			Some((lang, localized_path)) => {
				let lang = lang.strip_suffix(".lproj").ok_or_else(|| {
					io::Error::new(
						io::ErrorKind::InvalidData,
						"path with slash is not a localized directory",
					)
				})?;
				let lang = LanguageIdentifier::from_str(lang).map_err(|_| {
					io::Error::new(io::ErrorKind::InvalidData, "could not parse lang")
				})?;

				Ok(self
					.localized
					.entry(lang)
					.or_default()
					.get_mut(localized_path)?)
			}
			None => self.images.get_mut(path),
		}
	}

	pub(crate) fn paths(&self) -> Vec<(String, &Vec<u8>)> {
		let mut v = self.images.paths();
		for (lang, lasst) in &self.localized {
			for (path, ct) in lasst.paths() {
				v.push((format!("{lang}.lproj/{path}"), ct));
			}
		}
		v
	}
}

#[derive(Debug, Default)]
pub struct ImageAssets {
	/// The icon (icon.png)
	pub icon: ImageAsset,
	/// The background image (background.png)
	pub background: ImageAsset,
	/// The footer image (footer.png)
	pub footer: ImageAsset,
	/// The logo image (logo.png)
	pub logo: ImageAsset,
	/// The strip image (strip.png)
	pub strip: ImageAsset,
	/// The thumbnail image (thumbnail.png)
	pub thumbnail: ImageAsset,
}

impl ImageAssets {
	pub(crate) fn get_mut(&mut self, path: &str) -> io::Result<&mut Vec<u8>> {
		let name = path.strip_suffix(".png").ok_or_else(|| {
			io::Error::new(
				io::ErrorKind::InvalidData,
				"images are expected in png format",
			)
		})?;
		let (name, version) = name.split_once('@').unwrap_or((name, ""));

		let version = Version::from_str(version).map_err(|()| {
			io::Error::new(
				io::ErrorKind::InvalidData,
				"image version is not recognized",
			)
		})?;

		let asset = match name {
			"background" => &mut self.background,
			"footer" => &mut self.footer,
			"icon" => &mut self.icon,
			"logo" => &mut self.logo,
			"strip" => &mut self.strip,
			"thumbnail" => &mut self.thumbnail,
			_ => return Err(io::Error::new(io::ErrorKind::InvalidData, "")),
		};

		Ok(asset.get_mut(&version))
	}

	pub(crate) fn paths(&self) -> Vec<(String, &Vec<u8>)> {
		let mut paths = vec![];
		for (name, image) in [
			("icon", &self.icon),
			("background", &self.background),
			("footer", &self.footer),
			("logo", &self.logo),
			("strip", &self.strip),
			("thumbnail", &self.thumbnail),
		] {
			for (v, ct) in image.versions() {
				paths.push((format!("{name}{v}.png"), ct));
			}
		}
		paths
	}
}

#[derive(Debug, Default)]
pub struct LocalizedAssets {
	pub images: ImageAssets,
	pub strings: Option<Vec<u8>>,
}

impl LocalizedAssets {
	pub(crate) fn get_mut(&mut self, localized_path: &str) -> io::Result<&mut Vec<u8>> {
		match localized_path {
			"pass.strings" => Ok(self.strings.get_or_insert_with(Default::default)),
			_ => self.images.get_mut(localized_path),
		}
	}

	pub(crate) fn paths(&self) -> Vec<(String, &Vec<u8>)> {
		let mut v = self.images.paths();
		if let Some(strings) = &self.strings {
			v.push(("pass.strings".into(), strings));
		}
		v
	}
}

#[derive(Debug, Default)]
pub struct ImageAsset {
	pub size_x1: Option<Vec<u8>>,
	pub size_x2: Option<Vec<u8>>,
	pub size_x3: Option<Vec<u8>>,
}

impl ImageAsset {
	pub(crate) fn get_mut(&mut self, version: &Version) -> &mut Vec<u8> {
		match version {
			Version::Standard => self.size_x1.get_or_insert_with(Default::default),
			Version::Size2X => self.size_x2.get_or_insert_with(Default::default),
			Version::Size3X => self.size_x3.get_or_insert_with(Default::default),
		}
	}

	pub(crate) fn versions(&self) -> Vec<(&str, &Vec<u8>)> {
		let mut v = vec![];
		if let Some(s1) = &self.size_x1 {
			v.push(("", s1));
		}
		if let Some(s2) = &self.size_x2 {
			v.push(("@2x", s2));
		}
		if let Some(s3) = &self.size_x3 {
			v.push(("@3x", s3));
		}
		v
	}
}
