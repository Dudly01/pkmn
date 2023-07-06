pub mod pkmn_stats {

    #[derive(Debug, serde::Deserialize)]
    pub struct RbyRecord {
        ndex: i32,
        pokemon: String,
        types: String,
        hp: i32,
        attack: i32,
        defense: i32,
        speed: i32,
        special: i32,
        total: i32,
    }

    /// Loads the base stats from the CSV file.
    pub fn load_stats() -> Vec<RbyRecord> {
        let mut records: Vec<RbyRecord> = Vec::with_capacity(151);

        let csv_path = "data/base_stats.csv";
        let mut csv_reader = csv::Reader::from_path(csv_path).expect("could not load CSV file.");

        for result in csv_reader.deserialize() {
            let record: RbyRecord = result.unwrap();
            records.push(record);
        }

        records
    }
}
