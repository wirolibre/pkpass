use crate::{
	models::{Field, Fields, Metadata, PassKind, RowBehaviour},
	PassConfig,
};

impl Metadata {
	pub(crate) fn new(
		PassConfig {
			organization_name,
			description,
			serial_number,
			kind,
		}: PassConfig,
	) -> Self {
		let mut this = Self {
			format_version: 1,

			// TODO: not very clean, is filled later during signing
			pass_type_identifier: String::new(),
			team_identifier: String::new(),

			organization_name,
			description,
			serial_number,

			boarding_pass: None,
			coupon: None,
			event_ticket: None,
			generic: None,
			store_card: None,

			foreground_color: None,
			label_color: None,
			background_color: None,

			app_launch_url: None,
			associated_store_identifiers: Vec::default(),

			barcodes: Vec::default(),
			beacons: Vec::default(),
			expiration_date: None,
			grouping_identifier: None,
			locations: Vec::default(),
			logo_text: None,
			max_distance: None,
			nfc: None,
			relevant_date: None,
			semantics: None,
			sharing_prohibited: None,
			suppress_strip_shine: None,
			user_info: None,
			voided: None,

			web_service_url: None,
			authentication_token: None,
		};

		match kind {
			PassKind::BoardingPass(fields) => this.boarding_pass = Some(fields),
			PassKind::Coupon(fields) => this.coupon = Some(fields),
			PassKind::EventTicket(fields) => this.event_ticket = Some(fields),
			PassKind::Generic(fields) => this.generic = Some(fields),
			PassKind::StoreCard(fields) => this.store_card = Some(fields),
		};

		this
	}
}

impl Field {
	// TODO: do we really want `impl` here
	// impl forces non const api, if we introduce a generic `Metadata` const contexts might be useful nice
	pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
		Self {
			key: key.into(),
			value: value.into(),
			row: None,
			attributed_value: None,
			change_message: None,
			currency_code: None,
			data_detector_types: None,
			date_style: None,
			time_style: None,
			ignores_time_zone: None,
			is_relative: None,
			label: None,
			number_style: None,
			text_alignment: None,
			semantics: None,
		}
	}

	#[must_use]
	pub const fn row(mut self, behaviour: RowBehaviour) -> Self {
		self.row = Some(behaviour);
		self
	}

	// TODO: more fields

	#[must_use]
	pub fn label(mut self, label: impl Into<String>) -> Self {
		self.label = Some(label.into());
		self
	}

	// TODO: more fields
}

impl Fields {
	// TODO
	// pub fn add_secondary(mut self)
}
