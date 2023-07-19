mod utils;

use pokemon_dv_calculator as pkmn;
use wasm_bindgen::prelude::*;

use image::{DynamicImage, ImageBuffer, Rgba}; 

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn draw(pixels: &mut [u8]) {
    let count = pixels.len();

    for i in (0..count).step_by(4) {
        if (i / 4) % 10 == 0 {
            pixels[i] = 255; // Red component
            pixels[i + 1] = 255; // Green component
            pixels[i + 2] = 0; // Blue component
        }
    }
}

#[wasm_bindgen]
pub struct JsPosition {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
pub fn locate_gameboy(data: &[u8], width: u32, height: u32) -> Result<JsPosition, JsValue> {
    // Container for the image
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    // Copy over the data
    for y in 0..height as usize {
        for x in 0..width as usize {
            let i = y * width as usize + 4 * x;

            let rgba_pixel = Rgba([data[i], data[i + 1], data[i + 2], data[i + 3]]);
            img.put_pixel(x as u32, y as u32, rgba_pixel);
        }
    }

    let img = DynamicImage::ImageRgba8(img);

    let pos = pkmn::gameboy::locate_screen(&img);

    let Some(pos) = pos else {
        return Err(JsValue::from_str("No GameBoy was found"));
    };

    let pos = JsPosition{
        x: pos.x, 
        y: pos.y, 
        width: pos.width, 
        height: pos.height,
    };

    Ok(pos)
}
