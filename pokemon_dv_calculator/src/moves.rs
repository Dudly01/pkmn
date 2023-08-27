use std::collections::HashMap;

/// CSV record containing info from Smogon moves.
#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub category: String,
    pub power: String,
    pub accuracy: String,
    pub pp: String,
    pub description: String,
}

/// Loads the base stats from the CSV file.
pub fn load_moves() -> HashMap<String, Record> {
    let mut moves = HashMap::new();

    const CSV_DATA: &str = include_str!("../data/smogon_rb_moves.csv");
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(CSV_DATA.as_bytes());

    for result in csv_reader.deserialize() {
        let record: Record = result.unwrap();
        moves.insert(record.name.clone(), record);
    }

    moves
}
