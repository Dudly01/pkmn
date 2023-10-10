//! Pokémon stats related functionality.
//!
//! Sources:
//! https://www.smogon.com/ingame/guides/rby_gsc_stats.

use std::ops::Deref;

/// The extent to which a stat varies with respect to DV values.
///
/// Due to rounding, different DV values can produce the same stat value.
pub struct StatVariation {
    values: [i32; 16],
}

impl StatVariation {
    /// Calculates the stat variation from the level, base stat and the stat
    /// experience of a Pokémon. The HP is calculated slightly differently
    /// from the other stats.
    pub fn init(level: &i32, base: &i32, exp: &i32, is_hp: &bool) -> StatVariation {
        let offset = if *is_hp { level + 10 } else { 5 };

        let effort_gain = ((exp - 1) as f32).sqrt() + 1.0 / 4.0;
        let effort_gain = effort_gain as i32;

        let variation = std::array::from_fn(|i| {
            let dv = i as i32;
            let val = (((base + dv) * 2 + effort_gain) * level) as f32 / 100.0;
            let val = val as i32 + offset;
            val
        });

        StatVariation { values: variation }
    }
}

impl Deref for StatVariation {
    type Target = [i32; 16];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

/// The range of possible DV values for a stat, with both ends being inclusive.
pub struct DvRange {
    pub min: i32,
    pub max: i32,
}

impl DvRange {
    /// Inits the DvRange from a stat value and a stat variation.
    pub fn init(current_stat: &i32, variation: &StatVariation) -> Option<DvRange> {
        let first = variation.iter().position(|i| i == current_stat);
        let last = variation.iter().rposition(|i| i == current_stat);

        match (first, last) {
            (Some(a), Some(b)) => Some(DvRange {
                min: a as i32,
                max: b as i32,
            }),
            _ => None,
        }
    }
}
