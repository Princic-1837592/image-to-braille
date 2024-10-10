use std::{fs::File, io::Write, path::PathBuf};

use braille_ascii_art::{from_path, GrayMethod};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Cli {
	/// Path to the image to convert
	path: PathBuf,
	/// Destination path to save the braille art
	dest: PathBuf,
	#[clap(short, long, action)]
	/// Invert the braille pattern
	invert: bool,
	#[clap(short, long)]
	/// Width of the braille file in characters
	width: Option<u32>,
	#[clap(short, long)]
	/// Method to convert image to grayscale
	gray: Option<CliGrayMethod>,
	#[clap(short, long, action)]
	/// If not set, empty spaces will be filled with a single dot to ensure correct spacing
	monospace: bool,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum CliGrayMethod {
	Lightness,
	Average,
	Luminosity,
	Max,
	Min,
}

impl From<CliGrayMethod> for GrayMethod {
	fn from(value: CliGrayMethod) -> Self {
		match value {
			CliGrayMethod::Lightness => GrayMethod::Lightness,
			CliGrayMethod::Average => GrayMethod::Average,
			CliGrayMethod::Luminosity => GrayMethod::Luminosity,
			CliGrayMethod::Max => GrayMethod::Max,
			CliGrayMethod::Min => GrayMethod::Min,
		}
	}
}

fn main() {
	let cli = Cli::parse();
	let art = from_path(
		cli.path,
		cli.invert,
		cli.width,
		cli.gray.unwrap_or(CliGrayMethod::Luminosity).into(),
		cli.monospace,
	)
	.unwrap();
	let mut file = File::create(cli.dest).unwrap();
	file.write_all(art.as_bytes()).unwrap();
}
