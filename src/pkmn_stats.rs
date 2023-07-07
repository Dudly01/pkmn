pub mod pkmn_stats {

    #[derive(Debug, serde::Deserialize)]
    pub struct RbyRecord {
        pub ndex: i32,
        pub pokemon: String,
        pub types: String,
        pub hp: i32,
        pub attack: i32,
        pub defense: i32,
        pub speed: i32,
        pub special: i32,
        pub total: i32,
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
