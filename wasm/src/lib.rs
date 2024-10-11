use braille_ascii_art::{from_bytes, ConversionError, GrayMethod};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = apply)]
pub fn apply(
    bytes: &[u8],
    width: usize,
    invert: bool,
    has_alpha: bool,
    gray_method: usize,
    monospace: bool,
) -> Result<String, String> {
    from_bytes(
        bytes,
        width,
        invert,
        has_alpha,
        match gray_method {
            0 => GrayMethod::Average,
            1 => GrayMethod::Lightness,
            2 => GrayMethod::Luminosity,
            3 => GrayMethod::Max,
            _ => GrayMethod::Min,
        },
        monospace,
    )
    .map_err(|error| {
        match error {
            ConversionError::WidthNotEven => "WidthNotEven",
            ConversionError::HeightNotMultipleOfFour => "HeightNotMultipleOfFour",
        }
        .to_string()
    })
}
