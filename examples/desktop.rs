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

    let capturer = pkmn::screen_capturer::ScreenCapturer::for_primary_display();
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

        stdout
            .execute(Clear(terminal::ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?;

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

        println!(
            "{: <10} No.{: >3} :L{: <3}",
            record.pokemon, content.pkmn_no, level
        );

        println!();
        println!("Stats and DV [min:max]");
        println!(" HP: {:>3}    {}", stats.hp, hp);
        println!("ATT: {:>3}    {}", stats.attack, attack);
        println!("DEF: {:>3}    {}", stats.defense, defense);
        println!("SPD: {:>3}    {}", stats.speed, speed);
        println!("SPC: {:>3}    {}", stats.special, special);

        println!();
        println!("Base stats");

        println!(" HP: {:>3}", base_stats.hp);
        println!("ATT: {:>3}", base_stats.attack);
        println!("DEF: {:>3}", base_stats.defense);
        println!("SPD: {:>3}", base_stats.speed);
        println!("SPC: {:>3}", base_stats.special);
        println!("TOT: {:>3}", record.total);

        println!();
        println!("DV-Stats table");
        dv_stats_table.print(&stats);
    }
}
