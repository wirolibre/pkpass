// pkpass create custom.pkpass \
//       --logo icon.png --title Meliès \
//       --barcode qr:"2fa8bcf0-6bf2-4c18-ada7-d0a203592652@INT" \
//       --event --primary-field "Lundi 22 Juillet":"20h00" \
//       --location

use crate::Exec;
use std::path::PathBuf;

#[derive(clap::Args)]
pub(crate) struct Args {
	#[arg(long, env)]
	sign: Option<PathBuf>,
}

impl Exec for Args {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		// let pass = PkPass::new();

		Ok(())
	}
}
