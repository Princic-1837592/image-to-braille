use std::path::PathBuf;

use image::{imageops, imageops::overlay, open, DynamicImage, GrayImage, RgbImage, RgbaImage};

const UNICODE_OFFSET: u32 = 0x2800;

type Result<T> = std::result::Result<T, ConversionError>;

#[derive(Debug, Copy, Clone)]
pub enum ConversionError {
	WidthNotEven,
	HeightNotMultipleOfFour,
	InvalidLowThreshold,
	InvalidHighThreshold,
	InvalidBytes,
}

#[derive(Debug, Copy, Clone)]
pub enum GrayMethod {
	Lightness,
	Average,
	Luminosity,
	Max,
	Min,
}

#[derive(Debug, Copy, Clone)]
pub struct Canny {
	sigma: f32,
	low: f32,
	high: f32,
}

impl Canny {
	pub fn new(sigma: f32, low: f32, high: f32) -> Result<Self> {
		if !(0.0..=1.0).contains(&low) || low >= high {
			return Err(ConversionError::InvalidLowThreshold);
		}
		if !(0.0..=1.0).contains(&high) {
			return Err(ConversionError::InvalidHighThreshold);
		}
		Ok(Self { sigma, low, high })
	}
}

fn to_gray(pixel: [u8; 3], method: GrayMethod) -> u8 {
	let [r, g, b] = pixel;
	match method {
		GrayMethod::Lightness => {
			let max = r.max(g).max(b);
			let min = r.min(g).min(b);
			((max as u16 + min as u16) / 2) as u8
		}
		GrayMethod::Average => ((r as u16 + g as u16 + b as u16) / 3) as u8,
		GrayMethod::Luminosity => {
			(0.21 * r as f32 + 0.72 * g as f32 + 0.07 * b as f32).round() as u8
		}
		GrayMethod::Max => r.max(g).max(b),
		GrayMethod::Min => r.min(g).min(b),
	}
}

fn apply(
	bytes: &[u8],
	width: usize,
	invert: bool,
	gray_method: GrayMethod,
	monospace: bool,
	threshold: u8,
) -> String {
	let height = bytes.len() / width / 3;
	let mut result = String::with_capacity((bytes.len() / 3 / 8) * 3 + height);
	let pixel_offsets = [
		0,
		width,
		2 * width,
		1,
		width + 1,
		2 * width + 1,
		3 * width,
		3 * width + 1,
	];
	let mut pixel_buffer = [0; 3];
	for i in (0..height).step_by(4) {
		for j in (0..width).step_by(2) {
			let mut buffer: u8 = 0b11111111;
			for (o, offset) in pixel_offsets.iter().enumerate() {
				pixel_buffer.clone_from_slice(
					&bytes[(i * width + j + offset) * 3..(i * width + j + offset) * 3 + 3],
				);
				if to_gray(pixel_buffer, gray_method) >= threshold {
					buffer ^= 1 << o;
				}
			}
			if invert {
				buffer = !buffer;
			}
			if !monospace && buffer == 0 {
				buffer = 4;
			}
			result.push(char::from_u32(UNICODE_OFFSET + buffer as u32).unwrap());
		}
		result.push('\n');
	}
	result
}

pub fn from_bytes(
	bytes: &[u8],
	width: usize,
	invert: bool,
	gray_method: GrayMethod,
	monospace: bool,
	threshold: u8,
	canny: Option<Canny>,
) -> Result<String> {
	let height = bytes.len() / 3 / width;
	if width % 2 != 0 {
		return Err(ConversionError::WidthNotEven);
	}
	if height % 4 != 0 {
		return Err(ConversionError::HeightNotMultipleOfFour);
	}
	if let Some(canny) = canny {
		let img = DynamicImage::ImageRgb8(
			RgbImage::from_raw(width as u32, height as u32, bytes.to_vec())
				.ok_or(ConversionError::InvalidBytes)?,
		);
		let img = apply_canny(img.to_luma8(), canny);
		return Ok(apply(
			img.as_bytes(),
			img.width() as usize,
			invert,
			gray_method,
			monospace,
			1,
		));
	}
	Ok(apply(
		bytes,
		width,
		invert,
		gray_method,
		monospace,
		threshold,
	))
}

pub fn from_path(
	path: PathBuf,
	invert: bool,
	new_width: Option<u32>,
	gray_method: GrayMethod,
	monospace: bool,
	mut threshold: u8,
	mut canny: Option<Canny>,
) -> Result<String> {
	let img = open(path).unwrap();
	let mut white = RgbaImage::from_raw(
		img.width(),
		img.height(),
		vec![255; (img.width() * img.height() * 4) as usize],
	)
	.unwrap();
	overlay(&mut white, &img, 0, 0);
	let mut img = DynamicImage::ImageRgb8(DynamicImage::ImageRgba8(white).to_rgb8());
	if canny.is_some() {
		let canny = canny.take().unwrap();
		threshold = 1;
		img = apply_canny(img.to_luma8(), canny)
	}
	if let Some(new_width) = new_width.map(|w| w * 2) {
		let new_height =
			(img.height() as f32 / img.width() as f32 * new_width as f32).round() as u32;
		img = img.resize_to_fill(
			new_width - new_width % 2,
			new_height - new_height % 4,
			imageops::FilterType::Gaussian,
		);
	}
	from_bytes(
		img.as_bytes(),
		img.width() as usize,
		invert,
		gray_method,
		monospace,
		threshold,
		canny,
	)
}

fn apply_canny(img: GrayImage, Canny { sigma, low, high }: Canny) -> DynamicImage {
	edge_detection::canny(img, sigma, high, low).as_image()
}
