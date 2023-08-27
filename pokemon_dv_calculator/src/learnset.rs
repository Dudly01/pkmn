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
    let entries: Vec<Learnset> =
        serde_json::from_str(&LEARNSET_JSON).expect("Failed to parse JSON");
    entries
}
