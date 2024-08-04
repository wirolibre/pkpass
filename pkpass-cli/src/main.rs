use clap::Parser;
use pkpass::{sign::VerifyMode, Pass};
use std::{fs::File, path::PathBuf, str::FromStr};

mod create;
mod crypto;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

trait Exec {
	fn run(self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(clap::Subcommand)]
enum Command {
	Read(ReadArgs),
	Create(create::Args),
	#[clap(subcommand)]
	Crypto(crypto::Command),
}

impl Exec for Command {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		match self {
			Self::Read(args) => args.run(),
			Self::Create(args) => args.run(),
			Self::Crypto(args) => args.run(),
		}
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Cli::parse();
	args.command.run()
}

#[derive(clap::Args)]
struct ReadArgs {
	file: PathBuf,

	#[clap(long, value_parser = VerifyMode::from_str, default_value_t)]
	verify_mode: VerifyMode,
}

impl Exec for ReadArgs {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		let pkpass = Pass::read(File::open(self.file)?, self.verify_mode)?;
		println!("{pkpass:?}");
		Ok(())
	}
}
