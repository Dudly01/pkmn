use std::collections::HashMap;

/// A move is the skill Pokémon primarily use in battle.
/// Also known as an attack or technique.
#[derive(Debug, serde::Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
}

pub struct GscItems {
    data: HashMap<String, Item>,
    modified_names: HashMap<String, String>,
}

impl GscItems {
    pub fn new() -> GscItems {
        let mut items = HashMap::new();

        const CSV_DATA: &str = include_str!("../data/smogon_gs_items.csv");
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(CSV_DATA.as_bytes());

        for result in csv_reader.deserialize() {
            let item: Item = result.expect("could not deserialize Item");
            items.insert(item.name.to_lowercase(), item);
        }

        // Description taken from Bulbapedia
        let non_competitive_items = [
            ("amulet coin", "Doubles the prize money after battle."),
            ("cleanse tag", "Halves the encounter rate of wild Pokémon."),
            ("everstone", "Prevents the holder from evolving."),
            (
                "exp. share",
                "Holder receives 50% of the experience after battle.",
            ),
            ("lucky egg", "Holder receives 1.5x experience."),
            (
                "smoke ball",
                "Fleeing from a wild Pokémon is guaranteed to succeed.",
            ),
        ];

        items.extend(non_competitive_items.iter().map(|(name, description)| {
            (
                name.to_lowercase(),
                Item {
                    name: name.to_string(),
                    description: description.to_string(),
                },
            )
        }));

        let modified_names = HashMap::from(
            [
                ("blackglasses", "Black Glasses"),
                ("brightpowder", "Bright Powder"),
                ("miracleberry", "Miracle Berry"),
                ("mysteryberry", "Mystery Berry"),
                ("nevermeltice", "Never-Melt Ice"),
                ("przcureberry", "PRZ Cure Berry"),
                ("psncureberry", "PSN Cure Berry"),
                ("silverpowder", "Silver Powder"),
                ("thunderstone", "Thunder Stone"),
                ("twistedspoon", "Twisted Spoon"),
            ]
            .map(|(k, v)| (k.to_lowercase(), v.to_lowercase())),
        );

        GscItems {
            data: items,
            modified_names: modified_names,
        }
    }

    /// Returns a reference to the Item corresponding to the name.
    pub fn get(&self, name: &str) -> Option<&Item> {
        let name = name.to_lowercase();
        let item = self.data.get(&name);

        if item.is_none() {
            let new_name = self.modified_names.get(&name);
            if let Some(new_name) = new_name {
                let item = self.data.get(new_name);
                return item;
            }
        }

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_non_existing_item() {
        let items = GscItems::new();
        let i = items.get("XXX");

        assert!(i.is_none());
    }

    #[test]
    fn get_item() {
        let items = GscItems::new();
        let i = items.get("Never-Melt Ice");

        assert!(i.is_some_and(|i| i.description == "Holder's Ice-type attacks have 1.1x power."));
    }

    #[test]
    fn get_old_item() {
        let items = GscItems::new();
        let i = items.get("NEVERMELTICE");

        assert!(i.is_some_and(|i| i.description == "Holder's Ice-type attacks have 1.1x power."));
    }
}
