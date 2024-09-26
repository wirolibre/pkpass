use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
	fmt::{self, Debug},
	str::FromStr,
};
use url::Url;
use yansi::Painted;

// TODO: make a prelude

mod fields;
mod impls;
mod manifest;
mod semantics;
pub use fields::*;
pub use manifest::*;
pub use semantics::*;

/// Represent the `pass.json` file content
///
/// <https://developer.apple.com/documentation/walletpasses/pass>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Metadata {
	/// The version of the file format. The value must be 1.
	pub(crate) format_version: u64,

	/// The pass type identifier that’s registered with Apple. The value must be the same as the distribution certificate used to sign the pass.
	pub(crate) pass_type_identifier: String,
	/// The Team ID for the Apple Developer Program account that registered the pass type identifier.
	pub(crate) team_identifier: String,

	/// The name of the organization.
	// TODO: localizable string
	pub organization_name: String,
	/// A short description that iOS accessibility technologies use for a pass.
	// TODO: localizable string
	pub(crate) description: String,
	/// An alphanumeric serial number. The combination of the serial number and pass type identifier must be unique for each pass.
	pub(crate) serial_number: String,

	// TODO: rename all to `color_<part>`?
	/// A foreground color for the pass, specified as a CSS-style RGB triple, such as rgb(100, 10, 110).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub foreground_color: Option<RgbColor>,

	/// A color for the label text of the pass, specified as a CSS-style RGB
	/// triple, such as rgb(100, 10, 110). If you don’t provide a value, the
	/// system determines the label color.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label_color: Option<RgbColor>,

	/// A background color for the pass, specified as a CSS-style RGB triple, such as `rgb(23, 187, 82)`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background_color: Option<RgbColor>,

	// TODO: ugly but does not use flatten, which is required for ron should only be set once, and only one for the living of a pass
	// also quite heavy, take 5 times the size required
	#[serde(skip_serializing_if = "Option::is_none")]
	boarding_pass: Option<Fields>,
	#[serde(skip_serializing_if = "Option::is_none")]
	coupon: Option<Fields>,
	#[serde(skip_serializing_if = "Option::is_none")]
	event_ticket: Option<Fields>,
	#[serde(skip_serializing_if = "Option::is_none")]
	generic: Option<Fields>,
	#[serde(skip_serializing_if = "Option::is_none")]
	store_card: Option<Fields>,

	/// A URL the system passes to the associated app from [`associated_store_identifiers`] during launch.
	///
	/// This key isn’t supported for watchOS.
	#[serde(rename = "appLaunchURL")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub app_launch_url: Option<Url>,

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
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub associated_store_identifiers: Vec<u64>,

	/// An array of objects that represent possible barcodes on a pass. The
	/// system uses the first displayable barcode for the device.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub barcodes: Vec<Barcode>,

	/// An array of objects that represents the identity of Bluetooth Low Energy
	/// beacons the system uses to show a relevant pass.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub beacons: Vec<Beacon>,

	/// The date and time the pass expires. The value must be a complete date
	/// that includes hours and minutes, and may optionally include seconds.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_date: Option<DateTime<Utc>>,

	/// An identifier the system uses to group related boarding passes or event
	/// tickets. Wallet displays passes with the same groupingIdentifier, passTypeIdentifier, and type as a group.
	///
	/// Use this identifier to group passes that are tightly related, such as
	/// boarding passes for different connections on the same trip.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub grouping_identifier: Option<String>,

	/// An array of up to 10 objects that represent geographic locations the system uses to show a relevant pass.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub locations: Vec<Location>,

	/// The text to display next to the logo on the pass.
	// TODO: localizable string
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logo_text: Option<String>,

	/// The maximum distance, in meters, from a location in the locations array
	/// at which the pass is relevant. The system uses the smaller of either
	/// this distance or the default distance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_distance: Option<u64>,

	/// An object that contains the information to use for Value Added Service Protocol transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nfc: Option<Nfc>,

	/// The date and time when the pass becomes relevant as a W3C timestamp, such as the start time of a movie. The value must be a complete date that includes hours and minutes, and may optionally include seconds.
	///
	/// For information about the W3C timestamp format, see Time and Date Formats on the W3C website.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub relevant_date: Option<DateTime<Utc>>,

	/// An object that contains machine-readable metadata the system uses to offer a pass and suggest related actions. For example, setting Don’t Disturb mode for the duration of a movie.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub semantics: Option<SemanticTags>,

	/// A Boolean value introduced in iOS 11 that controls whether to show the Share button on the back of a pass. A value of true removes the button. The default value is false. This flag has no effect in earlier versions of iOS, nor does it prevent sharing the pass in some other way.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sharing_prohibited: Option<bool>,

	/// A Boolean value that controls whether to display the strip image without a shine effect. The default value is true.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suppress_strip_shine: Option<bool>,

	/// A JSON dictionary that contains any custom information for companion apps. The data doesn’t appear to the user.
	///
	/// For example, a pass for a cafe could include information about the customer’s favorite drink and sandwich in a machine-readable form. The companion app uses the data for placing an order for “the usual.”
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_info: Option<Value>,

	/// A Boolean value that indicates that the pass is void, such as a redeemed, one-time-use coupon. The default value is false.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voided: Option<bool>,

	// ------
	/// The URL for a web service that you use to update or personalize the pass. The URL can include an optional port number.
	#[serde(rename = "webServiceURL")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub web_service_url: Option<String>,

	/// The authentication token to use with the web service in the [`web_service_url`] key.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub authentication_token: Option<String>,
}

#[derive(Clone)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl RgbColor {
	#[must_use]
	pub const fn white() -> Self {
		Self(255, 255, 255)
	}

	#[must_use]
	pub const fn black() -> Self {
		Self(0, 0, 0)
	}
}

impl fmt::Debug for RgbColor {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let color = Painted::new("        ").bg(yansi::Color::Rgb(self.0, self.1, self.2));
		write!(
			f,
			"Color(r: {}, g: {}, b: {} {})",
			self.0, self.1, self.2, color
		)
	}
}

impl Serialize for RgbColor {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let color = format!("rgb({},{},{})", self.0, self.1, self.2);
		color.serialize(serializer)
	}
}

impl FromStr for RgbColor {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.get(0..=0) {
			Some("r") => {
				// TODO: proper string parsing
				let s = s
					.strip_prefix("rgb(")
					.and_then(|s| s.strip_suffix(')'))
					.map(|s| s.split(','))
					.ok_or("could not split on `,`")?;

				let mut vec = s.map(str::trim).map(str::parse);
				// TODO: error handling
				let red = vec
					.next()
					.ok_or("no red color")?
					.map_err(|_| "could not parse red color")?;
				let green = vec
					.next()
					.ok_or("no green color")?
					.map_err(|_| "could not parse green color")?;
				let blue = vec
					.next()
					.ok_or("no blue color")?
					.map_err(|_| "could not parse blue color")?;
				if vec.next().is_some() {
					return Err("rgb only has 3 colors");
				};

				Ok(Self(red, green, blue))
			}
			// TODO: remove, move to future fault tolerant pkpass parser
			// custom deser for color found in mcdonalds pkpass
			Some("#") => {
				let s = s.strip_prefix('#').unwrap();

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
				assert!(rest.is_empty());

				Ok(Self(red, green, blue))
			}
			_ => Err("format not recognized"),
		}
	}
}

impl<'de> Deserialize<'de> for RgbColor {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let s = String::deserialize(deserializer)?;
		s.parse()
			.map_err(|msg| serde::de::Error::custom(format!("could not parse color: {msg}")))
	}
}

/// <https://developer.apple.com/documentation/walletpasses/pass/barcodes>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Barcode {
	/// The format of the barcode.
	pub format: BarcodeFormat,

	/// The message or payload to display as a barcode.
	pub message: String,

	/// The IANA character set name of the text encoding to use to convert
	/// message from a string representation to a data representation that the
	/// system renders as a barcode, such as “iso-8859-1”.
	// TODO: is always UTF-8 if message is string
	pub message_encoding: String,

	/// The text to display near the barcode. For example, a human-readable
	/// version of the barcode data in case the barcode doesn’t scan.
	///
	/// The alternative text isn’t displayed for watchOS.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub alt_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BarcodeFormat {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Beacon {
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

/// <https://developer.apple.com/documentation/walletpasses/pass/locations>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Location {
	/// The latitude, in degrees, of the location.
	pub latitude: f64,

	/// The longitude, in degrees, of the location.
	pub longitude: f64,

	/// The altitude, in meters, of the location.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub altitude: Option<f64>,

	/// The text to display on the lock screen when the pass is relevant. For
	/// example, a description of a nearby location, such as “Store nearby on 1st and Main”.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub relevant_text: Option<String>,
}

/// <https://developer.apple.com/documentation/walletpasses/pass/nfc>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: not Strings
pub struct Nfc {
	/// The public encryption key the Value Added Services protocol uses. Use a
	/// Base64-encoded X.509 `SubjectPublicKeyInfo` structure that contains an
	/// ECDH public key for group P256.
	pub encryption_public_key: String,

	/// The payload the device transmits to the Apple Pay terminal. The size must be no more than 64 bytes. The system truncates messages longer than 64 bytes.
	pub message: String,

	/// A Boolean value that indicates whether the NFC pass requires
	/// authentication. The default value is false. A value of true requires
	/// the user to authenticate for each use of the NFC pass.
	///
	/// This key is valid in iOS 13.1 and later. Set sharingProhibited to true
	/// to prevent users from sharing passes with older iOS versions and
	/// bypassing the authentication requirement.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requires_authentication: Option<bool>,
}
