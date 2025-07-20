use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PPMImage {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl PPMImage {
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[wasm_bindgen]
    pub fn data(&self) -> Vec<u8> {
        self.pixels.clone()
    }
}

#[wasm_bindgen]
pub fn parse_ppm(ppm_text: &str) -> Result<PPMImage, JsValue> {
    let lines = ppm_text
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect::<Vec<_>>();

    if lines.len() < 3 {
        return Err(JsValue::from_str("Invalid PPM: Not enough header lines"));
    }

    if lines[0].trim() != "P3" {
        return Err(JsValue::from_str("Only P3 format is supported"));
    }

    let dims = lines[1]
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| JsValue::from_str("Invalid dimensions"))?;

    if dims.len() != 2 {
        return Err(JsValue::from_str("Dimensions must be 2 numbers"));
    }

    let (width, height) = (dims[0], dims[1]);

    let _max_val: usize = lines[2]
        .trim()
        .parse()
        .map_err(|_| JsValue::from_str("Invalid max color value"))?;

    let pixel_values = lines[3..]
        .join(" ")
        .split_whitespace()
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| JsValue::from_str("Invalid pixel data"))?;

    if pixel_values.len() != width * height * 3 {
        return Err(JsValue::from_str("Pixel count does not match dimensions"));
    }

    Ok(PPMImage {
        width,
        height,
        pixels: pixel_values,
    })
}
