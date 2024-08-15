use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
	fmt::{self, Debug},
	str::FromStr,
};
use url::Url;
use yansi::Painted;

mod fields;
mod manifest;
mod semantics;
pub use fields::*;
pub use manifest::*;
pub use semantics::*;

// GUIDELINES: users can interact with `transparent::*`

#[pkpass_macros::spec]
mod transparent {
	use serde::{Deserialize, Serialize};

	/// Represent the `pass.json` file content
	///
	/// <https://developer.apple.com/documentation/walletpasses/pass>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(deny_unknown_fields)]
	#[spec(pub)]
	pub struct Metadata {
		/// The version of the file format. The value must be 1.
		#[spec(formatVersion)]
		pub(crate) format_version: u64,

		/// The pass type identifier that’s registered with Apple. The value must be the same as the distribution certificate used to sign the pass.
		#[spec(passTypeIdentifier)]
		pub(crate) pass_type_identifier: String,
		/// The Team ID for the Apple Developer Program account that registered the pass type identifier.
		#[spec(teamIdentifier)]
		pub(crate) team_identifier: String,

		/// The name of the organization.
		// TODO: localizable string
		#[spec(organizationName)]
		pub organization_name: String,
		/// A short description that iOS accessibility technologies use for a pass.
		// TODO: localizable string
		#[spec(description)]
		pub(crate) description: String,
		/// An alphanumeric serial number. The combination of the serial number and pass type identifier must be unique for each pass.
		#[spec(serialNumber)]
		pub(crate) serial_number: String,

		#[serde(flatten)]
		pub color_theme: ColorTheme,

		#[serde(flatten)]
		pub(crate) kind: PassKind,

		/// A URL the system passes to the associated app from [`associated_store_identifiers`] during launch.
		///
		/// This key isn’t supported for watchOS.
		#[serde(skip_serializing_if = "Option::is_none")]
		#[spec(appLaunchURL)]
		pub app_launch_url: Option<url::Url>,

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
		pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,

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
		pub relevant_date: Option<chrono::DateTime<chrono::Utc>>,

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
		pub user_info: Option<serde_json::Value>,

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

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct ColorTheme {
		/// A foreground color for the pass, specified as a CSS-style RGB triple, such as rgb(100, 10, 110).
		#[serde(rename = "foregroundColor", skip_serializing_if = "Option::is_none")]
		pub foreground: Option<RgbColor>,

		/// A color for the label text of the pass, specified as a CSS-style RGB
		/// triple, such as rgb(100, 10, 110). If you don’t provide a value, the
		/// system determines the label color.
		#[serde(rename = "labelColor", skip_serializing_if = "Option::is_none")]
		pub label: Option<RgbColor>,

		/// A background color for the pass, specified as a CSS-style RGB triple, such as `rgb(23, 187, 82)`.
		#[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
		pub background: Option<RgbColor>,
	}

	#[derive(Clone)]
	pub struct RgbColor {
		pub red: u8,
		pub green: u8,
		pub blue: u8,
	}

	impl RgbColor {
		#[must_use]
		pub const fn new(red: u8, green: u8, blue: u8) -> Self {
			Self { red, green, blue }
		}

		#[must_use]
		pub const fn white() -> Self {
			Self::new(255, 255, 255)
		}

		#[must_use]
		pub const fn black() -> Self {
			Self::new(0, 0, 0)
		}
	}

	impl Into<yansi::Color> for RgbColor {
		fn into(self) -> yansi::Color {
			yansi::Color::Rgb(self.red, self.green, self.blue)
		}
	}

	impl std::fmt::Debug for RgbColor {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let color = yansi::Painted::new("   ").bg(self.clone().into());
			write!(
				f,
				"Color({}, {}, {}; {})",
				self.red, self.green, self.blue, color
			)
		}
	}

	impl Serialize for RgbColor {
		fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let color = format!("rgb({},{},{})", self.red, self.green, self.blue);
			color.serialize(serializer)
		}
	}

	impl<'de> Deserialize<'de> for RgbColor {
		fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			let s = String::deserialize(deserializer)?;

			let parsed = {
				// TODO: proper string parsing
				let s = s
					.strip_prefix("rgb(")
					.and_then(|s| s.strip_suffix(')'))
					.map(|s| s.split(','))
					.unwrap();

				let mut vec = s.map(str::trim).map(str::parse);
				// TODO: error handling
				let red = vec.next().unwrap().unwrap();
				let green = vec.next().unwrap().unwrap();
				let blue = vec.next().unwrap().unwrap();
				assert!(vec.next().is_none());

				Self { red, green, blue }
			};

			Ok(parsed)
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

	// --- fields

	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub enum PassKind {
		BoardingPass(BoardingPass),
		Coupon(Coupon),
		EventTicket(EventTicket),
		Generic(Generic),
		StoreCard(StoreCard),
	}

	/// <https://developer.apple.com/documentation/walletpasses/passfields>
	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct PassFields {
		#[serde(rename = "headerFields")]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub header: Vec<Field>,

		#[serde(rename = "primaryFields")]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub primary: Vec<Field>,

		#[serde(rename = "secondaryFields")]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub secondary: Vec<Field>,

		#[serde(rename = "auxiliaryFields")]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub auxiliary: Vec<RowField>,

		#[serde(rename = "backFields")]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub back: Vec<Field>,
	}

	/// <https://developer.apple.com/documentation/walletpasses/pass/boardingpass>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct BoardingPass {
		#[serde(flatten)]
		pub fields: PassFields,

		/// The type of transit for a boarding pass. This key is invalid for other types of passes.
		pub transit_type: TransitType,
	}

	/// <https://developer.apple.com/documentation/walletpasses/pass/coupon>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct Coupon {
		#[serde(flatten)]
		pub fields: PassFields,
	}

	/// <https://developer.apple.com/documentation/walletpasses/pass/eventticket>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct EventTicket {
		#[serde(flatten)]
		pub fields: PassFields,
	}

	/// <https://developer.apple.com/documentation/walletpasses/pass/eventticket>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct Generic {
		#[serde(flatten)]
		pub fields: PassFields,
	}

	/// <https://developer.apple.com/documentation/walletpasses/pass/eventticket>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct StoreCard {
		#[serde(flatten)]
		pub fields: PassFields,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
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

	/// The data detectors to apply to the value of a field on the back of the pass.
	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum DetectorType {
		#[serde(rename = "PKDataDetectorTypePhoneNumber")]
		PhoneNumber,
		#[serde(rename = "PKDataDetectorTypeLink")]
		Link,
		#[serde(rename = "PKDataDetectorTypeAddress")]
		Address,
		#[serde(rename = "PKDataDetectorTypeCalendarEvent")]
		CalendarEvent,
	}

	/// The style of the date to display in the field.
	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum DateStyle {
		#[serde(rename = "PKDateStyleNone")]
		None,
		#[serde(rename = "PKDateStyleShort")]
		Short,
		#[serde(rename = "PKDateStyleMedium")]
		Medium,
		#[serde(rename = "PKDateStyleLong")]
		Long,
		#[serde(rename = "PKDateStyleFull")]
		Full,
	}

	/// The style of the number to display in the field.
	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum NumberStyle {
		#[serde(rename = "PKNumberStyleDecimal")]
		Decimal,
		#[serde(rename = "PKNumberStylePercent")]
		Percent,
		#[serde(rename = "PKNumberStyleScientific")]
		Scientific,
		#[serde(rename = "PKNumberStyleSpellOut")]
		SpellOut,
	}

	/// The alignment for the content of a field.
	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum TextAlignment {
		#[serde(rename = "PKTextAlignmentLeft")]
		Left,
		#[serde(rename = "PKTextAlignmentCenter")]
		Center,
		#[serde(rename = "PKTextAlignmentRight")]
		Right,
		#[serde(rename = "PKTextAlignmentNatural")]
		Natural,
	}

	/// <https://developer.apple.com/documentation/walletpasses/passfieldcontent>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct Field {
		pub key: String,

		pub value: String,

		#[serde(flatten)]
		pub options: FieldOptions,
	}

	/// <https://developer.apple.com/documentation/walletpasses/passfields/auxiliaryfields>
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct RowField {
		pub key: String,

		pub value: String,

		pub row: RowBehaviour,

		#[serde(flatten)]
		pub options: FieldOptions,
	}

	// TODO: check option
	#[derive(Debug, Clone)]
	pub enum RowBehaviour {
		KeepRow,
		NewRow,
	}

	impl Serialize for RowBehaviour {
		fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			match self {
				Self::KeepRow => 0u8.serialize(serializer),
				Self::NewRow => 1u8.serialize(serializer),
			}
		}
	}

	impl<'de> Deserialize<'de> for RowBehaviour {
		fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			let num = u8::deserialize(deserializer)?;
			match num {
				0 => Ok(Self::KeepRow),
				1 => Ok(Self::NewRow),
				_ => Err(todo!()),
			}
		}
	}

	#[derive(Debug, Clone, Default, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct FieldOptions {
		/// The value of the field, including HTML markup for links.
		///
		/// The only supported tag is the `<a>` tag and its href attribute.
		///
		/// The value of this key overrides that of the value key.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub attributed_value: Option<String>,

		/// A format string for the alert text to display when the pass is updated.
		///
		/// The format string must contain the escape %@, which is replaced with the field’s new value.
		/// For example, “Gate changed to %@”.
		///
		/// You must provide a value for the system to show a change notification.
		///
		/// This field isn’t used for watchOS.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub change_message: Option<String>,

		/// The currency code to use for the value of the field.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub currency_code: Option<String>,

		/// The data detectors to apply to the value of a field on the back of the pass.
		///
		/// The default is to apply all data detectors. To use no data detectors, specify an empty array.
		///
		/// You don’t use data detectors for fields on the front of the pass.
		///
		/// This field isn’t used for watchOS.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub data_detector_types: Option<DetectorType>,

		/// The style of the date to display in the field.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub date_style: Option<DateStyle>,

		/// A Boolean value that controls the time zone for the time and date to display in the field.
		///
		/// The default value is false, which displays the time and date using the current device’s time zone.
		/// Otherwise, the time and date appear in the time zone associated with the date and time of value.
		///
		/// This key doesn’t affect the pass relevance calculation.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub ignores_time_zone: Option<bool>,

		/// A Boolean value that controls whether the date appears as a relative date.\
		///
		/// The default value is false, which displays the date as an absolute date.
		///
		/// This key doesn’t affect the pass relevance calculation.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub is_relative: Option<bool>,

		/// The text for a field label.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub label: Option<String>,

		/// The style of the number to display in the field.
		///
		/// Formatter styles have the same meaning as the formats with corresponding names in NumberFormatter.Style.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub number_style: Option<NumberStyle>,

		/// The alignment for the content of a field. The default is natural alignment, which aligns the text based on its script direction.
		///
		/// This key is invalid for primary and back fields.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub text_alignment: Option<TextAlignment>,

		/// The style of the time displayed in the field.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub time_style: Option<DateStyle>,

		/// Semantic tags
		///
		/// Metadata the system uses to offer a pass and suggest related actions.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub semantics: Option<SemanticTags>,
	}

	// -- semantics

	/// <https://developer.apple.com/documentation/walletpasses/pass/semantictags>
	#[derive(Debug, Clone, Default, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase", deny_unknown_fields)]
	pub struct SemanticTags {
		/// The IATA airline code, such as “EX” for flightCode “EX123”. Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub airline_code: Option<String>,

		/// An array of the Apple Music persistent ID for each artist performing at the event, in decreasing order of significance.
		///
		/// Use this key for any type of event ticket.
		#[serde(rename = "artistIDs")]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub artist_ids: Vec<String>,

		/// The unique abbreviation of the away team’s name. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub away_team_abbreviation: Option<String>,

		/// The home location of the away team. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub away_team_location: Option<String>,

		/// The name of the away team. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub away_team_name: Option<String>,

		/// The current balance redeemable with the pass. Use this key only for a store card pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub balance: Option<SemanticTagCurrencyAmount>,

		/// A group number for boarding. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub boarding_group: Option<String>,

		/// A sequence number for boarding. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub boarding_sequence_number: Option<String>,

		/// The number of the passenger car.
		///
		/// A train car is also called a carriage, wagon, coach, or bogie in some countries.
		///
		/// Use this key only for a train or other rail boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub car_number: Option<String>,

		/// A booking or reservation confirmation number. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub confirmation_number: Option<String>,

		/// The updated date and time of arrival, if different from the originally scheduled date and time.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub current_arrival_date: Option<chrono::DateTime<chrono::Utc>>,

		/// The updated date and time of boarding, if different from the originally scheduled date and time.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub current_boarding_date: Option<chrono::DateTime<chrono::Utc>>,

		/// The updated departure date and time, if different from the originally scheduled date and time.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub current_departure_date: Option<chrono::DateTime<chrono::Utc>>,

		/// The IATA airport code for the departure airport, such as “MPM” or “LHR”.
		///
		/// Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_airport_code: Option<String>,

		/// The full name of the departure airport, such as “Maputo International Airport”.
		///
		/// Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_airport_name: Option<String>,

		/// The gate number or letters of the departure gate, such as “1A”.
		///
		/// Do not include the word “Gate.”
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_gate: Option<String>,

		/// An object that represents the geographic coordinates of the transit departure location, suitable for display on a map.
		///
		/// If possible, use precise locations, which are more useful to travelers; for example, the specific location of an airport gate.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_location: Option<SemanticTagLocation>,

		/// A brief description of the departure location.
		///
		/// For example, for a flight departing from an airport whose code is “LHR,” an appropriate description might be “London, Heathrow“.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_location_description: Option<String>,

		/// The name of the departure platform, such as “A”.
		///
		/// Don’t include the word “Platform.” Use this key only for a train or other rail boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_platform: Option<String>,

		/// The name of the departure station, such as “1st Street Station”.
		///
		/// Use this key only for a train or other rail boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_station_name: Option<String>,

		/// The name or letter of the departure terminal, such as “A”.
		///
		/// Don’t include the word “Terminal.” Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub departure_terminal: Option<String>,

		/// The IATA airport code for the destination airport, such as “MPM” or “LHR”.
		///
		/// Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_airport_code: Option<String>,

		/// The full name of the destination airport, such as “London Heathrow”.
		///
		/// Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_airport_name: Option<String>,

		/// The gate number or letter of the destination gate, such as “1A”.
		///
		/// Don’t include the word “Gate”.
		///
		/// Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_gate: Option<String>,

		/// An object that represents the geographic coordinates of the transit departure location, suitable for display on a map.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_location: Option<SemanticTagLocation>,

		/// A brief description of the destination location.
		///
		/// For example, for a flight arriving at an airport whose code is “MPM,” “Maputo“ might be an appropriate description.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_location_description: Option<String>,

		/// The name of the destination platform, such as “A”.
		///
		/// Don’t include the word “Platform”.
		///
		/// Use this key only for a train or other rail boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_platform: Option<String>,

		/// The name of the destination station, such as “1st Street Station”.
		///
		/// Use this key only for a train or other rail boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_station_name: Option<String>,

		/// The terminal name or letter of the destination terminal, such as “A”.
		///
		/// Don’t include the word “Terminal”.
		///
		/// Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub destination_terminal: Option<String>,

		/// The duration of the event or transit journey, in seconds.
		///
		/// Use this key for any type of boarding pass and any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub duration: Option<u32>,

		/// The date and time the event ends. Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub event_end_date: Option<chrono::DateTime<chrono::Utc>>,

		/// The full name of the event, such as the title of a movie.
		///
		/// Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub event_name: Option<String>,

		/// The date and time the event starts.
		///
		/// Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub event_start_date: Option<chrono::DateTime<chrono::Utc>>,

		/// The type of event. Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub event_type: Option<SemanticEventType>,

		/// The IATA flight code, such as “EX123”. Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub flight_code: Option<String>,

		/// The numeric portion of the IATA flight code, such as 123 for flightCode “EX123”.
		///
		/// Use this key only for airline boarding passes.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub flight_number: Option<u32>,

		/// The genre of the performance, such as “Classical”. Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub genre: Option<String>,

		/// The unique abbreviation of the home team’s name. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub home_team_abbreviation: Option<String>,

		/// The home location of the home team. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub home_team_location: Option<String>,

		/// The name of the home team. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub home_team_name: Option<String>,

		/// The abbreviated league name for a sports event. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub league_abbreviation: Option<String>,

		/// The unabbreviated league name for a sports event. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub league_name: Option<String>,

		/// The name of a frequent flyer or loyalty program. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub membership_program_name: Option<String>,

		/// The ticketed passenger’s frequent flyer or loyalty number. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub membership_program_number: Option<String>,

		/// The originally scheduled date and time of arrival. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub original_arrival_date: Option<chrono::DateTime<chrono::Utc>>,

		/// The originally scheduled date and time of boarding. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub original_boarding_date: Option<chrono::DateTime<chrono::Utc>>,

		/// The originally scheduled date and time of departure. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub original_departure_date: Option<chrono::DateTime<chrono::Utc>>,

		/// An object that represents the name of the passenger. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub passenger_name: Option<SemanticTagPersonNameComponents>,

		/// An array of the full names of the performers and opening acts at the event, in decreasing order of significance.
		///
		/// Use this key for any type of event ticket.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub performer_names: Vec<String>,

		/// The priority status the ticketed passenger holds, such as “Gold” or “Silver”.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub priority_status: Option<String>,

		/// An array of objects that represent the details for each seat at an event or on a transit journey.
		///
		/// Use this key for any type of boarding pass or event ticket.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub seats: Vec<SemanticTagSeat>,

		/// The type of security screening for the ticketed passenger, such as “Priority”.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub security_screening: Option<String>,

		/// Determines whether the user’s device remains silent during an event or transit journey.
		///
		/// The system may override the key and determine the length of the period of silence.
		///
		/// Use this key for any type of boarding pass or event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub silence_requested: Option<bool>,

		/// The commonly used name of the sport. Use this key only for a sports event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub sport_name: Option<String>,

		/// The total price for the pass. Use this key for any pass type.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub total_price: Option<SemanticTagCurrencyAmount>,

		/// The name of the transit company. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub transit_provider: Option<String>,

		/// A brief description of the current boarding status for the vessel, such as “On Time” or “Delayed”.
		///
		/// For delayed status, provide [`current_boarding_date`], [`current_departure_date`], and [`current_arrival_date`].
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub transit_status: Option<String>,

		/// A brief description that explains the reason for the current transitStatus, such as “Thunderstorms”.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub transit_status_reason: Option<String>,

		/// The name of the vehicle to board, such as the name of a boat. Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub vehicle_name: Option<String>,

		/// The identifier of the vehicle to board, such as the aircraft registration number or train number.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub vehicle_number: Option<String>,

		/// A brief description of the type of vehicle to board, such as the model and manufacturer of a plane or the class of a boat.
		///
		/// Use this key for any type of boarding pass.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub vehicle_type: Option<String>,

		/// The full name of the entrance, such as “Gate A”, to use to gain access to the ticketed event.
		///
		/// Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub venue_entrance: Option<String>,

		/// An object that represents the geographic coordinates of the venue.
		///
		/// Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub venue_location: Option<SemanticTagLocation>,

		/// The full name of the venue.
		///
		/// Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub venue_name: Option<String>,

		/// The phone number for enquiries about the venue’s ticketed event.
		///
		/// Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub venue_phone_number: Option<String>,

		/// The full name of the room where the ticketed event is to take place.
		///
		/// Use this key for any type of event ticket.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub venue_room: Option<String>,

		/// An array of objects that represent the WiFi networks associated with the event; for example, the network name and password associated with a developer conference.
		///
		/// Use this key for any type of pass.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub wifi_access: Vec<SemanticTagWifiNetwork>,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct SemanticTagCurrencyAmount {
		/// The amount of money.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub amount: Option<String>,

		/// The currency code for amount.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub currency_code: Option<String>,
	}

	/// Represents the coordinates of a location.
	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct SemanticTagLocation {
		/// (Required) The latitude, in degrees.
		pub latitude: f64,

		/// (Required) The longitude, in degrees.
		pub longitude: f64,
	}

	/// Represents the parts of a person’s name.
	#[derive(Debug, Clone, Default, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct SemanticTagPersonNameComponents {
		/// The person’s family name or last name.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub family_name: Option<String>,

		/// The person’s given name; also called the forename or first name in some countries.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub given_name: Option<String>,

		/// The person’s middle name.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub middle_name: Option<String>,

		/// The prefix for the person’s name, such as “Dr”.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub name_prefix: Option<String>,

		/// The suffix for the person’s name, such as “Junior”.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub name_suffix: Option<String>,

		/// The person’s nickname.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub nickname: Option<String>,

		/// The phonetic representation of the person’s name.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub phonetic_representation: Option<String>,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct SemanticTagWifiNetwork {
		/// (Required) The password for the WiFi network.
		pub password: f64,

		/// (Required) The name for the WiFi network.
		pub ssid: f64,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum SemanticEventType {
		#[serde(rename = "PKEventTypeGeneric")]
		Generic,
		#[serde(rename = "PKEventTypeLivePerformance")]
		LivePerformance,
		#[serde(rename = "PKEventTypeMovie")]
		Movie,
		#[serde(rename = "PKEventTypeSports")]
		Sports,
		#[serde(rename = "PKEventTypeConference")]
		Conference,
		#[serde(rename = "PKEventTypeConvention")]
		Convention,
		#[serde(rename = "PKEventTypeWorkshop")]
		Workshop,
		#[serde(rename = "PKEventTypeSocialGathering")]
		SocialGathering,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct SemanticTagSeat {
		/// A description of the seat, such as “A flat bed seat”.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub seat_description: Option<String>,

		/// The identifier code for the seat.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub seat_identifier: Option<String>,

		/// The number of the seat.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub seat_number: Option<String>,

		/// The row that contains the seat.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub seat_row: Option<String>,

		/// The section that contains the seat.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub seat_section: Option<String>,

		/// The type of seat, such as “Reserved seating”.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub seat_type: Option<String>,
	}
}

/// <https://developer.apple.com/documentation/walletpasses/pass/barcodes>
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
// TODO: not Strings
pub struct Nfc {
	/// The public encryption key the Value Added Services protocol uses. Use a
	/// Base64-encoded X.509 `SubjectPublicKeyInfo` structure that contains an
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
