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

pub fn dv_stat_table_as_string(dv_table: &pkmn::stats::DvTable, stats: &pkmn::stats::Stats) -> String {
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

    let gameboy_pos = pkmn::gameboy::locate_screen(&img_screen);
    let Some(gameboy_pos) = gameboy_pos else {
            return Err(JsValue::from_str("No GameBoy screen was found!"));
    };

    let symbol_bitmaps = pkmn::ocr::create_symbol_bitmaps();
    let pkmn_base_stats = pkmn::stats::load_base_stats();
    let stats_screen_layout = pkmn::gameboy::StatScreen1Layout::new();

    let img_gameboy = img_screen
        .clone()
        .crop(
            gameboy_pos.x,
            gameboy_pos.y,
            gameboy_pos.width,
            gameboy_pos.height,
        )
        .resize_exact(
            stats_screen_layout.width as u32,
            stats_screen_layout.height as u32,
            image::imageops::FilterType::Nearest,
        );

    if stats_screen_layout.verify_screen(&img_gameboy) == false {
        return Err(JsValue::from_str("Not showing summary screen 1!"));
    };

    let content = stats_screen_layout.read_content(&img_gameboy, &symbol_bitmaps);
    let Ok(content) = content else {
        return Err(JsValue::from_str("Could not read summary screen content!"));
    };

    let ndex: usize = content.pkmn_no.parse().unwrap();
    let level: i32 = content.level.parse().unwrap();
    let stats = pkmn::stats::Stats::from_screen_content(&content);
    let record = &pkmn_base_stats[ndex - 1]; // -1 as Dex number starts with 1
    let base_stats = pkmn::stats::BaseStats::from_record(&record);

    let exp = pkmn::stats::Experience::with_no_experience();

    let dv_stats_table = pkmn::stats::DvTable::new(&level, &base_stats, &exp);

    let dv_ranges = pkmn::stats::DvRanges::new(&stats, &dv_stats_table);

    let hp = match dv_ranges.hp {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let attack = match dv_ranges.attack {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let defense = match dv_ranges.defense {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let speed = match dv_ranges.speed {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let special = match dv_ranges.special {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let mut text_result = String::with_capacity(128);

    text_result.push_str(&format!(
        "{: <} No.{: >3} :L{: <3}<br>",
        record.pokemon, content.pkmn_no, level
    ));

    text_result.push_str(&format!("<br>"));
    text_result.push_str(&format!("Stats       DVs [min:max]<br>"));
    text_result.push_str(&format!(" HP: {:>3}    {}<br>", stats.hp, hp));
    text_result.push_str(&format!("ATT: {:>3}    {}<br>", stats.attack, attack));
    text_result.push_str(&format!("DEF: {:>3}    {}<br>", stats.defense, defense));
    text_result.push_str(&format!("SPD: {:>3}    {}<br>", stats.speed, speed));
    text_result.push_str(&format!("SPC: {:>3}    {}<br>", stats.special, special));

    text_result.push_str(&format!("<br>"));
    text_result.push_str(&format!("Base stats<br>"));
    text_result.push_str(&format!(
        "{: >3}  {: >3}  {: >3}  {: >3}  {: >3}  {: >3}<br>",
        " HP", "ATT", "DEF", "SPC", "SPD", "SUM"
    ));
    text_result.push_str(&format!(
        "{: >3}  {: >3}  {: >3}  {: >3}  {: >3}  {: >3}<br>",
        base_stats.hp,
        base_stats.attack,
        base_stats.defense,
        base_stats.speed,
        base_stats.special,
        record.total,
    ));

    text_result.push_str(&format!("<br>"));
    text_result.push_str(&format!("DV-Stats table<br>"));
    text_result.push_str(&dv_stat_table_as_string(&dv_stats_table, &stats));

    Ok(JsValue::from_str(&text_result))
}
