use std::collections::HashMap;

/// Base stats csv records, containing the information about Pokemons.
#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub name: String,
    pub description: String,
}

/// Loads the base stats from the CSV file.
pub fn load_moves() -> HashMap<String, String> {
    let mut moves = HashMap::new();

    const CSV_DATA: &str = include_str!("../data/smogon_rb_moves.csv");
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(CSV_DATA.as_bytes());

    for result in csv_reader.deserialize() {
        let record: Record = result.unwrap();
        moves.insert(record.name, record.description);
    }

    moves
}
