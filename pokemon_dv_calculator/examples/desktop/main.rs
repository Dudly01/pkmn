/// Finds the GameBoy on the primary display and shows the result in the terminal.
pub mod screen_capturer;

use crossterm::{
    cursor,
    style::Print,
    terminal::{self, Clear},
    ExecutableCommand, Result,
};
use image::DynamicImage;
use std::io::stdout;

use pokemon_dv_calculator as pkmn;

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout
        .execute(terminal::SetTitle("PKMN DV calc"))?
        .execute(cursor::Hide)?;

    let capturer = screen_capturer::ScreenCapturer::for_primary_display();
    let Ok(mut capturer) = capturer else {
        panic!("There was an error in capturing the primary display.");
    };

    let symbol_bitmaps = pkmn::ocr::create_symbol_bitmaps();
    let pkmn_base_stats = pkmn::stats::load_base_stats();
    let stats_screen_layout = pkmn::gameboy::StatScreen1Layout::new();

    let mut previous_content: Option<pkmn::gameboy::StatsSreen1Content> = None;

    loop {
        let img_screen = capturer.next_frame();
        let Ok(img_screen) = img_screen else {
            panic!("There was an error retrieving the display frame.")
        };
        let img_screen = DynamicImage::ImageRgb8(img_screen.clone());

        let gameboy_pos = pkmn::gameboy::locate_screen(&img_screen);
        let Some(gameboy_pos) = gameboy_pos else {
            stdout
                .execute(Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?
                .execute(Print("No GameBoy screen was found!"))?;
            continue;
        };

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
            stdout
                .execute(Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?
                .execute(Print("Not showing summary screen 1!"))?;
            continue; // Not the screen we want
        }

        let content = stats_screen_layout.read_content(&img_gameboy, &symbol_bitmaps);
        let Ok(content) = content else {
            stdout
                .execute(Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?
                .execute(Print("Could not read summary screen content!"))?;
            continue;
        };

        // if previous_content == Some(content.clone()) {
        //     continue;
        // }
        // previous_content = Some(content.clone());

        let ndex: usize = content.pkmn_no.parse().unwrap();
        let level: i32 = content.level.parse().unwrap();
        let stats = pkmn::stats::Stats::from_screen_content(&content);
        let record = &pkmn_base_stats[ndex - 1]; // -1 as Dex number starts with 1
        let base_stats = pkmn::stats::BaseStats::from_record(&record);

        let exp = pkmn::stats::Experience::with_no_experience();

        let dv_stats_table = pkmn::stats::DvTable::new(&level, &base_stats, &exp);

        let dv_ranges = pkmn::stats::DvRanges::new(&stats, &dv_stats_table);

        let result = pkmn::stats::summarize_pkmn_stats(
            record,
            &base_stats,
            level,
            &stats,
            &dv_stats_table,
            &dv_ranges,
        );

        stdout
            .execute(Clear(terminal::ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?;

        println!("{}", result);
    }
}
