use openssl::sha::Sha1;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
	collections::{hash_map::Entry, HashMap},
	fmt::{self, Debug},
	ops::Deref,
	str::FromStr,
};
use unic_langid::LanguageIdentifier;
use url::Url;

/// Represent the `pass.json` file content
///
/// <https://developer.apple.com/documentation/walletpasses/pass>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Pass {
	/// A URL the system passes to the associated app from [`associated_store_identifiers`] during launch.
	///
	/// This key isn’t supported for watchOS.
	#[serde(rename = "appLaunchURL")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) app_launch_url: Option<Url>,

	/// An array of App Store identifiers for apps associated with the pass. The
	/// associated app on a device is the first item in the array that’s compatible
	/// with that device.
	///
	/// A link to launch the app is on the back of the pass. If the app isn’t
	/// installed, the link opens the App Store.
	///
	/// This key works only for payment passes.
	///
	/// This key isn’t supported for watchOS.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) associated_store_identifiers: Option<Vec<u64>>,

	/// The authentication token to use with the web service in the [`web_service_url`] key.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) authentication_token: Option<String>,

	/// A background color for the pass, specified as a CSS-style RGB triple, such as `rgb(23, 187, 82)`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) background_color: Option<RgbColor>,

	/// An object that represents a barcode on a pass.
	#[deprecated(
		since = "0.0.0",
		note = "Apple deprecated the barcode item, please use `barcodes` instead"
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) barcode: Option<Barcode>,

	/// An array of objects that represent possible barcodes on a pass. The
	/// system uses the first displayable barcode for the device.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) barcodes: Option<Vec<Barcodes>>,

	/// An array of objects that represents the identity of Bluetooth Low Energy
	/// beacons the system uses to show a relevant pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) beacons: Option<Vec<Beacons>>,

	/// An object that contains the information for a boarding pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) boarding_pass: Option<BoardingPass>,

	/// An object that contains the information for a coupon.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) coupon: Option<Coupon>,

	/// A short description that iOS accessibility technologies use for a pass.
	// TODO: localizable string
	pub(crate) description: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_ticket: Option<EventTicket>,

	/// The date and time the pass expires. The value must be a complete date
	/// that includes hours and minutes, and may optionally include seconds.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) expiration_date: Option<String>,

	/// A foreground color for the pass, specified as a CSS-style RGB triple, such as rgb(100, 10, 110).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) foreground_color: Option<RgbColor>,

	/// The version of the file format. The value must be 1.
	pub(crate) format_version: u64,

	/// An object that contains the information for a generic pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) generic: Option<Generic>,

	/// An identifier the system uses to group related boarding passes or event
	/// tickets. Wallet displays passes with the same groupingIdentifier, passTypeIdentifier, and type as a group.
	///
	/// Use this identifier to group passes that are tightly related, such as
	/// boarding passes for different connections on the same trip.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub grouping_identifier: Option<String>,

	/// A color for the label text of the pass, specified as a CSS-style RGB
	/// triple, such as rgb(100, 10, 110). If you don’t provide a value, the
	/// system determines the label color.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) label_color: Option<RgbColor>,

	/// An array of up to 10 objects that represent geographic locations the system uses to show a relevant pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) locations: Option<Vec<Locations>>,

	/// The text to display next to the logo on the pass.
	// TODO: localizable string
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logo_text: Option<String>,

	/// The maximum distance, in meters, from a location in the locations array
	/// at which the pass is relevant. The system uses the smaller of either
	/// this distance or the default distance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) max_distance: Option<u64>,

	/// An object that contains the information to use for Value Added Service Protocol transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) nfc: Option<Nfc>,

	/// The name of the organization.
	// TODO: localizable string
	pub(crate) organization_name: String,

	/// The pass type identifier that’s registered with Apple. The value must be the same as the distribution certificate used to sign the pass.
	pub(crate) pass_type_identifier: String,

	/// The date and time when the pass becomes relevant as a W3C timestamp, such as the start time of a movie. The value must be a complete date that includes hours and minutes, and may optionally include seconds.
	///
	/// For information about the W3C timestamp format, see Time and Date Formats on the W3C website.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) relevant_date: Option<String>,

	/// An object that contains machine-readable metadata the system uses to offer a pass and suggest related actions. For example, setting Don’t Disturb mode for the duration of a movie.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) semantics: Option<SemanticTags>,

	/// An alphanumeric serial number. The combination of the serial number and pass type identifier must be unique for each pass.
	pub(crate) serial_number: String,

	/// A Boolean value introduced in iOS 11 that controls whether to show the Share button on the back of a pass. A value of true removes the button. The default value is false. This flag has no effect in earlier versions of iOS, nor does it prevent sharing the pass in some other way.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) sharing_prohibited: Option<bool>,

	/// An object that contains the information for a store card.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) store_card: Option<StoreCard>,

	/// A Boolean value that controls whether to display the strip image without a shine effect. The default value is true.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) suppress_strip_shine: Option<bool>,

	/// The Team ID for the Apple Developer Program account that registered the pass type identifier.
	pub(crate) team_identifier: String,

	/// A JSON dictionary that contains any custom information for companion apps. The data doesn’t appear to the user.
	///
	/// For example, a pass for a cafe could include information about the customer’s favorite drink and sandwich in a machine-readable form. The companion app uses the data for placing an order for “the usual.”
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) user_info: Option<Value>,

	/// A Boolean value that indicates that the pass is void, such as a redeemed, one-time-use coupon. The default value is false.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) voided: Option<bool>,

	/// The URL for a web service that you use to update or personalize the pass. The URL can include an optional port number.
	#[serde(rename = "webServiceURL")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) web_service_url: Option<String>,
	// ---
}

pub(crate) struct RgbColor(u8, u8, u8);

impl fmt::Debug for RgbColor {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Color(r: {}, g: {}, b: {})", self.0, self.1, self.2)
	}
}

impl Serialize for RgbColor {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let color = format!("rgb({},{},{})", self.0, self.1, self.2);
		color.serialize(serializer)
	}
}

impl FromStr for RgbColor {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.get(0..=0) {
			Some("r") => {
				// TODO: proper string parsing
				let s = s
					.strip_prefix("rgb(")
					.map(|s| s.strip_suffix(')'))
					.flatten()
					.map(|s| s.split(','))
					.unwrap();

				let mut vec = s.map(str::trim).map(str::parse);
				// TODO: error handling
				let red = vec.next().unwrap().unwrap();
				let green = vec.next().unwrap().unwrap();
				let blue = vec.next().unwrap().unwrap();
				assert!(vec.next().is_none());

				Ok(Self(red, green, blue))
			}
			Some("#") => {
				let s = s.strip_prefix("#").unwrap();

				let red = s
					.get(0..=1)
					.map(|by| hex::decode(by).unwrap()[0])
					.map(u8::try_from)
					.unwrap()
					.unwrap();
				let green = s
					.get(2..=3)
					.map(|by| hex::decode(by).unwrap()[0])
					.map(u8::try_from)
					.unwrap()
					.unwrap();
				let blue = s
					.get(4..=5)
					.map(|by| hex::decode(by).unwrap()[0])
					.map(u8::try_from)
					.unwrap()
					.unwrap();
				let rest = s.get(6..).unwrap();

				Ok(Self(red, green, blue))
			}
			_ => Err(()),
		}
	}
}

impl<'de> Deserialize<'de> for RgbColor {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let s = String::deserialize(deserializer)?;
		Ok(s.parse().unwrap())
	}
}

// TODO: add a feature flag for this?
/// <https://developer.apple.com/documentation/walletpasses/pass/barcode>
#[deprecated(
	since = "0.0.0",
	note = "Apple deprecated this Barcode attr item, please use `Barcodes` instead"
)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Barcode {
	/// The text to display near the barcode. For example, a human-readable
	/// version of the barcode data in case the barcode doesn’t scan.
	///
	/// The alternative text isn’t displayed for watchOS.
	#[serde(skip_serializing_if = "Option::is_none")]
	alt_text: Option<String>,

	/// The format of the barcode.
	format: BarcodeFormat,

	/// The message or payload to display as a barcode.
	message: String,

	/// The IANA character set name of the text encoding to use to convert
	/// message from a string representation to a data representation that the
	/// system renders as a barcode, such as “iso-8859-1”.
	// TODO: should be: message_encoding: String,
	message_encoding: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BarcodeFormat {
	#[serde(rename = "PKBarcodeFormatQR")]
	Qr,
	#[serde(rename = "PKBarcodeFormatPDF417")]
	Pdf417,
	#[serde(rename = "PKBarcodeFormatAztec")]
	Aztec,
}

/// <https://developer.apple.com/documentation/walletpasses/pass/barcodes>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Barcodes {
	/// The text to display near the barcode. For example, a human-readable
	/// version of the barcode data in case the barcode doesn’t scan.
	///
	/// The alternative text isn’t displayed for watchOS.
	#[serde(skip_serializing_if = "Option::is_none")]
	alt_text: Option<String>,

	/// The format of the barcode.
	format: BarcodesFormat,

	/// The message or payload to display as a barcode.
	message: String,

	/// The IANA character set name of the text encoding to use to convert
	/// message from a string representation to a data representation that the
	/// system renders as a barcode, such as “iso-8859-1”.
	// TODO: should be: message_encoding: String,
	message_encoding: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BarcodesFormat {
	#[serde(rename = "PKBarcodeFormatQR")]
	Qr,
	#[serde(rename = "PKBarcodeFormatPDF417")]
	Pdf417,
	#[serde(rename = "PKBarcodeFormatAztec")]
	Aztec,
	/// Pdf128 is not supported on watchOS.
	#[serde(rename = "PKBarcodeFormatCode128")]
	Pdf128,
}

/// <https://developer.apple.com/documentation/walletpasses/pass/beacons>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Beacons {
	// The major identifier of a Bluetooth Low Energy location beacon.
	#[serde(skip_serializing_if = "Option::is_none")]
	major: Option<u16>,

	// The minor identifier of a Bluetooth Low Energy location beacon.
	#[serde(skip_serializing_if = "Option::is_none")]
	minor: Option<u16>,

	// The unique identifier of a Bluetooth Low Energy location beacon.
	#[serde(rename = "proximityUUID")]
	proximity_uuid: String,

	// The text to display on the lock screen when the pass is relevant. For
	/// example, a description of a nearby location, such as “Store nearby on
	/// 1st and Main”.
	#[serde(skip_serializing_if = "Option::is_none")]
	relevant_text: Option<String>,
}

/// <https://developer.apple.com/documentation/walletpasses/pass/boardingpass>
#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: inherit pass fields
pub struct BoardingPass {
	/// The type of transit for a boarding pass. This key is invalid for other types of passes.
	transit_type: TransitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransitType {
	#[serde(rename = "PKTransitTypeAir")]
	Air,
	#[serde(rename = "PKTransitTypeBoat")]
	Boat,
	#[serde(rename = "PKTransitTypeBus")]
	Bus,
	#[serde(rename = "PKTransitTypeGeneric")]
	Generic,
	#[serde(rename = "PKTransitTypeTrain")]
	Train,
}

/// <https://developer.apple.com/documentation/walletpasses/pass/coupon>
#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: inherit pass fields
pub struct Coupon {}

/// <https://developer.apple.com/documentation/walletpasses/pass/eventticket>
#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: inherit pass fields
pub struct EventTicket {}

/// <https://developer.apple.com/documentation/walletpasses/pass/eventticket>
#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: inherit pass fields
pub struct Generic {}

/// <https://developer.apple.com/documentation/walletpasses/pass/locations>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Locations {
	/// The altitude, in meters, of the location.
	#[serde(skip_serializing_if = "Option::is_none")]
	altitude: Option<f64>,

	/// The latitude, in degrees, of the location.
	latitude: f64,

	/// The longitude, in degrees, of the location.
	longitude: f64,

	/// The text to display on the lock screen when the pass is relevant. For
	/// example, a description of a nearby location, such as “Store nearby on 1st and Main”.
	#[serde(skip_serializing_if = "Option::is_none")]
	relevant_text: Option<String>,
}

/// <https://developer.apple.com/documentation/walletpasses/pass/nfc>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: not Strings
pub struct Nfc {
	/// The public encryption key the Value Added Services protocol uses. Use a
	/// Base64-encoded X.509 SubjectPublicKeyInfo structure that contains an
	/// ECDH public key for group P256.
	encryption_public_key: String,

	/// The payload the device transmits to the Apple Pay terminal. The size must be no more than 64 bytes. The system truncates messages longer than 64 bytes.
	message: String,

	/// A Boolean value that indicates whether the NFC pass requires
	/// authentication. The default value is false. A value of true requires
	/// the user to authenticate for each use of the NFC pass.
	///
	/// This key is valid in iOS 13.1 and later. Set sharingProhibited to true
	/// to prevent users from sharing passes with older iOS versions and
	/// bypassing the authentication requirement.
	#[serde(skip_serializing_if = "Option::is_none")]
	requires_authentication: Option<bool>,
}

/// <https://developer.apple.com/documentation/walletpasses/pass/semantictags>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SemanticTags {
	// TODO
}

/// <https://developer.apple.com/documentation/walletpasses/pass/eventticket>
#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: inherit pass fields
pub struct StoreCard {}

// TODO: remove -----------------------------

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
