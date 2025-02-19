use pkpass::{
	models::{Fields, PassKind},
	Pass, PassConfig,
};
use std::fs;
use uuid::Uuid;

#[path = "common/identity.rs"]
mod identity;

const ICON: &[u8; 314_069] = include_bytes!("assets/icon.png");

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let identity = identity::get_identity()?;

	let fields = Fields::default();

	let mut pass = Pass::new(PassConfig {
		organization_name: "Acme Inc.".into(),
		description: "A custom pass to try out my library".into(),
		serial_number: Uuid::new_v4().as_simple().to_string(),
		kind: PassKind::EventTicket(fields),
	});

	let Pass { assets, .. } = &mut pass;

	assets.images.icon.size_x1.replace(ICON.to_vec());

	let file = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open("custom.pkpass")?;

	pass.write(&identity, file)?;

	Ok(())
}
