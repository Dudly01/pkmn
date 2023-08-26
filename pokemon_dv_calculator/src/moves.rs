/// Base stats csv records, containing the information about Pokemons.
#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub name: String,
    pub description: String,
}

/// Loads the base stats from the CSV file.
pub fn load_moves() -> Vec<Record> {
    let mut records: Vec<Record> = Vec::with_capacity(151);

    const CSV_DATA: &str = include_str!("../data/smogon_rb_moves.csv");
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(CSV_DATA.as_bytes());

    for result in csv_reader.deserialize() {
        let record: Record = result.unwrap();
        records.push(record);
    }

    records
}
