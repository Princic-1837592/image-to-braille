use braille_ascii_art::{from_bytes, Canny, ConversionError, GrayMethod};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn parse(
	bytes: &[u8],
	width: usize,
	invert: bool,
	gray_method: usize,
	monospace: bool,
	threshold: u8,
	sigma: f32,
	low: f32,
	high: f32,
) -> Result<String, String> {
	from_bytes(
		bytes,
		width,
		invert,
		match gray_method {
			0 => GrayMethod::Average,
			1 => GrayMethod::Lightness,
			2 => GrayMethod::Luminosity,
			3 => GrayMethod::Max,
			_ => GrayMethod::Min,
		},
		monospace,
		threshold,
		Some(Canny::new(sigma, low, high).map_err(|_| "InvalidCannySettings")?),
	)
	.map_err(|error| {
		match error {
			ConversionError::WidthNotEven => "WidthNotEven",
			ConversionError::HeightNotMultipleOfFour => "HeightNotMultipleOfFour",
			ConversionError::InvalidLowThreshold => "InvalidLowThreshold",
			ConversionError::InvalidHighThreshold => "InvalidHighThreshold",
			ConversionError::InvalidBytes => "InvalidBytes",
		}
		.to_string()
	})
}
