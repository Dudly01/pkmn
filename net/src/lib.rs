mod utils;

use core as pkmn;
use wasm_bindgen::prelude::*;

use image::{DynamicImage, ImageBuffer, Rgba};

/// Locates the GameBoy, reads the contents of the summary screen 1
/// and returns the stats of the found pokemon.
#[wasm_bindgen]
pub fn read_stats_from_screen(data: &[u8], width: u32, height: u32) -> Result<JsValue, JsValue> {
    if data.len() != (width * height * 4) as usize {
        return Err(JsValue::from_str("Dimensions do not add up."));
    }

    // Container for the image
    let mut img_screen: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Copy over the data from the pixelData
    for y in 0..height {
        for x in 0..width {
            let i = (y * width * 4 + x * 4) as usize;
            let rgba_pixel = Rgba([data[i], data[i + 1], data[i + 2], data[i + 3]]);
            img_screen.put_pixel(x, y, rgba_pixel);
        }
    }

    let img_screen = DynamicImage::ImageRgba8(img_screen);

    let scan_result = pkmn::utils::scan_img(img_screen);

    let text_output = match scan_result {
        Ok(text_output) => text_output,
        Err(error) => error,
    };
    let text_output = text_output.replace("\n", "<br>");

    Ok(JsValue::from_str(&text_output))
}
