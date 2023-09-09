//! Module containing the data of the Pokémon.

/// Pokémon
#[derive(Debug, serde::Deserialize)]
pub struct Pokemon {
    #[serde(rename = "dex_number")]
    pub ndex: i32,
    #[serde(rename = "name")]
    pub pokemon: String,
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

/// Loads the Pokemon from the CSV file.
pub fn load_pokemon() -> Vec<Pokemon> {
    let mut records: Vec<Pokemon> = Vec::with_capacity(151);

    const CSV_DATA: &str = include_str!("../data/smogon_rb_pokemon.csv");
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(CSV_DATA.as_bytes());

    for result in csv_reader.deserialize() {
        let record: Pokemon = result.unwrap();
        records.push(record);
    }

    records
}
