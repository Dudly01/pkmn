mod utils;

use wasm_bindgen::prelude::*;

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
