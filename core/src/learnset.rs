use serde;
use serde_json;
use std::ops::Deref;

#[derive(Debug, serde::Deserialize)]
pub struct Learnset {
    pub ndex: String,
    pub pokemon: String,
    pub by_leveling_up: Vec<Vec<String>>,
}

pub struct RbyLearnsets {
    sets: Vec<Learnset>,
}

impl RbyLearnsets {
    pub fn new() -> RbyLearnsets {
        const LEARNSET_JSON: &str = include_str!("../data/geni_learnsets.json");

        // Deserialize the JSON data into a Vec<Entry>
        let entries: Vec<Learnset> =
            serde_json::from_str(&LEARNSET_JSON).expect("Failed to parse JSON");

        RbyLearnsets { sets: entries }
    }
}

impl Deref for RbyLearnsets {
    type Target = Vec<Learnset>;

    fn deref(&self) -> &Self::Target {
        &self.sets
    }
}

pub struct GscLearnsets {
    sets: Vec<Learnset>,
}

impl GscLearnsets {
    pub fn new() -> GscLearnsets {
        const LEARNSET_JSON: &str = include_str!("../data/genii_learnsets.json");

        // Deserialize the JSON data into a Vec<Entry>
        let entries: Vec<Learnset> =
            serde_json::from_str(&LEARNSET_JSON).expect("Failed to parse JSON");

        GscLearnsets { sets: entries }
    }
}

impl Deref for GscLearnsets {
    type Target = Vec<Learnset>;

    fn deref(&self) -> &Self::Target {
        &self.sets
    }
}
