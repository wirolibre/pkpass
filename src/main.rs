use clap::{error::Result, Parser};
use pkpass::{
	sign::{Identity, SigningPen},
	PkPass,
};
use std::{fs::File, path::PathBuf};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
	Read { file: PathBuf },
	Create(CreateArgs),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Cli::parse();

	match args.command {
		Command::Read { file } => {
			match file.extension() {
				Some(ext) if ext == "pkpass" => {}
				_ => eprintln!("warning: file doesn't have a pkpass file ext, still trying"),
			}

			let pkpass = PkPass::read(File::open(file)?)?;
			dbg!(pkpass);

			Ok(())
		}
		Command::Create(args) => create_pass(args),
	}
}

#[derive(clap::Args)]
struct CreateArgs {
	#[arg(long, env)]
	sign: Option<String>,
}

fn create_pass(args: CreateArgs) -> Result<(), Box<dyn std::error::Error>> {
	// let pass = PkPass::new();

	Ok(())
}
