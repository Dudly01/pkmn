use serde;
use serde_json;

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

    /// Returns a reference to the Learnset corresponding to the Pokemon.
    pub fn get_pokemon(&self, name: &str) -> Option<&Learnset> {
        let learnset = self.sets.iter().find(|&p| p.pokemon == name);
        learnset
    }

    /// Returns a reference to the Learnset corresponding to the national dex number.
    pub fn get_ndex(&self, ndex: usize) -> Option<&Learnset> {
        let learnset = self.sets.get(ndex - 1); // Pokemon are stored in order
        learnset
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

    /// Returns a reference to the Learnset corresponding to the Pokemon.
    pub fn get_pokemon(&self, name: &str) -> Option<&Learnset> {
        let learnset = self.sets.iter().find(|&p| p.pokemon == name);
        learnset
    }

    /// Returns a reference to the Learnset corresponding to the national dex number.
    pub fn get_ndex(&self, ndex: usize) -> Option<&Learnset> {
        let learnset = self.sets.get(ndex - 1); // Pokemon are stored in order
        learnset
    }
}
