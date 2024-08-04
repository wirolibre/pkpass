use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// <https://developer.apple.com/documentation/walletpasses/pass/semantictags>
#[derive(Debug, Default, Serialize, Deserialize)]
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
	pub current_arrival_date: Option<DateTime<Utc>>,

	/// The updated date and time of boarding, if different from the originally scheduled date and time.
	///
	/// Use this key for any type of boarding pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_boarding_date: Option<DateTime<Utc>>,

	/// The updated departure date and time, if different from the originally scheduled date and time.
	///
	/// Use this key for any type of boarding pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_departure_date: Option<DateTime<Utc>>,

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
	pub event_end_date: Option<DateTime<Utc>>,

	/// The full name of the event, such as the title of a movie.
	///
	/// Use this key for any type of event ticket.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_name: Option<String>,

	/// The date and time the event starts.
	///
	/// Use this key for any type of event ticket.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_start_date: Option<DateTime<Utc>>,

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
	pub original_arrival_date: Option<DateTime<Utc>>,

	/// The originally scheduled date and time of boarding. Use this key for any type of boarding pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub original_boarding_date: Option<DateTime<Utc>>,

	/// The originally scheduled date and time of departure. Use this key for any type of boarding pass.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub original_departure_date: Option<DateTime<Utc>>,

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

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagLocation {
	/// (Required) The latitude, in degrees.
	pub latitude: f64,

	/// (Required) The longitude, in degrees.
	pub longitude: f64,
}

/// Represents the parts of a person’s name.
#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagWifiNetwork {
	/// (Required) The password for the WiFi network.
	pub password: f64,

	/// (Required) The name for the WiFi network.
	pub ssid: f64,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Default, Serialize, Deserialize)]
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
