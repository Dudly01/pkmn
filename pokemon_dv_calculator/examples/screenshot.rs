/// Finds the GameBoy on the "screenshot.png", shows images in windows, shows stats in terminal.
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use show_image::{create_window, event};

use pkmn::gameboy::{locate_screen, StatScreen1Layout};
use pkmn::stats::{BaseStats, DvRanges, DvTable, Experience, Stats};
use pokemon_dv_calculator as pkmn;

#[show_image::main]
fn main() -> Result<(), String> {
    let symbol_bitmaps = pkmn::ocr::create_symbol_bitmaps();
    let pkmn_base_stats = pkmn::stats::load_base_stats();
    let pkmn_learnsets = pkmn::learnset::load_learnsets();
    let stats_screen_layout = StatScreen1Layout::new();

    // let window_initial = create_window("Initial", Default::default()).unwrap();
    // let window_grey = create_window("Greyscale", Default::default()).unwrap();
    // let window_threshold = create_window("Threshold", Default::default()).unwrap();
    // let window_erode = create_window("Erode", Default::default()).unwrap();
    let window_gameboy = create_window("GameBoy", Default::default()).unwrap();

    let image_initial = ImageReader::open("screenshot.png")
        .unwrap()
        .decode()
        .unwrap();

    let screen_pos = locate_screen(&image_initial);
    let Some(screen_pos) = screen_pos else {
        return Err("Did not find GameBoy screen".to_string());
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

    let img_screen_small = image_screen.resize_exact(
        stats_screen_layout.width as u32,
        stats_screen_layout.height as u32,
        FilterType::Nearest,
    );

    let is_stats_screen = stats_screen_layout.verify_screen(&img_screen_small);
    if !is_stats_screen {
        return Err("GameBoy not showing Summary screen 1".to_string());
    }

    let content = stats_screen_layout.read_content(&img_screen_small, &symbol_bitmaps);
    let Ok(content) = content else {
        return Err("Could not read content from summary screen 1".to_string());
        };

    let ndex: usize = content.pkmn_no.parse().unwrap();
    let level: i32 = content.level.parse().unwrap();
    let stats = Stats::from_screen_content(&content);
    let record = &pkmn_base_stats[ndex - 1]; // -1 as Dex number starts with 1
    let base_stats = BaseStats::from_record(&record);

    let exp = Experience::with_no_experience();

    let dv_stats_table = DvTable::new(&level, &base_stats, &exp);

    let dv_ranges = DvRanges::new(&stats, &dv_stats_table);

    let result = pkmn::stats::summarize_pkmn_stats(
        record,
        &base_stats,
        level,
        &stats,
        &dv_stats_table,
        &dv_ranges,
    );

    println!("{}", result);

    let learnset = &pkmn_learnsets[ndex];
    let result = pkmn::learnset::get_pretty_learnset_table(learnset).unwrap();
    println!("{}", result);

    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.
    for event in window_gameboy.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                return Ok(());
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

    Ok(())
}
