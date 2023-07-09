use image::imageops::crop_imm;
use image::Luma;
use image::{DynamicImage, GrayImage, ImageBuffer};
use imageproc::contrast::threshold;
use imageproc::rect::Rect;
use imageproc::{contours::Contour, template_matching::match_template};
use show_image::create_window;
use std::cmp::{max, min};

/// Returns the inclusive bounding box of a contour.
pub fn get_bounding_box(contour: &Contour<i32>) -> Result<Rect, &str> {
    if contour.points.len() < 1 {
        return Err("Contour contains no points!");
    }

    let curr_point = &contour.points[0];
    let mut x_min = curr_point.x;
    let mut x_max = curr_point.x;
    let mut y_min = curr_point.y;
    let mut y_max = curr_point.y;

    for point in &contour.points[1..] {
        x_min = min(x_min, point.x);
        x_max = max(x_max, point.x);
        y_min = min(y_min, point.y);
        y_max = max(y_max, point.y);
    }

    let width = x_max - x_min + 1;
    let height = y_max - y_min + 1;
    let rectangle = Rect::at(x_min, y_min).of_size(width as u32, height as u32);
    Ok(rectangle)
}

/// Finds the GameBoy screen candidates within the contours.
/// Candidates have a minimum size of 160x140 and a ratio of ~10:9.
pub fn find_screen_candidates(contours: &Vec<Contour<i32>>) -> Vec<Rect> {
    let target_ratio = 10.0 / 9.0;
    let tolerance = 0.01;

    let mut potential_rects: Vec<Rect> = Vec::with_capacity(8);
    for contour in contours {
        let bbox = get_bounding_box(&contour).unwrap();

        if bbox.width() < 160 || bbox.height() < 144 {
            continue; // Smaller than original resolution
        }

        let ratio = bbox.width() as f32 / bbox.height() as f32;
        if (ratio - target_ratio).abs() > tolerance {
            continue; // Not within tolerance
        }

        potential_rects.push(bbox);
    }
    potential_rects
}

/// Returns a vector with the available chars, and another with their bitmaps.
/// The bitmaps have 7x7 pixels.
pub fn create_char_bitmaps() -> (Vec<char>, Vec<[u8; 49]>) {
    let mut chars: Vec<char> = Vec::with_capacity(11);
    let mut char_fonts: Vec<[u8; 49]> = Vec::with_capacity(11);

    // 0
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 1, 0, 0, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 0, 0, 1, 0, //
        0, 0, 1, 1, 1, 0, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('0');
    char_fonts.push(char);

    // 1
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 1, 1, 1, 1, 1, 1, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('1');
    char_fonts.push(char);

    // 2
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 0, 0, 0, 1, 1, 1, //
        0, 1, 1, 1, 1, 0, 0, //
        1, 1, 1, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 1, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('2');
    char_fonts.push(char);

    // 3
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 0, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('3');
    char_fonts.push(char);

    // 4
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 1, 1, 1, 0, //
        0, 0, 1, 1, 1, 1, 0, //
        0, 1, 1, 0, 1, 1, 0, //
        1, 1, 0, 0, 1, 1, 0, //
        1, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('4');
    char_fonts.push(char);

    // 5
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        0, 0, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('5');
    char_fonts.push(char);

    // 6
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('6');
    char_fonts.push(char);

    // 7
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 0, 0, 0, //
        0, 0, 1, 1, 0, 0, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('7');
    char_fonts.push(char);

    // 8
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('8');
    char_fonts.push(char);

    // 9
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push('9');
    char_fonts.push(char);

    // Empty char
    let char: [u8; 49] = [
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
    ]
    .iter()
    .map(|&n| if n == 0 { 255 } else { 0 })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
    chars.push(' ');
    char_fonts.push(char);

    (chars, char_fonts)
}

/// Returns the best matching char for the input image.
pub fn match_char(
    char_image: DynamicImage,
    known_chars: &(Vec<char>, Vec<[u8; 49]>),
) -> Result<char, &'static str> {
    if char_image.width() != 7 || char_image.height() != 7 {
        return Err("Character bitmap has incorrect dimensions.");
    }

    let img_grey = &char_image.to_luma8();
    let img_binary = threshold(&img_grey, 200);

    // let window_debug = create_window("Debug Char Match", Default::default()).unwrap();
    // window_debug
    //     .set_image("GameBoy", img_binary.clone())
    //     .unwrap();

    let mut results: Vec<i32> = Vec::with_capacity(known_chars.0.len());

    for known_bitmap in &known_chars.1 {
        let mut total_diff = 0;
        for (a, b) in known_bitmap.iter().zip(img_binary.pixels()) {
            let b_value = match b {
                Luma([v]) => v,
            };

            let diff = (*a != *b_value) as i32;

            total_diff += diff
        }
        results.push(total_diff);
    }

    let min_index = results
        .iter()
        .enumerate()
        .min_by_key(|&(_, &value)| value)
        .map(|(index, _)| index)
        .unwrap();

    let best_match = known_chars.0[min_index];
    Ok(best_match)
}

/// Returns the best string of best matching chars for the input image.
pub fn match_field(
    field_image: DynamicImage,
    known_chars: &(Vec<char>, Vec<[u8; 49]>),
) -> Result<String, &'static str> {
    if field_image.height() != 7 || (field_image.width() + 1) % 8 != 0 {
        return Err("Input dimensions are incorrect.");
    }

    let num_chars = (field_image.width() + 1) / 8;
    let mut found_chars: Vec<_> = Vec::with_capacity(num_chars as usize);

    for i in 0..num_chars {
        let x_char = i * (7 + 1);
        let y_char = 0;
        let w_char = 7;
        let h_char = 7;

        let img_char = field_image.clone().crop(x_char, y_char, w_char, h_char);
        let found_char = match_char(img_char, known_chars).unwrap();
        found_chars.push(found_char);
    }

    let field_string: String = found_chars.into_iter().collect();
    Ok(field_string)
}

/// Returns the possible DV-HP pairs based on the supplied data.
/// Only for HP.
pub fn get_dv_hp_pairs(level: i32, base: i32, exp: i32) -> Vec<i32> {
    let offset = level + 10;
    let result = calc_possible_stat_values(level, base, exp, offset);
    result
}

/// Returns the possible DV-STAT pairs based on the supplied data.
/// Not for HP.
pub fn get_dv_stat_pairs(level: i32, base: i32, exp: i32) -> Vec<i32> {
    let offset = 5;
    let result = calc_possible_stat_values(level, base, exp, offset);
    result
}

/// Returns the possible DV-STAT pairs based on the supplied data.
/// Acts as a helper function.
fn calc_possible_stat_values(level: i32, base: i32, exp: i32, offset: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(16);

    let effort_gain = ((exp - 1) as f32).sqrt() + 1.0 / 4.0;
    let effort_gain = effort_gain as i32;

    for dv in 0..16 {
        let val = (((base + dv) * 2 + effort_gain) * level) as f32 / 100.0;
        let val = val as i32 + offset;
        result.push(val);
    }

    result
}

#[derive(Debug)]
pub struct BaseStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

#[derive(Debug)]
pub struct CurrentStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

#[derive(Debug)]
pub struct StatExperience {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

pub fn print_dv_table(
    hp: &Vec<i32>,
    attack: &Vec<i32>,
    defense: &Vec<i32>,
    speed: &Vec<i32>,
    special: &Vec<i32>,
) {
    println!(
        "{: >5}{: >5}{: >5}{: >5}{: >5}{: >5}",
        "DV", "HP", "ATT", "DEF", "SPD", "SPC"
    );

    for i in 0..16 {
        let curr_hp = hp[i];
        let curr_attack = attack[i];
        let curr_defense = defense[i];
        let curr_speed = speed[i];
        let curr_special = special[i];

        println!(
            "{: >5}{: >5}{: >5}{: >5}{: >5}{: >5}",
            i, curr_hp, curr_attack, curr_defense, curr_speed, curr_special
        );
    }
}

/// Returns the range that the value is present in the sorted vector.
pub fn find_value_range(value: i32, vector: Vec<i32>) -> Result<(usize, usize), &'static str> {
    if vector.len() < 1 {
        return Err("Vector contains no values");
    }

    let mut start = -1;
    let mut end = -1;

    for (i, val) in vector.iter().enumerate() {
        if *val == value as i32 {
            start = i as i32;
            break;
        }
    }

    if start == -1 {
        return Err("Vector does not contain reference value");
    }

    for (i, val) in vector.iter().enumerate().rev() {
        if *val == value as i32 {
            end = i as i32 + 1;
            break;
        }
    }

    Ok((start as usize, end as usize))
}

/// Returns the location of the GameBoy screen on an image.
pub fn locate_gameboy_screen(img: DynamicImage) -> Option<(u32, u32, u32, u32)> {
    let img_gray = img.into_luma8();

    let threshold_val = 200;
    let img_threshold = threshold(&img_gray, threshold_val);

    let erode_size = 1;
    let image_erode = imageproc::morphology::erode(
        &img_threshold,
        imageproc::distance_transform::Norm::LInf,
        erode_size,
    );

    let contours = imageproc::contours::find_contours::<i32>(&image_erode);

    let screen_candidates = find_screen_candidates(&contours);

    let largest_candidate = screen_candidates
        .iter()
        .max_by_key(|rect| rect.width() * rect.height());

    let gameboy_screen_position = match largest_candidate {
        Some(r) => Some((
            r.left() as u32 - erode_size as u32,
            r.top() as u32 - erode_size as u32,
            r.width() + 2 * erode_size as u32,
            r.height() + 2 * erode_size as u32,
        )),
        None => None,
    };

    gameboy_screen_position
}

/// The inclusive bounding box of something.
/// The elements on the border are part of the thing.
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// The position of important data on first page of the stats screen.
/// Could also be called as the summary screen.
pub struct StatScreen1Layout {
    pub width: i32,
    pub height: i32,
    pub pkmn_ndex_pos: Position,
    pub level_field_pos: Position,
    pub hp_field_pos: Position,
    pub attack_field_pos: Position,
    pub defense_field_pos: Position,
    pub speed_field_pos: Position,
    pub special_field_pos: Position,
}

impl StatScreen1Layout {
    /// Populates the struct with the known positions.
    /// Beware, text that is of no concern are not populated.
    pub fn new() -> StatScreen1Layout {
        let field_width = 23;
        let field_height = 7;

        let x_pkmn_no = 24;
        let y_pkmn_no = 56;

        let x_level = 120;
        let y_level = 16;

        let x_hp = 150 - field_width + 1;
        let y_hp = 39 - field_height;

        let x_attack = 70 - field_width + 1;
        let y_attack = 87 - field_height;

        let x_defense = 70 - field_width + 1;
        let y_defense = 103 - field_height;

        let x_speed = 70 - field_width + 1;
        let y_speed = 119 - field_height;

        let x_special = 70 - field_width + 1;
        let y_special = 135 - field_height;

        StatScreen1Layout {
            width: 160,
            height: 144,
            pkmn_ndex_pos: Position {
                x: x_pkmn_no,
                y: y_pkmn_no,
                width: field_width,
                height: field_height,
            },
            level_field_pos: Position {
                x: x_level,
                y: y_level,
                width: field_width,
                height: field_height,
            },
            hp_field_pos: Position {
                x: x_hp,
                y: y_hp,
                width: field_width,
                height: field_height,
            },
            attack_field_pos: Position {
                x: x_attack,
                y: y_attack,
                width: field_width,
                height: field_height,
            },
            defense_field_pos: Position {
                x: x_defense,
                y: y_defense,
                width: field_width,
                height: field_height,
            },
            speed_field_pos: Position {
                x: x_speed,
                y: y_speed,
                width: field_width,
                height: field_height,
            },
            special_field_pos: Position {
                x: x_special,
                y: y_special,
                width: field_width,
                height: field_height,
            },
        }
    }
}

/// Reads the text from the image.
pub fn read_text(
    img: &DynamicImage,
    layout: &StatScreen1Layout,
    symbols: &(Vec<char>, Vec<[u8; 49]>),
) -> (String, String, String, String, String, String, String) {
    if img.width() as i32 != layout.width || img.height() as i32 != layout.height {
        panic!("Mismatch in image and layout dimensions.")
    }

    let pos = &layout.pkmn_ndex_pos;
    let img_pkmn_no = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let pkmn_no = match_field(img_pkmn_no, symbols).unwrap();

    let pos = &layout.level_field_pos;
    let img_level = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let level = match_field(img_level, symbols).unwrap();

    let pos = &layout.hp_field_pos;
    let img_hp = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let hp = match_field(img_hp, symbols).unwrap();

    let pos = &layout.attack_field_pos;
    let img_attack = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let attack = match_field(img_attack, symbols).unwrap();

    let pos = &layout.defense_field_pos;
    let img_defense = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let defense = match_field(img_defense, &symbols).unwrap();

    let pos = &layout.speed_field_pos;
    let img_speed = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let speed = match_field(img_speed, &symbols).unwrap();

    let pos = &layout.special_field_pos;
    let img_special = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let special = match_field(img_special, &symbols).unwrap();

    (pkmn_no, level, hp, attack, defense, speed, special)
}
