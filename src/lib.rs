use std::path::PathBuf;

use image::{imageops::overlay, EncodableLayout};

const UNICODE_OFFSET: u32 = 0x2800;

#[derive(Debug, Copy, Clone)]
pub enum ConversionError {
	WidthNotEven,
	HeightNotMultipleOfFour,
}

#[derive(Debug, Copy, Clone)]
pub enum GrayMethod {
	Lightness,
	Average,
	Luminosity,
	Max,
	Min,
}

fn to_gray(pixel: [u8; 4], method: GrayMethod) -> u8 {
	let [r, g, b, _] = pixel;
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
	depth: usize,
	gray_method: GrayMethod,
	monospace: bool,
) -> String {
	let height = bytes.len() / width / depth;
	let mut result = String::with_capacity((bytes.len() / depth / 8) * 3 + height);
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
	let mut pixel_buffer = [0; 4];
	for i in (0..height).step_by(4) {
		for j in (0..width).step_by(2) {
			let mut buffer: u8 = 0b11111111;
			for (o, offset) in pixel_offsets.iter().enumerate() {
				pixel_buffer.clone_from_slice(
					&bytes[(i * width + j + offset) * depth..(i * width + j + offset) * depth + 4],
				);
				if depth == 4 {
					pixel_buffer[3] = 0xff;
				}
				if pixel_buffer[3] >= 128 && to_gray(pixel_buffer, gray_method) >= 128 {
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
	has_alpha: bool,
	gray_method: GrayMethod,
	monospace: bool,
) -> Result<String, ConversionError> {
	let depth = if has_alpha { 4 } else { 3 };
	let height = bytes.len() / depth / width;
	if width % 2 != 0 {
		return Err(ConversionError::WidthNotEven);
	}
	if height % 4 != 0 {
		return Err(ConversionError::HeightNotMultipleOfFour);
	}
	Ok(apply(bytes, width, invert, depth, gray_method, monospace))
}

pub fn from_path(
	path: PathBuf,
	invert: bool,
	new_width: Option<u32>,
	gray_method: GrayMethod,
	monospace: bool,
) -> Result<String, ConversionError> {
	let mut img = image::open(path).unwrap();
	if let Some(new_width) = new_width.map(|w| w * 2) {
		let new_height =
			(img.height() as f32 / img.width() as f32 * new_width as f32).round() as u32;
		img = img.resize(new_width, new_height, image::imageops::FilterType::Gaussian);
	}
	if img.width() % 2 != 0 {
		img = img.crop_imm(0, 0, img.width() - 1, img.height());
	}
	if img.height() % 4 != 0 {
		img = img.crop_imm(0, 0, img.width(), img.height() - img.height() % 4);
	}
	let mut white = image::RgbaImage::from_raw(
		img.width(),
		img.height(),
		vec![255; (img.width() * img.height() * 4) as usize],
	)
	.unwrap();
	overlay(&mut white, &img, 0, 0);
	from_bytes(
		white.as_bytes(),
		white.width() as usize,
		invert,
		true,
		gray_method,
		monospace,
	)
}
