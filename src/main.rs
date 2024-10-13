use std::{fs::File, io::Write, path::PathBuf};

use braille_ascii_art::{from_path, Canny, GrayMethod};
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
	#[clap(short, long, default_value = "128")]
	/// Threshold to determine if a pixel is black or white. Must be between 0 and 255
	threshold: u8,
	#[clap(short, long)]
	/// Sigma value for canny edge detection
	sigma: Option<f32>,
	#[clap(short, long)]
	/// Low threshold value for canny edge detection
	low: Option<f32>,
	#[clap(short, long)]
	/// High threshold value for canny edge detection
	high: Option<f32>,
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
		cli.threshold,
		cli.sigma.and_then(|s| {
			cli.low
				.and_then(|l| cli.high.map(|h| Canny::new(s, l, h).unwrap()))
		}),
	)
	.unwrap();
	let mut file = File::create(cli.dest).unwrap();
	file.write_all(art.as_bytes()).unwrap();
}
