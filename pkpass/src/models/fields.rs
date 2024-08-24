use crate::models::SemanticTags;
use serde::{Deserialize, Serialize};

// TODO: insert design pictures to show layout diffs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum PassKind {
	/// <https://developer.apple.com/documentation/walletpasses/pass/boardingpass-data.dictionary>
	BoardingPass(Fields),
	/// <https://developer.apple.com/documentation/walletpasses/pass/coupon-data.dictionary>
	Coupon(Fields),
	/// <https://developer.apple.com/documentation/walletpasses/pass/eventticket-data.dictionary>
	EventTicket(Fields),
	/// <https://developer.apple.com/documentation/walletpasses/pass/generic-data.dictionary>
	Generic(Fields),
	/// <https://developer.apple.com/documentation/walletpasses/pass/storecard-data.dictionary>
	StoreCard(Fields),
}

/// <https://developer.apple.com/documentation/walletpasses/passfields>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fields {
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
	pub auxiliary: Vec<Field>,

	#[serde(rename = "backFields")]
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub back: Vec<Field>,

	/// The type of transit for a boarding pass. This key is invalid for other types of passes.
	// TODO: doc
	/// PANIC: Only valid for a boarding pass
	pub transit_type: Option<TransitType>,
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
#[derive(Serialize, Clone, Deserialize, Debug)]
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
#[derive(Serialize, Clone, Deserialize, Debug)]
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
#[derive(Serialize, Clone, Deserialize, Debug)]
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
#[derive(Serialize, Clone, Deserialize, Debug)]
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

	// TODO: document properly, should only be used in aux fields
	/// <https://developer.apple.com/documentation/walletpasses/passfields/auxiliaryfields>
	#[serde(skip_serializing_if = "Option::is_none")]
	pub row: Option<RowBehaviour>,

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
			_ => Err(serde::de::Error::custom("rowBehaviour doesn't match spec")),
		}
	}
}
