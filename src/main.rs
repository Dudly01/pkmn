use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use show_image::{create_window, event};

use std::time::Instant;

use pkmn::gameboy::{locate_screen, StatScreen1Layout};
use pkmn::stats::{BaseStats, DvRanges, DvTable, Experience, Stats};
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

        let ndex: usize = content.pkmn_no.parse().unwrap();
        let level: i32 = content.level.parse().unwrap();
        let stats = Stats::from_screen_content(&content);
        let record = &pkmn_base_stats[ndex - 1]; // -1 as Dex number starts with 1
        let base_stats = BaseStats::from_record(&record);

        let exp = Experience::with_no_experience();

        let dv_stats_table = DvTable::new(&level, &base_stats, &exp);

        dv_stats_table.print();

        let dv_ranges = DvRanges::new(&stats, &dv_stats_table);

        let hp = match dv_ranges.hp {
            Some(r) => format!("min {:>2} - max {:>2}", r.0, r.1),
            None => String::from("Stat is not within expectations."),
        };

        let attack = match dv_ranges.attack {
            Some(r) => format!("min {:>2} - max {:>2}", r.0, r.1),
            None => String::from("Stat is not within expectations."),
        };

        let defense = match dv_ranges.defense {
            Some(r) => format!("min {:>2} - max {:>2}", r.0, r.1),
            None => String::from("Stat is not within expectations."),
        };

        let special = match dv_ranges.special {
            Some(r) => format!("min {:>2} - max {:>2}", r.0, r.1),
            None => String::from("Stat is not within expectations."),
        };

        let speed = match dv_ranges.speed {
            Some(r) => format!("min {:>2} - max {:>2}", r.0, r.1),
            None => String::from("Stat is not within expectations."),
        };

        println!(" HP: {:<3} DV: {}", stats.hp, hp);
        println!("ATT: {:<3} DV: {}", stats.attack, attack);
        println!("DEF: {:<3} DV: {}", stats.defense, defense);
        println!("SPE: {:<3} DV: {}", stats.speed, special);
        println!("SPC: {:<3} DV: {}", stats.special, speed);

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
