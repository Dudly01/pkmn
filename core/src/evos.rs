/// Loads the evo chains from the file
pub fn load_evos() -> Vec<String> {
    const EVO_DATA: &str = include_str!("../data/geni_evo_chains.txt");

    let evo_chains = EVO_DATA.lines().map(|x| x.to_owned()).collect();

    evo_chains
}

/// Loads the evo chains from the file
pub fn load_gsc_evos() -> Vec<String> {
    const EVO_DATA: &str = include_str!("../data/genii_evo_chains.txt");

    let evo_chains = EVO_DATA.lines().map(|x| x.to_owned()).collect();

    evo_chains
}
