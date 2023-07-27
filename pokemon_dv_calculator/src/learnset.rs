use serde;
use serde_json;

#[derive(Debug, serde::Deserialize)]
pub struct Learnset {
    pub ndex: String,
    pub pokemon: String,
    pub by_leveling_up: Vec<Vec<String>>,
}

/// Returns the learnset for each pokemon.
pub fn load_learnsets() -> Vec<Learnset> {
    const LEARNSET_JSON: &str = include_str!("../data/learnset.json");

    // Deserialize the JSON data into a Vec<Entry>
    let entries: Vec<Learnset> = serde_json::from_str(&LEARNSET_JSON).expect("Failed to parse JSON");
    entries
}

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset is the same among game versions.
fn pretty_learnset_table(learnset: &Learnset) -> Result<String, String> {
    for row in &learnset.by_leveling_up {
        if row.len() != 6 {
            return Err("Found row with not exactly 6 elements".to_owned());
        }
    }

    let mut result = String::with_capacity(256);

    let header = &learnset.by_leveling_up[0];
    result.push_str(&format!(
        "{:^5} {:^15} {:^7} {:^5} {:^8} {:^3}\n",
        header[0], header[1], header[2], header[3], header[4], header[5]
    ));

    for row in learnset.by_leveling_up.iter().skip(1) {
        result.push_str(&format!(
            "{:<5} {:<15} {:^7} {:<5} {:<8} {:<3}\n",
            row[0], row[1], row[2], row[3], row[4], row[5]
        ));
    }

    Ok(result)
}

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset differs among game versions.
fn pretty_diff_learnset_table(learnset: &Learnset) -> Result<String, String> {
    for row in &learnset.by_leveling_up {
        if row.len() != 7 {
            return Err("Found row with not exactly 7 elements".to_owned());
        }
    }

    let mut result = String::with_capacity(256);

    let header = &learnset.by_leveling_up[0];
    result.push_str(&format!(
        "{:^3} {:^3} {:^15} {:^7} {:^5} {:^8} {:^3}\n",
        header[0], header[1], header[2], header[3], header[4], header[5], header[6]
    ));

    for row in learnset.by_leveling_up.iter().skip(1) {
        result.push_str(&format!(
            "{:<3} {:<3} {:<15} {:^7} {:<5} {:<8} {:<3}\n",
            row[0], row[1], row[2], row[3], row[4], row[5], row[6]
        ));
    }

    Ok(result)
}

/// Returns the string with the formatted "By leveling up" learnset.
pub fn get_pretty_learnset_table(entry: &Learnset) -> Result<String, String> {
    let same_learnset = entry.by_leveling_up[0].len() == 6;
    let result = match same_learnset {
        true => pretty_learnset_table(entry),
        false => pretty_diff_learnset_table(entry),
    };

    result
}
