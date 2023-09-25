use crate as pkmn;
use crate::moves::Moves;
use crate::stats::{DvRange, StatVariation};
use image::imageops::invert;
use image::DynamicImage;
use imageproc::contrast::threshold_mut;
use pkmn::learnset::Learnset;

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset is the same among game versions.
fn fmt_shared_learnset_table(learnset: &Learnset, moves: &Moves) -> Result<String, String> {
    let mut t = String::with_capacity(256);

    // Pokemon
    t.push_str(&format!(
        "#{} {} learnset:\n",
        learnset.ndex, learnset.pokemon
    ));

    // Header
    t.push_str(&format!(
        "{:<3}  {}\n",
        "Lvl",
        pkmn::moves::fmt_move_header()
    ));

    // Moves
    for row in learnset.by_leveling_up.iter().skip(1) {
        let move_name = &row[1];
        let move_ = moves.get(move_name);

        let rt = format!("{:<3}  {}\n", row[0], pkmn::moves::fmt_move(move_));
        t.push_str(&rt);
    }

    Ok(t)
}

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset differs among game versions.
fn fmt_divering_learnset_table(learnset: &Learnset, moves: &Moves) -> Result<String, String> {
    let mut t = String::with_capacity(256);

    // Pokemon
    t.push_str(&format!(
        "#{} {} learnset:\n",
        learnset.ndex, learnset.pokemon
    ));

    // Header
    t.push_str(&format!(
        "{:<3}  {:<3}  {}\n",
        "RB",
        "Y",
        pkmn::moves::fmt_move_header()
    ));

    // Moves
    for row in learnset.by_leveling_up.iter().skip(1) {
        let move_name = &row[1];
        let move_ = moves.get(move_name);

        let rt = format!(
            "{:<3}  {:<3}  {}\n",
            row[0],
            row[1],
            pkmn::moves::fmt_move(move_)
        );

        t.push_str(&rt);
    }

    Ok(t)
}

/// Returns the string with the formatted "By leveling up" learnset.
pub fn fmt_learnset(learnset: &Learnset, moves: &Moves) -> Result<String, String> {
    let level_up_table = &learnset.by_leveling_up;
    let col_count = level_up_table[0].len();
    for row in level_up_table {
        if row.len() != col_count {
            return Err(format!("Mismatching column count for {}", learnset.pokemon));
        }
    }

    let t = match col_count {
        2 => fmt_shared_learnset_table(learnset, moves),
        3 => fmt_divering_learnset_table(learnset, moves),
        _ => Err(format!(
            "Expected column count of 2 or 3, got {}",
            col_count
        )),
    };

    t
}

/// Scans the image and returns the printable text.
/// The summary screen 1 is for printing the stat DVs.
/// The summary screen 2 us for printing the learnset and evolution chain.
pub fn scan_img(img_screen: DynamicImage) -> Result<String, String> {
    // Init data
    let chars = pkmn::char::Charset::new();
    let pokedex = pkmn::pokemon::Pokedex::new();
    let pkmn_learnsets = pkmn::learnset::Learnsets::new();
    let pkmn_evo_chains = pkmn::evos::load_evos();
    let pkmn_moves = pkmn::moves::Moves::new();

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

    let mut img_gameboy = img_gameboy.to_luma8();
    threshold_mut(&mut img_gameboy, 200);
    invert(&mut img_gameboy);

    let is_summary_screen_1 = stats_screen_1_layout.verify_layout(&img_gameboy, &chars);
    let is_summary_screen_2 = stats_screen_2_layout.verify_layout(&img_gameboy, &chars);

    if !is_summary_screen_1 && !is_summary_screen_2 {
        return Err("Not showing summary screen 1 nor 2!".to_string());
    }

    if is_summary_screen_1 {
        let content = stats_screen_1_layout
            .read_fields(&img_gameboy, &chars)
            .expect("Failed to read Summary 1");

        let ndex: usize = content.pkmn_no as usize;
        let pokemon = &pokedex[ndex - 1];

        let var_hp = StatVariation::init(&content.level, &pokemon.hp, &0, &true);
        let var_attack = StatVariation::init(&content.level, &pokemon.attack, &0, &false);
        let var_defense = StatVariation::init(&content.level, &pokemon.defense, &0, &false);
        let var_speed = StatVariation::init(&content.level, &pokemon.speed, &0, &false);
        let var_special = StatVariation::init(&content.level, &pokemon.special, &0, &false);

        let range_hp = DvRange::init(&content.hp, &var_hp).unwrap();
        let range_attack = DvRange::init(&content.attack, &var_attack).unwrap();
        let range_defense = DvRange::init(&content.defense, &var_defense).unwrap();
        let range_speed = DvRange::init(&content.speed, &var_speed).unwrap();
        let range_special = DvRange::init(&content.special, &var_special).unwrap();

        let mut t = String::new();

        t.push_str(&format!(
            "#{} {} :L{}\n\n",
            pokemon.ndex, pokemon.name, content.level
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}  {}\n",
            "Stat", "Base", "Value", "DV [min-max]"
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "HP", pokemon.hp, content.hp, range_hp.min, range_hp.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "ATT", pokemon.attack, content.attack, range_attack.min, range_attack.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "DEF", pokemon.defense, content.defense, range_defense.min, range_defense.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "SPD", pokemon.speed, content.speed, range_speed.min, range_speed.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "SPC", pokemon.special, content.special, range_special.min, range_special.max
        ));

        t.push_str("\n\nDV-Value Table\n");

        t.push_str(&format!(
            "{:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}\n",
            "DV", "HP", "ATT", "DEF", "SPD", "SPC",
        ));

        // Returns the notification char upon equality, space otherwise.
        let notif_char = |eq: bool| -> char {
            if eq {
                '-'
            } else {
                ' '
            }
        };

        let notif_hp: [char; 16] = std::array::from_fn(|i| notif_char(var_hp[i] == content.hp));
        let notif_attack: [char; 16] =
            std::array::from_fn(|i| notif_char(var_attack[i] == content.attack));
        let notif_defense: [char; 16] =
            std::array::from_fn(|i| notif_char(var_defense[i] == content.defense));
        let notif_speed: [char; 16] =
            std::array::from_fn(|i| notif_char(var_speed[i] == content.speed));
        let notif_special: [char; 16] =
            std::array::from_fn(|i| notif_char(var_special[i] == content.special));

        for i in 0..16 {
            t.push_str(&format!(
                "{:>3} {:>3}{} {:>3}{} {:>3}{} {:>3}{} {:>3}{}\n",
                i,
                var_hp[i],
                notif_hp[i],
                var_attack[i],
                notif_attack[i],
                var_defense[i],
                notif_defense[i],
                var_speed[i],
                notif_speed[i],
                var_special[i],
                notif_special[i],
            ));
        }

        return Ok(t);
    }

    if is_summary_screen_2 {
        let content = stats_screen_2_layout.read_fields(&img_gameboy, &chars);
        let Ok(content) = content else {
            return Err("Could not read summary screen 2 content!".to_string());
        };

        let ndex: usize = content.pkmn_no.parse().unwrap();

        let pkmn_name = &pokedex[ndex - 1].name;
        let evo_chains: Vec<_> = pkmn_evo_chains
            .iter()
            .filter(|x| x.contains(pkmn_name))
            .collect();

        let mut pkmn_names: Vec<&str> = Vec::new();
        for chain in &evo_chains {
            let pkmn = chain.split("->").step_by(2);
            for name in pkmn {
                if !pkmn_names.contains(&name) {
                    pkmn_names.push(name);
                }
            }
        }

        let evo_chain_learnsets = pkmn_names
            .iter()
            .map(|name| pokedex.iter().find(|r| r.name == *name).unwrap())
            .map(|r| r.ndex)
            .map(|ndex| &pkmn_learnsets[ndex as usize - 1])
            .collect::<Vec<&pkmn::learnset::Learnset>>();

        let mut text_result = String::with_capacity(256);

        text_result.push_str(&format!("{}\n", &pkmn::moves::fmt_move_header()));
        for attack_name in [
            &content.attack_1,
            &content.attack_2,
            &content.attack_3,
            &content.attack_4,
        ] {
            match attack_name.as_str() {
                "-" => text_result.push_str("-\n"),
                _ => {
                    let move_ = pkmn_moves.get(&attack_name);
                    text_result.push_str(&format!("{}\n", pkmn::moves::fmt_move(move_)));
                }
            }
        }

        text_result.push_str(&"\nEvo chain(s):\n");
        println!("Evo chains:\n");
        for chain in evo_chains {
            text_result.push_str(&format!("{}\n", chain.replace("->", "   ->   ")));
        }

        text_result.push_str(&"\n");
        for learnset in &evo_chain_learnsets {
            text_result.push_str(&format!(
                "{}\n",
                fmt_learnset(learnset, &pkmn_moves).unwrap()
            ));
        }

        return Ok(text_result);
    }

    return Err("Error in scanning logic. Went down logic path it should not have".to_string());
}