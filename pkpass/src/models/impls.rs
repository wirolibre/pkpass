use crate::models::{Field, Fields, RowBehaviour};

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
