use crate as pkmn;
use crate::moves::{GscMoves, Moves};
use crate::roi::Roi;
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

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset is the same among game versions.
fn fmt_gsc_shared_learnset_table(learnset: &Learnset, moves: &GscMoves) -> Result<String, String> {
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
fn fmt_gsc_divering_learnset_table(
    learnset: &Learnset,
    moves: &GscMoves,
) -> Result<String, String> {
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
pub fn fmt_gsc_learnset(learnset: &Learnset, moves: &GscMoves) -> Result<String, String> {
    let level_up_table = &learnset.by_leveling_up;
    let col_count = level_up_table[0].len();
    for row in level_up_table {
        if row.len() != col_count {
            return Err(format!("Mismatching column count for {}", learnset.pokemon));
        }
    }

    let t = match col_count {
        2 => fmt_gsc_shared_learnset_table(learnset, moves),
        3 => fmt_gsc_divering_learnset_table(learnset, moves),
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

    let rby_pokedex = pkmn::pokemon::Pokedex::new();
    let rby_learnsets = pkmn::learnset::Learnsets::new();
    let rby_evo_chains = pkmn::evos::load_evos();
    let rby_moves = pkmn::moves::Moves::new();

    let gsc_pokedex = pkmn::pokemon::GscPokedex::new();
    let gsc_learnsets = pkmn::learnset::GscLearnsets::new();
    let gsc_evo_chains = pkmn::evos::load_gsc_evos();
    let gsc_moves = pkmn::moves::GscMoves::new();

    let rby_summary_1 = pkmn::gameboy::RbySummary1::new();
    let rby_summary_2 = pkmn::gameboy::RbySummary2::new();

    let gsc_summary_1 = pkmn::gameboy::GscSummary1::new();
    let gsc_summary_2 = pkmn::gameboy::GscSummary2::new();
    let gsc_summary_3 = pkmn::gameboy::GscSummary3::new();

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
            rby_summary_1.width as u32,
            rby_summary_1.height as u32,
            image::imageops::FilterType::Nearest,
        );

    let mut img_gameboy = img_gameboy.to_luma8();
    let threshold_val = 140; // Anything in [30, 170]
    threshold_mut(&mut img_gameboy, threshold_val);
    invert(&mut img_gameboy);

    let is_rby_summary_1 = rby_summary_1.verify_layout(&img_gameboy, &chars);
    let is_rby_summary_2 = rby_summary_2.verify_layout(&img_gameboy, &chars);
    let is_gsc_summary_1 = gsc_summary_1.verify_layout(&img_gameboy, &chars);
    let is_gsc_summary_2 = gsc_summary_2.verify_layout(&img_gameboy, &chars);
    let is_gsc_summary_3 = gsc_summary_3.verify_layout(&img_gameboy, &chars);

    if is_rby_summary_1 {
        let content = rby_summary_1
            .read_fields(&img_gameboy, &chars)
            .expect("Failed to read Summary 1");

        let ndex: usize = content.pkmn_no as usize;
        let pokemon = &rby_pokedex[ndex - 1];

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

        t.push_str("\nDV-Value Table\n");

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

    if is_rby_summary_2 {
        let content = rby_summary_2.read_fields(&img_gameboy, &chars);
        let Ok(content) = content else {
            return Err("Could not read summary screen 2 content!".to_string());
        };

        let ndex: usize = content.pkmn_no.parse().unwrap();

        let pkmn_name = &rby_pokedex[ndex - 1].name;
        let evo_chains: Vec<_> = rby_evo_chains
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
            .map(|name| rby_pokedex.iter().find(|r| r.name == *name).unwrap())
            .map(|r| r.ndex)
            .map(|ndex| &rby_learnsets[ndex as usize - 1])
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
                    let move_ = rby_moves.get(&attack_name);
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
                fmt_learnset(learnset, &rby_moves).unwrap()
            ));
        }

        return Ok(text_result);
    }

    if is_gsc_summary_1 {
        let ndex = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_1.ndex, &chars);

        let level = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_1.level, &chars);

        let hp = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_1.hp, &chars);

        let mut t: String = String::new();
        #[cfg(debug_assertions)]
        {
            t.push_str("GSC Summary 1\n");
            t.push_str(&format!("Ndex: {ndex:?}\n"));
            t.push_str(&format!("Level: {level:?}\n"));
            t.push_str(&format!("Hp: {hp:?}\n"));
        }

        let ndex: usize = ndex
            .expect("Failed to read ndex")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse ndex to an integer");

        let level: i32 = level
            .expect("Failed to read level")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse level to an integer");

        let hp: i32 = hp
            .expect("Failed to read hp")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse hp to an integer");

        let pokemon = &gsc_pokedex[ndex - 1];

        let var_hp = StatVariation::init(&level, &pokemon.hp, &0, &true);
        let range_hp = DvRange::init(&hp, &var_hp).unwrap();

        t.push_str(&format!(
            "#{} {} :L{}\n\n",
            pokemon.ndex, pokemon.name, level
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}  {}\n",
            "Stat", "Base", "Value", "DV [min-max]"
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "HP", pokemon.hp, hp, range_hp.min, range_hp.max
        ));

        // Returns the notification char upon equality, space otherwise.
        let notif_char = |eq: bool| -> char {
            if eq {
                '-'
            } else {
                ' '
            }
        };

        let notif_hp: [char; 16] = std::array::from_fn(|i| notif_char(var_hp[i] == hp));
        t.push_str("\nDV-Value Table\n");
        t.push_str(&format!("{:>3}  {:>3}\n", "DV", "HP",));
        for i in 0..16 {
            t.push_str(&format!("{:>3} {:>3}{}\n", i, var_hp[i], notif_hp[i],));
        }

        return Ok(t);
    }

    if is_gsc_summary_2 {
        let ndex = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.ndex, &chars);

        let level = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.level, &chars);

        let attack_1 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.attack_1, &chars);

        let attack_2 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.attack_2, &chars);

        let attack_3 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.attack_3, &chars);

        let attack_4 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.attack_4, &chars);

        let mut t = String::new();

        #[cfg(debug_assertions)]
        {
            t.push_str("GSC Summary 2\n");
            t.push_str(&format!("Ndex: {ndex:?}\n"));
            t.push_str(&format!("Level: {level:?}\n"));
            t.push_str(&format!("Attack 1: {attack_1:?}\n"));
            t.push_str(&format!("Attack 2: {attack_2:?}\n"));
            t.push_str(&format!("Attack 3: {attack_3:?}\n"));
            t.push_str(&format!("Attack 4: {attack_4:?}\n"));
        }

        let ndex: usize = ndex
            .expect("Failed to read ndex")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse ndex to an integer");

        let level: i32 = level
            .expect("Failed to read level")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse level to an integer");

        let attack_1 = attack_1
            .expect("Failed to read attack_1")
            .trim()
            .to_string();

        let attack_2 = attack_2
            .expect("Failed to read attack_2")
            .trim()
            .to_string();

        let attack_3 = attack_3
            .expect("Failed to read attack_3")
            .trim()
            .to_string();

        let attack_4 = attack_4
            .expect("Failed to read attack_4")
            .trim()
            .to_string();

        let pokemon = &gsc_pokedex[ndex - 1];
        t.push_str(&format!(
            "#{} {} :L{}\n\n",
            pokemon.ndex, pokemon.name, level
        ));

        t.push_str(&format!("{}\n", &pkmn::moves::fmt_move_header()));
        for attack_name in [&attack_1, &attack_2, &attack_3, &attack_4] {
            match attack_name.as_str() {
                "-" => t.push_str("-\n"),
                _ => {
                    let move_ = gsc_moves.get(&attack_name);
                    t.push_str(&format!("{}\n", pkmn::moves::fmt_move(move_)));
                }
            }
        }

        let pokemon = &gsc_pokedex[ndex - 1];
        let evo_chains: Vec<_> = gsc_evo_chains
            .iter()
            .filter(|x| x.contains(&pokemon.name))
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
            .map(|name| gsc_pokedex.iter().find(|r| r.name == *name).unwrap())
            .map(|r| r.ndex)
            .map(|ndex| &gsc_learnsets[ndex as usize - 1])
            .collect::<Vec<&pkmn::learnset::Learnset>>();

        t.push_str(&"\nEvo chain(s):\n");
        for chain in evo_chains {
            t.push_str(&format!("{}\n", chain.replace("->", "   ->   ")));
        }

        t.push_str(&"\n");
        for learnset in &evo_chain_learnsets {
            t.push_str(&format!(
                "{}\n",
                fmt_gsc_learnset(learnset, &gsc_moves).unwrap()
            ));
        }

        return Ok(t);
    }

    if is_gsc_summary_3 {
        let ndex = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_3.ndex, &chars);

        let level = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_3.level, &chars);

        let attack = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_3.attack, &chars);

        let defense = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_3.defense, &chars);

        let spc_attack = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_3.spc_attack, &chars);

        let spc_defense = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_3.spc_defense, &chars);

        let speed = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_3.speed, &chars);

        let mut t = String::new();

        #[cfg(debug_assertions)]
        {
            t.push_str("GSC Summary 3\n");
            t.push_str(&format!("Ndex: {ndex:?}\n"));
            t.push_str(&format!("Level: {level:?}\n"));
            t.push_str(&format!("Attack: {attack:?}\n"));
            t.push_str(&format!("Defense: {defense:?}\n"));
            t.push_str(&format!("Spc. Attack: {spc_attack:?}\n"));
            t.push_str(&format!("Spc. Defense: {spc_defense:?}\n"));
            t.push_str(&format!("Speed: {speed:?}\n"));
        }

        let ndex: usize = ndex
            .expect("Failed to read ndex")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse ndex to an integer");

        let level: i32 = level
            .expect("Failed to read level")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse level to an integer");

        let attack: i32 = attack
            .expect("Failed to read hp")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse attack to an integer");

        let defense: i32 = defense
            .expect("Failed to read hp")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse defense to an integer");

        let spc_attack: i32 = spc_attack
            .expect("Failed to read hp")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse spc_attack to an integer");

        let spc_defense: i32 = spc_defense
            .expect("Failed to read hp")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse spc_defense to an integer");

        let speed: i32 = speed
            .expect("Failed to read hp")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse speed to an integer");

        let pokemon = &gsc_pokedex[ndex - 1];

        let var_attack = StatVariation::init(&level, &pokemon.attack, &0, &false);
        let var_defense = StatVariation::init(&level, &pokemon.defense, &0, &false);
        let var_spc_attack = StatVariation::init(&level, &pokemon.special_attack, &0, &false);
        let var_spc_defense = StatVariation::init(&level, &pokemon.special_defense, &0, &false);
        let var_speed = StatVariation::init(&level, &pokemon.speed, &0, &false);

        let range_attack = DvRange::init(&attack, &var_attack).unwrap();
        let range_defense = DvRange::init(&defense, &var_defense).unwrap();
        let range_spc_attack = DvRange::init(&spc_attack, &var_spc_attack).unwrap();
        let range_spc_defense = DvRange::init(&spc_defense, &var_spc_defense).unwrap();
        let range_speed = DvRange::init(&speed, &var_speed).unwrap();

        t.push_str(&format!(
            "#{} {} :L{}\n\n",
            pokemon.ndex, pokemon.name, level
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}  {}\n",
            "Stat", "Base", "Value", "DV [min-max]"
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "ATT", pokemon.attack, attack, range_attack.min, range_attack.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "DEF", pokemon.defense, defense, range_defense.min, range_defense.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "SPA", pokemon.special_attack, spc_attack, range_spc_attack.min, range_spc_attack.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "SPD",
            pokemon.special_attack,
            spc_defense,
            range_spc_defense.min,
            range_spc_defense.max
        ));

        t.push_str(&format!(
            "{:>4}  {:>4}  {:>4}    {}-{}\n",
            "SPE", pokemon.speed, speed, range_speed.min, range_speed.max
        ));

        t.push_str("\nDV-Value Table\n");

        // Returns the notification char upon equality, space otherwise.
        let notif_char = |eq: bool| -> char {
            if eq {
                '-'
            } else {
                ' '
            }
        };

        let notif_attack: [char; 16] = std::array::from_fn(|i| notif_char(var_attack[i] == attack));
        let notif_defense: [char; 16] =
            std::array::from_fn(|i| notif_char(var_defense[i] == defense));
        let notif_spc_attack: [char; 16] =
            std::array::from_fn(|i| notif_char(var_spc_attack[i] == spc_attack));
        let notif_spc_defense: [char; 16] =
            std::array::from_fn(|i| notif_char(var_spc_defense[i] == spc_defense));
        let notif_speed: [char; 16] = std::array::from_fn(|i| notif_char(var_speed[i] == speed));

        t.push_str(&format!(
            "{:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}\n",
            "DV", "ATT", "DEF", "SPA", "SPD", "SPE",
        ));

        for i in 0..16 {
            t.push_str(&format!(
                "{:>3} {:>3}{} {:>3}{} {:>3}{} {:>3}{} {:>3}{}\n",
                i,
                var_attack[i],
                notif_attack[i],
                var_defense[i],
                notif_defense[i],
                var_spc_attack[i],
                notif_spc_attack[i],
                var_spc_defense[i],
                notif_spc_defense[i],
                var_speed[i],
                notif_speed[i],
            ));
        }

        return Ok(t);
    }

    return Err("Screen found but not recognised.".to_string());
}
