#[derive(Debug, serde::Deserialize)]
pub struct RbyPokemon {
    #[serde(rename = "dex_number")]
    pub ndex: i32,
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

pub struct RbyPokedex {
    pokemon: Vec<RbyPokemon>,
}

impl RbyPokedex {
    pub fn new() -> RbyPokedex {
        let mut pokedex: Vec<RbyPokemon> = Vec::with_capacity(151);

        const CSV_DATA: &str = include_str!("../data/smogon_rb_pokemon.csv");
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(CSV_DATA.as_bytes());

        for result in csv_reader.deserialize() {
            let record: RbyPokemon = result.unwrap();
            pokedex.push(record);
        }

        RbyPokedex { pokemon: pokedex }
    }

    /// Returns a reference to the Pokemon corresponding to the name.
    pub fn get_pokemon(&self, name: &str) -> Option<&RbyPokemon> {
        let pokemon = self.pokemon.iter().find(|p| p.name == name);
        pokemon
    }

    /// Returns a reference to the Pokemon corresponding to the national dex number.
    pub fn get_ndex(&self, ndex: usize) -> Option<&RbyPokemon> {
        let pokemon = self.pokemon.get(ndex - 1); // Pokemon are stored in order
        pokemon
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GscPokemon {
    #[serde(rename = "dex_number")]
    pub ndex: i32,
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

    /// Returns a reference to the Pokemon corresponding to the name.
    pub fn get_pokemon(&self, name: &str) -> Option<&GscPokemon> {
        let pokemon = self.pokemon.iter().find(|p| p.name == name);
        pokemon
    }

    /// Returns a reference to the Pokemon corresponding to the national dex number.
    pub fn get_ndex(&self, ndex: usize) -> Option<&GscPokemon> {
        let pokemon = self.pokemon.get(ndex - 1); // Pokemon are stored in order
        pokemon
    }
}
