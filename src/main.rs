use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::Rgba;
use image::{DynamicImage, GrayImage, RgbImage};
use imageproc::contrast::threshold;
use imageproc::drawing::draw_hollow_rect;
use imageproc::rect::Rect;
use pkmn::{
    create_char_bitmaps, find_value_range, get_dv_hp_pairs, get_dv_stat_pairs,
    locate_gameboy_screen, match_field, print_dv_table, BaseStats, CurrentStats, StatScreenLayout,
};
use scrap::{Capturer, Display};
use show_image::{create_window, event};

use std::time::{Duration, Instant};

use pokemon_dv_calculator as pkmn;
mod pkmn_stats;

#[show_image::main]
fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    let known_chars = create_char_bitmaps();

    // let window_initial = create_window("Initial", Default::default()).unwrap();
    // let window_grey = create_window("Greyscale", Default::default()).unwrap();
    // let window_threshold = create_window("Threshold", Default::default()).unwrap();
    // let window_erode = create_window("Erode", Default::default()).unwrap();
    let window_gameboy = create_window("GameBoy", Default::default()).unwrap();

    loop {
        // // Wait until there's a frame.
        // let buffer = match capturer.frame() {
        //     Ok(buffer) => buffer,
        //     Err(error) => {
        //         if error.kind() == WouldBlock {
        //             // Keep spinning.
        //             thread::sleep(one_frame);
        //             continue;
        //         } else {
        //             panic!("Error: '{}'", error);
        //         }
        //     }
        // };

        // // Convert BGRA buffer into dense RGB array
        // let mut raw_pixels: Vec<u8> = Vec::with_capacity(w * h * 3);
        // let stride = buffer.len() / h;
        // for y in 0..h {
        //     for x in 0..w {
        //         let i = stride * y + 4 * x;
        //         raw_pixels.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i]]);
        //     }
        // }

        // Show initial image
        // let image_initial = RgbImage::from_raw(w as u32, h as u32, raw_pixels).unwrap();

        let image_initial = ImageReader::open("screenshot.png")
            .unwrap()
            .decode()
            .unwrap();

        let gb_screen_pos = locate_gameboy_screen(image_initial.clone());

        let Some((x, y, width, height)) = gb_screen_pos else {
            continue;  // Did not find gameBoy screen
        };

        let image_screen = image_initial.clone().crop(x, y, width, height);

        window_gameboy
            .set_image("GameBoy", image_screen.clone())
            .unwrap();

        let stats_screen_layout = StatScreenLayout::new();

        let mut img_screen_small = image_screen.resize_exact(
            stats_screen_layout.width as u32,
            stats_screen_layout.height as u32,
            FilterType::Nearest,
        );

        let pos = stats_screen_layout.pkmn_ndex_pos;
        let img_pkmn_no = img_screen_small.crop(pos.x, pos.y, pos.width, pos.height);
        let pkmn_no = match_field(img_pkmn_no, &known_chars).unwrap();
        println!("No: '{}'", pkmn_no);

        let pos = stats_screen_layout.level_field_pos;
        let img_level = img_screen_small.crop(pos.x, pos.y, pos.width, pos.height);
        let level = match_field(img_level, &known_chars).unwrap();
        println!("level: '{}'", level);

        let pos = stats_screen_layout.hp_field_pos;
        let img_hp = img_screen_small.crop(pos.x, pos.y, pos.width, pos.height);
        let hp = match_field(img_hp, &known_chars).unwrap();
        println!("hp: '{}'", hp);

        let pos = stats_screen_layout.attack_field_pos;
        let img_attack = img_screen_small.crop(pos.x, pos.y, pos.width, pos.height);
        let attack = match_field(img_attack, &known_chars).unwrap();
        println!("attack: '{}'", attack);

        let pos = stats_screen_layout.defense_field_pos;
        let img_defense = img_screen_small.crop(pos.x, pos.y, pos.width, pos.height);
        let defense = match_field(img_defense, &known_chars).unwrap();
        println!("defense: '{}'", defense);

        let pos = stats_screen_layout.speed_field_pos;
        let img_speed = img_screen_small.crop(pos.x, pos.y, pos.width, pos.height);
        let speed = match_field(img_speed, &known_chars).unwrap();
        println!("speed: '{}'", speed);

        let pos = stats_screen_layout.special_field_pos;
        let img_special = img_screen_small.crop(pos.x, pos.y, pos.width, pos.height);
        let special = match_field(img_special, &known_chars).unwrap();
        println!("special: '{}'", special);

        let hp: i32 = hp.trim().parse().expect("Could not parse hp");
        let attack: i32 = attack.trim().parse().expect("Could not parse attack");
        let defense: i32 = defense.trim().parse().expect("Could not parse defense");
        let speed: i32 = speed.trim().parse().expect("Could not parse speed");
        let special: i32 = special.trim().parse().expect("Could not parse special");

        let current_stats = CurrentStats {
            hp: hp,
            attack: attack,
            defense: defense,
            speed: speed,
            special: special,
        };

        println!("{:?}", current_stats);

        let base_stats = pkmn_stats::pkmn_stats::load_stats();
        // for stat in &base_stats {
        //     println!("{:?}", stat)
        // }

        let pkmn_no: usize = pkmn_no.parse().unwrap();
        let found_pkmn_stats = &base_stats[pkmn_no - 1]; // -1 as Dex number starts with 1

        println!("Found this pokemon on the screen {:?}", found_pkmn_stats);

        let current_base_stats = BaseStats {
            hp: found_pkmn_stats.hp,
            attack: found_pkmn_stats.attack,
            defense: found_pkmn_stats.defense,
            speed: found_pkmn_stats.speed,
            special: found_pkmn_stats.special,
        };

        println!("{:?}", current_base_stats);

        let level = level
            .trim()
            .parse()
            .expect("Could not parse level into int");
        let hp_dv = get_dv_hp_pairs(level, current_base_stats.hp, 0);
        let attack_dv = get_dv_stat_pairs(level, current_base_stats.attack, 0);
        let defense_dv = get_dv_stat_pairs(level, current_base_stats.defense, 0);
        let speed_dv = get_dv_stat_pairs(level, current_base_stats.speed, 0);
        let special_dv = get_dv_stat_pairs(level, current_base_stats.special, 0);

        print_dv_table(&hp_dv, &attack_dv, &defense_dv, &speed_dv, &special_dv);

        let hp_dv_range = find_value_range(current_stats.hp, hp_dv);

        let hp_dv_range = match hp_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid HP value"),
        };

        println!(" HP DV: {}", hp_dv_range);

        let attack_dv_range = find_value_range(current_stats.attack, attack_dv);

        let attack_dv_range = match attack_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid attack value"),
        };

        println!("ATT DV: {}", attack_dv_range);

        let defense_dv_range = find_value_range(current_stats.defense, defense_dv);

        let defense_dv_range = match defense_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid defense value"),
        };

        println!("DEF DV: {}", defense_dv_range);

        let speed_dv_range = find_value_range(current_stats.speed, speed_dv);

        let speed_dv_range = match speed_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid speed value"),
        };

        println!("SPE DV: {}", speed_dv_range);

        let special_dv_range = find_value_range(current_stats.special, special_dv);

        let special_dv_range = match special_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid special value"),
        };

        println!("SPC DV: {}", special_dv_range);

        // Print keyboard events until Escape is pressed, then exit.
        // If the user closes the window, the channel is closed and the loop also exits.
        let time_wait = Instant::now();
        for event in window_gameboy.event_channel().unwrap() {
            if let event::WindowEvent::KeyboardInput(event) = event {
                println!("{:#?}", event);
                if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                    && event.input.state.is_pressed()
                {
                    return;
                }

                if event.input.key_code == Some(event::VirtualKeyCode::S)
                    && event.input.state.is_pressed()
                {
                    image_screen
                        .save("gameboy.png")
                        .expect("Could not save image");
                }
            }
            // if time_wait.elapsed().as_millis() > 50 {
            //     break;
            // }
        }
    }
}
