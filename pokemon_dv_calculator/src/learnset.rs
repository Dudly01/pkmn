use serde;
use serde_json;

#[derive(Debug, serde::Deserialize)]
pub struct Entry {
    pub ndex: String,
    pub pokemon: String,
    pub by_leveling_up: Vec<Vec<String>>,
}

/// Returns the learnset for each pokemon.
pub fn load_learnsets() -> Vec<Entry> {
    const LEARNSET_JSON: &str = include_str!("../data/learnset.json");

    // Deserialize the JSON data into a Vec<Entry>
    let entries: Vec<Entry> = serde_json::from_str(&LEARNSET_JSON).expect("Failed to parse JSON");
    entries
}

pub fn print_learnsets(entries: &Vec<Entry>) {
    for entry in entries {
        println!("Pokemon: {}", entry.pokemon);
        println!("NDex: {}", entry.ndex);

        let same_learnset = entry.by_leveling_up[0].len() == 6;
        match same_learnset {
            true => {
                for row in &entry.by_leveling_up {
                    println!(
                        "{}, {}, {}, {}, {}, {}",
                        row[0], row[1], row[2], row[3], row[4], row[5]
                    )
                }
            }
            false => {
                for row in &entry.by_leveling_up {
                    println!(
                        "{}, {}, {}, {}, {}, {}, {}",
                        row[0], row[1], row[2], row[3], row[4], row[5], row[6]
                    )
                }
            }
        }
        println!("-------------------");
    }
}
