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

pub fn dv_stat_table_as_string(
    dv_table: &pkmn::stats::DvTable,
    stats: &pkmn::stats::Stats,
) -> String {
    let mut text_result = String::with_capacity(128);

    text_result.push_str(&format!(
        "{: >4} {: >4} {: >4} {: >4} {: >4} {: >4}<br>",
        "DV", "HP", "ATT", "DEF", "SPD", "SPC"
    ));

    for i in 0..16 {
        let special_char = "-";

        let hp_eq = if dv_table.hp[i] == stats.hp {
            special_char
        } else {
            " "
        };
        let attack_eq = if dv_table.attack[i] == stats.attack {
            special_char
        } else {
            " "
        };
        let defense_eq = if dv_table.defense[i] == stats.defense {
            special_char
        } else {
            " "
        };
        let speed_eq = if dv_table.speed[i] == stats.speed {
            special_char
        } else {
            " "
        };
        let special_eq = if dv_table.special[i] == stats.special {
            special_char
        } else {
            " "
        };

        text_result.push_str(&format!(
            "{: >4} {: >4}{}{: >4}{}{: >4}{}{: >4}{}{: >4}{}<br>",
            i,
            dv_table.hp[i],
            hp_eq,
            dv_table.attack[i],
            attack_eq,
            dv_table.defense[i],
            defense_eq,
            dv_table.speed[i],
            speed_eq,
            dv_table.special[i],
            special_eq,
        ));
    }
    text_result
}

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
