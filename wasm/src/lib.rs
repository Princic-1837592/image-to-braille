use braille_ascii_art::{from_bytes, GrayMethod};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = apply)]
pub fn apply(
    bytes: String,
    width: usize,
    invert: bool,
    has_alpha: bool,
    gray_method: usize,
    monospace: bool,
) -> String {
    match from_bytes(
        &[],
        width,
        invert,
        has_alpha,
        match gray_method {
            0 => GrayMethod::Average,
            1 => GrayMethod::Lightness,
            2 => GrayMethod::Luminosity,
            _ => GrayMethod::Average,
        },
        monospace,
    ) {
        Ok(_) => "ok",
        Err(_) => "",
    }
    .to_string()
}
