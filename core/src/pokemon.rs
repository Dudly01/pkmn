use std::ops::Deref;

#[derive(Debug, serde::Deserialize)]
pub struct Pokemon {
    #[serde(rename = "dex_number")]
    pub ndex: i32,
    #[serde(rename = "name")]
    pub name: String,
    pub type1: String,
    pub type2: String,
    pub hp: i32,
    #[serde(rename = "atk")]
    pub attack: i32,
    #[serde(rename = "def")]
    pub defense: i32,
    #[serde(rename = "spe")]
    pub speed: i32,
    #[serde(rename = "spa")]
    pub special: i32,
}

pub struct Pokedex {
    pokemon: Vec<Pokemon>,
}

impl Pokedex {
    pub fn new() -> Pokedex {
        let mut pokedex: Vec<Pokemon> = Vec::with_capacity(151);

        const CSV_DATA: &str = include_str!("../data/smogon_rb_pokemon.csv");
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(CSV_DATA.as_bytes());

        for result in csv_reader.deserialize() {
            let record: Pokemon = result.unwrap();
            pokedex.push(record);
        }

        Pokedex { pokemon: pokedex }
    }
}

impl Deref for Pokedex {
    type Target = Vec<Pokemon>;

    fn deref(&self) -> &Self::Target {
        &self.pokemon
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GscPokemon {
    #[serde(rename = "dex_number")]
    pub ndex: i32,
    #[serde(rename = "name")]
    pub name: String,
    pub type1: String,
    pub type2: String,
    pub hp: i32,
    #[serde(rename = "atk")]
    pub attack: i32,
    #[serde(rename = "def")]
    pub defense: i32,
    #[serde(rename = "spa")]
    pub special_attack: i32,
    #[serde(rename = "spd")]
    pub special_defense: i32,
    #[serde(rename = "spe")]
    pub speed: i32,
}

pub struct GscPokedex {
    pokemon: Vec<GscPokemon>,
}

impl GscPokedex {
    pub fn new() -> GscPokedex {
        let mut pokedex: Vec<GscPokemon> = Vec::with_capacity(251);

        const CSV_DATA: &str = include_str!("../data/smogon_gs_pokemon.csv");
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(CSV_DATA.as_bytes());

        for result in csv_reader.deserialize() {
            let record: GscPokemon = result.unwrap();
            pokedex.push(record);
        }

        GscPokedex { pokemon: pokedex }
    }
}

impl Deref for GscPokedex {
    type Target = Vec<GscPokemon>;

    fn deref(&self) -> &Self::Target {
        &self.pokemon
    }
}
