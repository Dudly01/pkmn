#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub ndex: i32,
    pub pokemon: String,
    pub types: String,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
    pub total: i32,
}

/// Loads the base stats from the CSV file.
pub fn load_base_stats() -> Vec<Record> {
    let mut records: Vec<Record> = Vec::with_capacity(151);

    let csv_path = "data/base_stats.csv";
    let mut csv_reader = csv::Reader::from_path(csv_path).expect("could not load CSV file.");

    for result in csv_reader.deserialize() {
        let record: Record = result.unwrap();
        records.push(record);
    }

    records
}

/// Returns a vector of stats, where the index corresponds to the DV value.
/// The HP and the other stats differ slightly in calculation.
/// Hence the is_hp boolean argument.
pub fn get_dv_stat_pairs(level: i32, base: i32, exp: i32, is_hp: bool) -> Vec<i32> {
    let offset = if is_hp { level + 10 } else { 5 };

    let mut result = Vec::with_capacity(16);
    let effort_gain = ((exp - 1) as f32).sqrt() + 1.0 / 4.0;
    let effort_gain = effort_gain as i32;

    for dv in 0..16 {
        let val = (((base + dv) * 2 + effort_gain) * level) as f32 / 100.0;
        let val = val as i32 + offset;
        result.push(val);
    }

    result
}

pub fn print_dv_table(
    hp: &Vec<i32>,
    attack: &Vec<i32>,
    defense: &Vec<i32>,
    speed: &Vec<i32>,
    special: &Vec<i32>,
) {
    println!(
        "{: >5}{: >5}{: >5}{: >5}{: >5}{: >5}",
        "DV", "HP", "ATT", "DEF", "SPD", "SPC"
    );

    for i in 0..16 {
        let curr_hp = hp[i];
        let curr_attack = attack[i];
        let curr_defense = defense[i];
        let curr_speed = speed[i];
        let curr_special = special[i];

        println!(
            "{: >5}{: >5}{: >5}{: >5}{: >5}{: >5}",
            i, curr_hp, curr_attack, curr_defense, curr_speed, curr_special
        );
    }
}

pub fn find_dv_range(
    stat_val: &i32,
    dv_stat_pairs: &Vec<i32>,
) -> Result<(usize, usize), &'static str> {
    if dv_stat_pairs.len() != 16 {
        return Err("DV-stat pairs does not contain exactly 16 elements.");
    }

    let mut start = -1;
    let mut end = -1;

    for (i, val) in dv_stat_pairs.iter().enumerate() {
        if *val == *stat_val as i32 {
            start = i as i32;
            break;
        }
    }

    if start == -1 {
        return Err("DV-stat pairs do not contain desired stat value.");
    }

    for (i, val) in dv_stat_pairs.iter().enumerate().rev() {
        if *val == *stat_val as i32 {
            end = i as i32 + 1;
            break;
        }
    }

    Ok((start as usize, end as usize))
}
