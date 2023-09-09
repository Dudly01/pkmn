//! Moves
//!
//! Sources:
//! https://www.smogon.com/dex/rb/moves/
//! https://bulbapedia.bulbagarden.net/wiki/List_of_modified_moves#Between_generations_15

use std::collections::HashMap;

/// A move is the skill Pokémon primarily use in battle.
/// Also known as an attack or technique.
#[derive(Debug, serde::Deserialize)]
pub struct Move {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub category: String,
    pub power: String,
    pub accuracy: String,
    pub pp: String,
    pub description: String,
}

/// The moves available in Gen I.
///
/// Note:
/// The moves are stored with their new case-sensitive names.
/// E.g. PoisonPowder -> Poison Powder
pub struct Moves {
    data: HashMap<String, Move>,
    modified_names: HashMap<&'static str, &'static str>,
}

impl Moves {
    pub fn new() -> Moves {
        let mut moves = HashMap::new();

        const CSV_DATA: &str = include_str!("../data/smogon_rb_moves.csv");
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(CSV_DATA.as_bytes());

        for result in csv_reader.deserialize() {
            let m: Move = result.unwrap();
            moves.insert(m.name.clone(), m);
        }

        let modified_names = HashMap::from([
            ("AncientPower", "Ancient Power"),
            ("BubbleBeam", "Bubble Beam"),
            ("DoubleSlap", "Double Slap"),
            ("DragonBreath", "Dragon Breath"),
            ("DynamicPunch", "Dynamic Punch"),
            ("ExtremeSpeed", "Extreme Speed"),
            ("Faint Attack", "Feint Attack"),
            ("FeatherDance", "Feather Dance"),
            ("GrassWhistle", "Grass Whistle"),
            ("Hi Jump Kick", "High Jump Kick"),
            ("PoisonPowder", "Poison Powder"),
            ("Sand-Attack", "Sand Attack"),
            ("Selfdestruct", "Self-Destruct"),
            ("SmellingSalt", "Smelling Salts"),
            ("SmokeScreen", "Smokescreen"),
            ("Softboiled", "Soft-Boiled"),
            ("SolarBeam", "Solar Beam"),
            ("SonicBoom", "Sonic Boom"),
            ("ThunderPunch", "Thunder Punch"),
            ("ThunderShock", "Thunder Shock"),
            ("ViceGrip", "Vice Grip"),
            ("Vice Grip", "Vise Grip"),      // Gen VII to VIII
            ("Conversion2", "Conversion 2"), // Crystal to Stadium 2
        ]);

        Moves {
            data: moves,
            modified_names: modified_names,
        }
    }

    /// Returns a reference to the Move corresponding to the name.
    pub fn get(&self, name: &str) -> Option<&Move> {
        let move_ = self.data.get(name);

        if move_.is_none() {
            let new_name = self.modified_names.get(name);
            if let Some(new_name) = new_name {
                let move_ = self.data.get(*new_name);
                return move_;
            }
        }

        move_
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_non_existing_move() {
        let moves = Moves::new();
        let move_ = moves.get("XXX");

        assert!(move_.is_none());
    }

    #[test]
    fn get_move() {
        let moves = Moves::new();
        let move_ = moves.get("Poison Powder");

        assert!(move_.is_some_and(|m| m.type_ == "Poison"));
    }

    #[test]
    fn get_old_move() {
        let moves = Moves::new();
        let move_ = moves.get("PoisonPowder");

        assert!(move_.is_some_and(|m| m.type_ == "Poison"));
    }
}
