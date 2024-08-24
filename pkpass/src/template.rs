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
	use super::*;

	#[test]
	fn render_template() -> Result<(), Box<dyn std::error::Error>> {
		// let tpl = include_str!("examples/template.ron");
		let tpl = "";
		let tpl = ron::from_str::<Template>(tpl)?;

		dbg!(tpl);

		Ok(())
	}
}
