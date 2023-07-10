use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use scrap::{Capturer, Display};
use show_image::{create_window, event};

use std::time::{Duration, Instant};

use pkmn::gameboy::{locate_screen, StatScreen1Layout};
use pokemon_dv_calculator as pkmn;

#[show_image::main]
fn main() {
    let symbol_bitmaps = pkmn::ocr::create_symbol_bitmaps();
    let pkmn_base_stats = pkmn::stats::load_base_stats();

    // let window_initial = create_window("Initial", Default::default()).unwrap();
    // let window_grey = create_window("Greyscale", Default::default()).unwrap();
    // let window_threshold = create_window("Threshold", Default::default()).unwrap();
    // let window_erode = create_window("Erode", Default::default()).unwrap();
    let window_gameboy = create_window("GameBoy", Default::default()).unwrap();

    loop {
        let image_initial = ImageReader::open("screenshot.png")
            .unwrap()
            .decode()
            .unwrap();

        let screen_pos = locate_screen(image_initial.clone());
        let Some(screen_pos) = screen_pos else {
            continue;  // Did not find gameBoy screen
        };

        let image_screen = image_initial.clone().crop(
            screen_pos.x,
            screen_pos.y,
            screen_pos.width,
            screen_pos.height,
        );
        window_gameboy
            .set_image("GameBoy", image_screen.clone())
            .unwrap();

        let stats_screen_layout = StatScreen1Layout::new();

        let img_screen_small = image_screen.resize_exact(
            stats_screen_layout.width as u32,
            stats_screen_layout.height as u32,
            FilterType::Nearest,
        );

        let content = stats_screen_layout.read_content(&img_screen_small, &symbol_bitmaps);

        let pkmn_no: usize = content.pkmn_no.parse().unwrap();
        let level: i32 = content.level.parse().unwrap();
        let hp: i32 = content.hp.parse().unwrap();
        let attack: i32 = content.attack.parse().unwrap();
        let defense: i32 = content.defense.parse().unwrap();
        let speed: i32 = content.speed.parse().unwrap();
        let special: i32 = content.special.parse().unwrap();

        let pkmn_base_stats = &pkmn_base_stats[pkmn_no - 1]; // -1 as Dex number starts with 1
        println!("Found this pokemon on the screen {:?}", pkmn_base_stats);

        let hp_dv = pkmn::stats::get_dv_stat_pairs(level, pkmn_base_stats.hp, 0, true);
        let attack_dv = pkmn::stats::get_dv_stat_pairs(level, pkmn_base_stats.attack, 0, false);
        let defense_dv = pkmn::stats::get_dv_stat_pairs(level, pkmn_base_stats.defense, 0, false);
        let speed_dv = pkmn::stats::get_dv_stat_pairs(level, pkmn_base_stats.speed, 0, false);
        let special_dv = pkmn::stats::get_dv_stat_pairs(level, pkmn_base_stats.special, 0, false);

        pkmn::stats::print_dv_table(&hp_dv, &attack_dv, &defense_dv, &speed_dv, &special_dv);

        let hp_dv_range = pkmn::stats::find_dv_range(&hp, &hp_dv);
        let hp_dv_range = match hp_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid HP value"),
        };

        let attack_dv_range = pkmn::stats::find_dv_range(&attack, &attack_dv);
        let attack_dv_range = match attack_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid attack value"),
        };

        let defense_dv_range = pkmn::stats::find_dv_range(&defense, &defense_dv);
        let defense_dv_range = match defense_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid defense value"),
        };

        let speed_dv_range = pkmn::stats::find_dv_range(&speed, &speed_dv);
        let speed_dv_range = match speed_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid speed value"),
        };

        let special_dv_range = pkmn::stats::find_dv_range(&special, &special_dv);
        let special_dv_range = match special_dv_range {
            Ok(val) => format!("min {:>2} - max {:>2}", val.0, val.1 - 1),
            Err(_) => String::from("Invalid special value"),
        };

        println!(" HP: {:<3} DV: {}", hp, hp_dv_range);
        println!("ATT: {:<3} DV: {}", attack, attack_dv_range);
        println!("DEF: {:<3} DV: {}", defense, defense_dv_range);
        println!("SPE: {:<3} DV: {}", speed, speed_dv_range);
        println!("SPC: {:<3} DV: {}", special, special_dv_range);

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
