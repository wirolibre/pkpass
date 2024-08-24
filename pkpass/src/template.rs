use crate::{
	models::{Assets, Metadata},
	Pass,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
	pub variables: Vec<Variable>,
	pub meta: Metadata,
	// TODO: later
	// assets: ImageAssets,
}

impl Template {
	#[must_use]
	pub fn render(self) -> Pass {
		Pass::from_raw_parts(self.meta, Assets::default())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Variable {
	String(String),
	Number(u64),
	Date(String),
}

#[cfg(test)]
mod tests {
	use std::{fs::File, io::read_to_string};

	use super::*;

	#[test]
	fn render_template() -> Result<(), Box<dyn std::error::Error>> {
		let tpl = read_to_string(File::open("template.ron")?)?;
		let tpl = ron::from_str::<Template>(&tpl)?;

		dbg!(tpl);

		Ok(())
	}
}
