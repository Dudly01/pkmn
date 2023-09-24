use std::ops::Deref;

/// Stores the possible stat values corresponding to the possible DV values.
pub struct StatVariation {
    values: [i32; 16],
}

impl StatVariation {
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

/// Stores the minimum and maximum (inclusive) DV values of a Pokemon.
/// 
/// A stat value may not determine an exact DV value due to rounding.
pub struct DvRange {
    pub min: i32,
    pub max: i32,
}

impl DvRange {
    pub fn init(current_stat: &i32, variation: &StatVariation) -> Option<DvRange> {
        let first = variation.iter().position(|i| i == current_stat);
        let last = variation.iter().rev().position(|i| i == current_stat);

        match (first, last) {
            (Some(a), Some(b)) => Some(DvRange {
                min: a as i32,
                max: b as i32,
            }),
            _ => None,
        }
    }
}
