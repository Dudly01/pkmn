use crate as pkmn;
use image::DynamicImage;

/// Scans the image and returns the printable text.
/// The summary screen 1 is for printing the stat DVs.
/// The summary screen 2 us for printing the learnset and evolution chain.
pub fn scan_img(img_screen: DynamicImage) -> Result<String, String> {
    // Init data
    let symbol_bitmaps = pkmn::ocr::create_symbol_bitmaps();
    let pkmn_base_stats = pkmn::stats::load_base_stats();
    let pkmn_learnsets = pkmn::learnset::load_learnsets();
    let pkmn_evo_chains = pkmn::evos::load_evos();
    let stats_screen_1_layout = pkmn::gameboy::StatScreen1Layout::new();
    let stats_screen_2_layout = pkmn::gameboy::StatScreen2Layout::new();

    // Do actual scanning
    let gameboy_pos = pkmn::gameboy::locate_screen(&img_screen);
    let Some(gameboy_pos) = gameboy_pos else {
            return Err("No GameBoy screen was found!".to_string());
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
        return Err("Not showing summary screen 1 nor 2!".to_string());
    }

    if is_summary_screen_1 {
        let content = stats_screen_1_layout.read_content(&img_gameboy, &symbol_bitmaps);
        let Ok(content) = content else {
                return Err("Could not read summary screen 1 content!".to_string());
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

        return Ok(result);
    }

    if is_summary_screen_2 {
        let content = stats_screen_2_layout.read_content(&img_gameboy, &symbol_bitmaps);
        let Ok(content) = content else {
            return Err("Could not read summary screen 2 content!".to_string());
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

        let mut text_result = String::with_capacity(256);

        text_result.push_str(&format!("{} learnset:\n", learnset.pokemon));
        text_result.push_str(&format!(
            "{}\n",
            pkmn::learnset::get_pretty_learnset_table(learnset).unwrap()
        ));

        text_result.push_str(&"Evo chain(s):\n");
        println!("Evo chains:\n");
        for chain in evo_chains {
            text_result.push_str(&format!("{}\n", chain.replace(">", "   >   ")));
        }

        text_result.push_str(&"\n");
        for learnset in &evo_chain_learnsets {
            text_result.push_str(&format!("{} learnset:\n", learnset.pokemon));
            text_result.push_str(&format!(
                "{}\n",
                pkmn::learnset::get_pretty_learnset_table(learnset).unwrap()
            ));
        }

        return Ok(text_result);
    }

    return Err("Error in scanning logic. Went down logic path it should not have".to_string());
}
