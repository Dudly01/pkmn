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
    let pkmn_learnsets = pkmn::learnset::load_learnsets();
    let pkmn_evo_chains = pkmn::evos::load_evos();
    let stats_screen_1_layout = pkmn::gameboy::StatScreen1Layout::new();
    let stats_screen_2_layout = pkmn::gameboy::StatScreen2Layout::new();

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
                stats_screen_1_layout.width as u32,
                stats_screen_1_layout.height as u32,
                image::imageops::FilterType::Nearest,
            );

        let is_summary_screen_1 = stats_screen_1_layout.verify_screen(&img_gameboy);
        let is_summary_screen_2 = stats_screen_2_layout.verify_screen(&img_gameboy);

        if !is_summary_screen_1 && !is_summary_screen_2 {
            stdout
                .execute(Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?
                .execute(Print("Not showing summary screen 1 nor 2!"))?;
            continue; // Not the screens we want
        }

        if is_summary_screen_1 {
            let content = stats_screen_1_layout.read_content(&img_gameboy, &symbol_bitmaps);
            let Ok(content) = content else {
            stdout
                .execute(Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?
                .execute(Print("Could not read summary screen 1 content!"))?;
            continue;
            };

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

        if is_summary_screen_2 {
            let content = stats_screen_2_layout.read_content(&img_gameboy, &symbol_bitmaps);
            let Ok(content) = content else {
            stdout
                .execute(Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?
                .execute(Print("Could not read summary screen 2 content!"))?;
            continue;
            };

            let ndex: usize = content.pkmn_no.parse().unwrap();

            let pkmn_name = &pkmn_base_stats[ndex - 1].pokemon;
            let evo_chains: Vec<_> = pkmn_evo_chains
                .iter()
                .filter(|x| x.contains(pkmn_name))
                .collect();

            let mut pkmn_names: Vec<&str> = Vec::new();
            for chain in &evo_chains {
                let pkmn = chain.split(">").step_by(2);
                for name in pkmn {
                    if !pkmn_names.contains(&name) {
                        pkmn_names.push(name);
                    }
                }
            }

            let learnset = &pkmn_learnsets[ndex - 1];

            let evo_chain_learnsets = pkmn_names
                .iter()
                .map(|name| pkmn_base_stats.iter().find(|r| r.pokemon == *name).unwrap())
                .map(|r| r.ndex)
                .map(|ndex| &pkmn_learnsets[ndex as usize - 1])
                .collect::<Vec<&pkmn::learnset::Learnset>>();

            stdout
                .execute(Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?;

            println!("{} learnset", learnset.pokemon);
            println!(
                "{}",
                pkmn::learnset::get_pretty_learnset_table(learnset).unwrap()
            );

            println!("");
            println!("Evo chains:");
            for chain in evo_chains {
                println!("{}", chain.replace(">", "   >   "));
            }

            println!("");
            for learnset in &evo_chain_learnsets {
                println!("{} learnset", learnset.pokemon);
                println!(
                    "{}",
                    pkmn::learnset::get_pretty_learnset_table(learnset).unwrap()
                );
            }
        }
    }
}
