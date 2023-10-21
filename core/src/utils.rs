use crate as pkmn;
use crate::char::Charset;
use crate::fmt;
use crate::gameboy::{GscSummary1, GscSummary2, GscSummary3, RbySummary1, RbySummary2};
use crate::items::GscItems;
use crate::learnset::{GscLearnsets, RbyLearnsets};
use crate::moves::{GscMoves, Moves};
use crate::pokemon::{GscPokedex, RbyPokedex};
use crate::stats::{DvRange, StatVariation};
use image::imageops::invert;
use image::{DynamicImage, GrayImage};
use imageproc::contrast::threshold_mut;

fn scan_rby_summary_1(
    img_gameboy: &GrayImage,
    rby_summary_1: &RbySummary1,
    chars: &Charset,
    rby_pokedex: &RbyPokedex,
) -> Result<String, String> {
    let content = rby_summary_1
        .read_fields(&img_gameboy, &chars)
        .map_err(|err| format!("could not read RBY summary 1: {err}"))?;

    let ndex: usize = content.pkmn_no as usize;
    let pokemon = rby_pokedex
        .get_ndex(ndex)
        .ok_or(format!("could not find Pokemon with ndex '{ndex}'"))?;

    let var_hp = StatVariation::init(&content.level, &pokemon.hp, &0, &true);
    let var_attack = StatVariation::init(&content.level, &pokemon.attack, &0, &false);
    let var_defense = StatVariation::init(&content.level, &pokemon.defense, &0, &false);
    let var_speed = StatVariation::init(&content.level, &pokemon.speed, &0, &false);
    let var_special = StatVariation::init(&content.level, &pokemon.special, &0, &false);

    let range_hp = DvRange::init(&content.hp, &var_hp)
        .map_err(|err| format!("could not determine HP DV range: {err}"))?;
    let range_attack = DvRange::init(&content.attack, &var_attack)
        .map_err(|err| format!("could not determine Attack DV range: {err}"))?;
    let range_defense = DvRange::init(&content.defense, &var_defense)
        .map_err(|err| format!("could not determine Defense DV range: {err}"))?;
    let range_speed = DvRange::init(&content.speed, &var_speed)
        .map_err(|err| format!("could not determine Speed DV range: {err}"))?;
    let range_special = DvRange::init(&content.special, &var_special)
        .map_err(|err| format!("could not determine Special DV range: {err}"))?;

    let mut t = String::new();

    t.push_str(&format!(
        "#{} {} :L{}\n\n",
        pokemon.ndex, pokemon.name, content.level
    ));

    t.push_str(&fmt::fmt_stat_header());
    t.push_str(&fmt::fmt_stat_row(
        "HP",
        &pokemon.hp,
        &content.hp,
        &range_hp,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "ATT",
        &pokemon.attack,
        &content.attack,
        &range_attack,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "DEF",
        &pokemon.defense,
        &content.defense,
        &range_defense,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "SPD",
        &pokemon.speed,
        &content.speed,
        &range_speed,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "SPC",
        &pokemon.special,
        &content.special,
        &range_special,
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

fn scan_rby_summary_2(
    img_gameboy: &GrayImage,
    rby_summary_2: &RbySummary2,
    chars: &Charset,
    rby_pokedex: &RbyPokedex,
    rby_evo_chains: &Vec<String>,
    rby_learnsets: &RbyLearnsets,
    rby_moves: &Moves,
) -> Result<String, String> {
    let content = rby_summary_2.read_fields(&img_gameboy, &chars);
    let Ok(content) = content else {
            return Err("Could not read summary screen 2 content!".to_string());
        };

    let ndex: usize = content
        .pkmn_no
        .parse()
        .map_err(|_| format!("could not parse ndex '{}' into an integer", content.pkmn_no))?;

    let pkmn_name = rby_pokedex
        .get_ndex(ndex)
        .ok_or(&format!("could not find Pokemon at ndex '{ndex}'"))?
        .name
        .to_owned();

    let evo_chains: Vec<_> = rby_evo_chains
        .iter()
        .filter(|x| x.contains(&pkmn_name))
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
        .map(|&name| {
            rby_learnsets
                .get_pokemon(name)
                .ok_or(format!("no learnset found for Pokemon '{name}'"))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("could not collect learnsets for evo chain: {err}"))?;

    let mut text_result = String::with_capacity(256);

    text_result.push_str(&format!("{}\n", &fmt::fmt_move_header()));
    for move_name in [
        &content.move_1,
        &content.move_2,
        &content.move_3,
        &content.move_4,
    ] {
        match move_name.as_str() {
            "-" => text_result.push_str("-\n"),
            _ => {
                let move_ = rby_moves.get(&move_name);
                text_result.push_str(&format!("{}\n", fmt::fmt_move(move_)));
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
            fmt::fmt_learnset(learnset, &rby_moves).expect("could not format learnset")
        ));
    }

    return Ok(text_result);
}

fn scan_gsc_summary_1(
    img_gameboy: &GrayImage,
    gsc_summary_1: &GscSummary1,
    chars: &Charset,
    gsc_pokedex: &GscPokedex,
) -> Result<String, String> {
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

    let ndex = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_1.ndex, &chars)
        .map_err(|err| format!("could not read ndex: {err}"))?;
    let ndex = ndex
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("could not parse ndex '{ndex}' to an integer"))?;

    let level = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_1.level, &chars)
        .map_err(|err| format!("could not read level: {err}"))?;
    let level = level
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse level '{level}' to an integer"))?;

    let hp = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_1.hp, &chars)
        .map_err(|err| format!("could not read hp: {err}"))?;
    let hp = hp
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse hp '{hp}' to an integer"))?;

    let pokemon = gsc_pokedex
        .get_ndex(ndex)
        .ok_or(format!("could not find Pokemon at ndex '{ndex}'"))?;

    let var_hp = StatVariation::init(&level, &pokemon.hp, &0, &true);
    let range_hp = DvRange::init(&hp, &var_hp)
        .map_err(|err| format!("could not determine HP DV range: {err}"))?;

    t.push_str(&format!(
        "#{} {} :L{}\n\n",
        pokemon.ndex, pokemon.name, level
    ));

    t.push_str(&fmt::fmt_stat_header());
    t.push_str(&fmt::fmt_stat_row("HP", &pokemon.hp, &hp, &range_hp));

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

fn scan_gsc_summary_2(
    img_gameboy: &GrayImage,
    gsc_summary_2: &GscSummary2,
    chars: &Charset,
    gsc_pokedex: &GscPokedex,
    gsc_items: &GscItems,
    gsc_moves: &GscMoves,
    gsc_evo_chains: &Vec<String>,
    gsc_learnsets: &GscLearnsets,
) -> Result<String, String> {
    let ndex = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.ndex, &chars);

    let level = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.level, &chars);

    let item = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.item, &chars);

    let move_1 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.move_1, &chars);

    let move_2 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.move_2, &chars);

    let move_3 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.move_3, &chars);

    let move_4 = pkmn::ocr::read_field(&img_gameboy, &gsc_summary_2.move_4, &chars);

    let mut t = String::new();

    #[cfg(debug_assertions)]
    {
        t.push_str("GSC Summary 2\n");
        t.push_str(&format!("Ndex: {ndex:?}\n"));
        t.push_str(&format!("Level: {level:?}\n"));
        t.push_str(&format!("Item: {item:?}\n"));
        t.push_str(&format!("Move 1: {move_1:?}\n"));
        t.push_str(&format!("Move 2: {move_2:?}\n"));
        t.push_str(&format!("Move 3: {move_3:?}\n"));
        t.push_str(&format!("Move 4: {move_4:?}\n"));
    }

    let ndex = ndex.map_err(|err| format!("could not read ndex: {err}"))?;
    let ndex = ndex
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("could not parse ndex '{ndex}' to an integer"))?;

    let level = level.map_err(|err| format!("could not read level: {err}"))?;
    let level = level
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse level '{level}' to an integer"))?;

    let item_name = item
        .map_err(|err| format!("could not read item: {err}"))?
        .trim()
        .to_string();

    let move_1 = move_1
        .map_err(|err| format!("could not read move_1: {err}"))?
        .trim()
        .to_string();

    let move_2 = move_2
        .map_err(|err| format!("could not read move_2: {err}"))?
        .trim()
        .to_string();

    let move_3 = move_3
        .map_err(|err| format!("could not read move_3: {err}"))?
        .trim()
        .to_string();

    let move_4 = move_4
        .map_err(|err| format!("could not read move_4: {err}"))?
        .trim()
        .to_string();

    let pokemon = &gsc_pokedex
        .get_ndex(ndex)
        .ok_or(format!("could not find Pokemon at ndex '{ndex}'"))?;
    t.push_str(&format!(
        "#{} {} :L{}\n\n",
        pokemon.ndex, pokemon.name, level
    ));

    t.push_str(&format!("Item:\n"));
    let item = gsc_items.get(&item_name);
    match item {
        Some(item) => t.push_str(&format!("{:<12}  {}\n\n", item_name, item.description)),
        None => t.push_str(&format!("{:<12}  {}\n\n", item_name, "NO DATA")),
    }

    t.push_str(&format!("{}\n", &fmt::fmt_move_header()));
    for move_name in [&move_1, &move_2, &move_3, &move_4] {
        match move_name.as_str() {
            "-" => t.push_str("-\n"),
            _ => {
                let move_ = gsc_moves.get(&move_name);
                t.push_str(&format!("{}\n", fmt::fmt_move(move_)));
            }
        }
    }

    let pokemon = &gsc_pokedex
        .get_ndex(ndex)
        .ok_or(format!("could not find Pokemon at ndex '{ndex}'"))?;
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
        .map(|&name| {
            gsc_learnsets
                .get_pokemon(name)
                .ok_or(format!("no learnset found for Pokemon {name}"))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("could not collect learnsets for evo chain: {err}"))?;

    t.push_str(&"\nEvo chain(s):\n");
    for chain in evo_chains {
        t.push_str(&format!("{}\n", chain.replace("->", "   ->   ")));
    }

    t.push_str(&"\n");
    for learnset in &evo_chain_learnsets {
        t.push_str(&format!(
            "{}\n",
            fmt::fmt_gsc_learnset(learnset, &gsc_moves).expect("could not format learnset")
        ));
    }

    return Ok(t);
}

fn scan_gsc_summary_3(
    img_gameboy: &GrayImage,
    gsc_summary_3: &GscSummary3,
    chars: &Charset,
    gsc_pokedex: &GscPokedex,
) -> Result<String, String> {
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

    let ndex = ndex.map_err(|err| format!("could not read ndex: {err}"))?;
    let ndex = ndex
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("could not parse ndex '{ndex}' to an integer"))?;

    let level = level.map_err(|err| format!("could not read level: {err}"))?;
    let level = level
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse level '{level}' to an integer"))?;

    let attack = attack.map_err(|err| format!("could not read attack: {err}"))?;
    let attack = attack
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse attack '{attack}' to an integer"))?;

    let defense = defense.map_err(|err| format!("could not read defense: {err}"))?;
    let defense = defense
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse defense '{defense}' to an integer"))?;

    let spc_attack = spc_attack.map_err(|err| format!("could not read spc_attack: {err}"))?;
    let spc_attack = spc_attack
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse spc_attack '{spc_attack}' to an integer"))?;

    let spc_defense = spc_defense.map_err(|err| format!("could not read spc_defense: {err}"))?;
    let spc_defense = spc_defense
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse spc_defense '{spc_defense}' to an integer"))?;

    let speed = speed.map_err(|err| format!("could not read speed: {err}"))?;
    let speed = speed
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("could not parse speed '{speed}' to an integer"))?;

    let pokemon = &gsc_pokedex
        .get_ndex(ndex)
        .ok_or(format!("could not find Pokemon at ndex '{ndex}'"))?;

    let var_attack = StatVariation::init(&level, &pokemon.attack, &0, &false);
    let var_defense = StatVariation::init(&level, &pokemon.defense, &0, &false);
    let var_spc_attack = StatVariation::init(&level, &pokemon.special_attack, &0, &false);
    let var_spc_defense = StatVariation::init(&level, &pokemon.special_defense, &0, &false);
    let var_speed = StatVariation::init(&level, &pokemon.speed, &0, &false);

    let range_attack = DvRange::init(&attack, &var_attack)
        .map_err(|err| format!("could not determine Attack DV range: {err}"))?;
    let range_defense = DvRange::init(&defense, &var_defense)
        .map_err(|err| format!("could not determine Defense DV range: {err}"))?;
    let range_spc_attack = DvRange::init(&spc_attack, &var_spc_attack)
        .map_err(|err| format!("could not determine Spc. Attack DV range: {err}"))?;
    let range_spc_defense = DvRange::init(&spc_defense, &var_spc_defense)
        .map_err(|err| format!("could not determine Spc. Defense DV range: {err}"))?;
    let range_speed = DvRange::init(&speed, &var_speed)
        .map_err(|err| format!("could not determine Speed DV range: {err}"))?;

    t.push_str(&format!(
        "#{} {} :L{}\n\n",
        pokemon.ndex, pokemon.name, level
    ));

    t.push_str(&fmt::fmt_stat_header());
    t.push_str(&fmt::fmt_stat_row(
        "ATT",
        &pokemon.attack,
        &attack,
        &range_attack,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "DEF",
        &pokemon.defense,
        &defense,
        &range_defense,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "SPA",
        &pokemon.special_attack,
        &spc_attack,
        &range_spc_attack,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "SPD",
        &pokemon.special_defense,
        &spc_defense,
        &range_spc_defense,
    ));
    t.push_str(&fmt::fmt_stat_row(
        "SPE",
        &pokemon.speed,
        &speed,
        &range_speed,
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
    let notif_defense: [char; 16] = std::array::from_fn(|i| notif_char(var_defense[i] == defense));
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

/// Scans the image and returns the printable text.
/// The summary screen 1 is for printing the stat DVs.
/// The summary screen 2 us for printing the learnset and evolution chain.
pub fn scan_img(img_screen: DynamicImage) -> Result<String, String> {
    // Init data
    let chars = pkmn::char::Charset::new();

    let rby_pokedex = pkmn::pokemon::RbyPokedex::new();
    let rby_learnsets = pkmn::learnset::RbyLearnsets::new();
    let rby_evo_chains = pkmn::evos::load_evos();
    let rby_moves = pkmn::moves::Moves::new();

    let gsc_pokedex = pkmn::pokemon::GscPokedex::new();
    let gsc_learnsets = pkmn::learnset::GscLearnsets::new();
    let gsc_evo_chains = pkmn::evos::load_gsc_evos();
    let gsc_moves = pkmn::moves::GscMoves::new();
    let gsc_items = pkmn::items::GscItems::new();

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
        let result = scan_rby_summary_1(&img_gameboy, &rby_summary_1, &chars, &rby_pokedex);
        return result;
    }

    if is_rby_summary_2 {
        let result = scan_rby_summary_2(
            &img_gameboy,
            &rby_summary_2,
            &chars,
            &rby_pokedex,
            &rby_evo_chains,
            &rby_learnsets,
            &rby_moves,
        );
        return result;
    }

    if is_gsc_summary_1 {
        let result = scan_gsc_summary_1(&img_gameboy, &gsc_summary_1, &chars, &gsc_pokedex);
        return result;
    }

    if is_gsc_summary_2 {
        let result = scan_gsc_summary_2(
            &img_gameboy,
            &gsc_summary_2,
            &chars,
            &gsc_pokedex,
            &gsc_items,
            &gsc_moves,
            &gsc_evo_chains,
            &gsc_learnsets,
        );
        return result;
    }

    if is_gsc_summary_3 {
        let result = scan_gsc_summary_3(&img_gameboy, &gsc_summary_3, &chars, &gsc_pokedex);
        return result;
    }

    return Err("Screen found but not recognised.".to_string());
}
