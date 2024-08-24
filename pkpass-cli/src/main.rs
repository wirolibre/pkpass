use clap::Parser;
use pkpass::{sign::VerifyMode, template::Template, Pass};
use ron::ser::PrettyConfig;
use std::{
	fs::{self, File},
	path::PathBuf,
	str::FromStr,
};

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
	/// Read and print debug structure of pass to output
	Read(ReadArgs),

	// TODO: merge into read?
	/// Convert an existing pass into a "ready to be customized" template
	Convert(ConvertArgs),

	/// Create a working pass from plain arguments
	Create(create::Args),

	/// Create a working pass from plain arguments
	Render(RenderArgs),

	#[clap(subcommand)]
	Crypto(crypto::Command),
}

impl Exec for Command {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		match self {
			Self::Read(args) => args.run(),
			Self::Convert(args) => args.run(),
			Self::Create(args) => args.run(),
			Self::Render(args) => args.run(),
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

		dbg!(pkpass.metadata);

		// TODO: need custom print, else prints pictures' bytes
		// dbg!(pkpass.assets)

		Ok(())
	}
}

#[derive(clap::Args)]
struct ConvertArgs {
	pass: PathBuf,

	output: Option<PathBuf>,
}

impl Exec for ConvertArgs {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		let output = self
			.output
			.unwrap_or_else(|| self.pass.file_name().unwrap().into())
			.with_extension("ron");
		let output = fs::OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(output)?;

		let pkpass = Pass::read(File::open(self.pass)?, VerifyMode::No)?;

		let template = Template {
			variables: Vec::default(),
			meta: pkpass.metadata,
		};

		// TODO: these make no sense in a template: passTypeIdentifier, teamIdentifier, serialNumber,

		ron::ser::to_writer_pretty(output, &template, PrettyConfig::new().struct_names(true))?;

		Ok(())
	}
}

#[derive(clap::Args)]
struct RenderArgs {
	template: PathBuf,
}

impl Exec for RenderArgs {
	fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		let template = File::open(self.template)?;
		let template = ron::de::from_reader::<_, Template>(template)?;

		dbg!(template.variables);
		dbg!(template.meta);

		// TODO: render template using passed variables

		Ok(())
	}
}
