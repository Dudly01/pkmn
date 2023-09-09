use serde;
use serde_json;
use std::ops::Deref;

#[derive(Debug, serde::Deserialize)]
pub struct Learnset {
    pub ndex: String,
    pub pokemon: String,
    pub by_leveling_up: Vec<Vec<String>>,
}

pub struct Learnsets {
    sets: Vec<Learnset>,
}

impl Learnsets {
    pub fn new() -> Learnsets {
        const LEARNSET_JSON: &str = include_str!("../data/learnset.json");

        // Deserialize the JSON data into a Vec<Entry>
        let entries: Vec<Learnset> =
            serde_json::from_str(&LEARNSET_JSON).expect("Failed to parse JSON");

        Learnsets { sets: entries }
    }
}

impl Deref for Learnsets {
    type Target = Vec<Learnset>;

    fn deref(&self) -> &Self::Target {
        &self.sets
    }
}
