use clap::Parser;
use pkpass::PkPass;
use std::{fs::File, path::PathBuf};

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
	Read {
		file: PathBuf,
	},
	Create(create::Args),
	#[clap(subcommand)]
	Crypto(crypto::Command),
}

impl Exec for Command {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		match self {
			Self::Read { file } => read_pkpass(file),
			Self::Create(args) => args.run(),
			Self::Crypto(args) => args.run(),
		}
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Cli::parse();
	args.command.run()
}

fn read_pkpass(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
	let pkpass = PkPass::read(File::open(file)?)?;
	dbg!(pkpass);
	Ok(())
}
