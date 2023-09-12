use crate::gameboy::StatsSreen1Content;
use crate::pokemon::Pokemon;
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

/// The stats of a Pokemon.
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

impl Stats {
    pub fn from_screen_content(content: &StatsSreen1Content) -> Stats {
        Stats {
            hp: content.hp,
            attack: content.attack,
            defense: content.defense,
            speed: content.speed,
            special: content.special,
        }
    }
}

/// The base stats of a Pokemon species.
pub struct BaseStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

impl BaseStats {
    pub fn from_record(record: &Pokemon) -> BaseStats {
        BaseStats {
            hp: record.hp,
            attack: record.attack,
            defense: record.defense,
            speed: record.speed,
            special: record.special,
        }
    }
}

/// The experience (Effort Values) gained by a Pokemon.
pub struct Experience {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

impl Experience {
    pub fn with_no_experience() -> Experience {
        Experience {
            hp: 0,
            attack: 0,
            defense: 0,
            speed: 0,
            special: 0,
        }
    }
}

/// Contains the stat values corresponding to the possible DV values.
pub struct DvTable {
    pub hp: [i32; 16],
    pub attack: [i32; 16],
    pub defense: [i32; 16],
    pub speed: [i32; 16],
    pub special: [i32; 16],
}

impl DvTable {
    pub fn new(level: &i32, base: &BaseStats, exp: &Experience) -> DvTable {
        let hp = get_dv_stat_pairs(level, &base.hp, &exp.hp, &true);
        let attack = get_dv_stat_pairs(level, &base.attack, &exp.attack, &false);
        let defense = get_dv_stat_pairs(level, &base.defense, &exp.defense, &false);
        let speed = get_dv_stat_pairs(level, &base.speed, &exp.speed, &false);
        let special = get_dv_stat_pairs(level, &base.special, &exp.special, &false);

        DvTable {
            hp: hp.try_into().unwrap(),
            attack: attack.try_into().unwrap(),
            defense: defense.try_into().unwrap(),
            speed: speed.try_into().unwrap(),
            special: special.try_into().unwrap(),
        }
    }

    /// Prints the table to the terminal in a nicely formatted fashion.
    pub fn print(&self, stats: &Stats) {
        println!(
            "{: >4} {: >4} {: >4} {: >4} {: >4} {: >4}",
            "DV", "HP", "ATT", "DEF", "SPD", "SPC"
        );

        for i in 0..16 {
            let special_char = "-";

            let hp_eq = if self.hp[i] == stats.hp {
                special_char
            } else {
                " "
            };
            let attack_eq = if self.attack[i] == stats.attack {
                special_char
            } else {
                " "
            };
            let defense_eq = if self.defense[i] == stats.defense {
                special_char
            } else {
                " "
            };
            let speed_eq = if self.speed[i] == stats.speed {
                special_char
            } else {
                " "
            };
            let special_eq = if self.special[i] == stats.special {
                special_char
            } else {
                " "
            };

            println!(
                "{: >4} {: >4}{}{: >4}{}{: >4}{}{: >4}{}{: >4}{}",
                i,
                self.hp[i],
                hp_eq,
                self.attack[i],
                attack_eq,
                self.defense[i],
                defense_eq,
                self.speed[i],
                speed_eq,
                self.special[i],
                special_eq,
            );
        }
    }
}

/// Returns a vector of stats, where the index corresponds to the DV value.
/// The HP and the other stats differ slightly in calculation.
/// Hence the is_hp boolean argument.
pub fn get_dv_stat_pairs(level: &i32, base: &i32, exp: &i32, is_hp: &bool) -> Vec<i32> {
    let offset = if *is_hp { level + 10 } else { 5 };

    let mut result = Vec::with_capacity(16);
    let effort_gain = ((exp - 1) as f32).sqrt() + 1.0 / 4.0;
    let effort_gain = effort_gain as i32;

    for dv in 0..16 {
        let val = (((base + dv) * 2 + effort_gain) * level) as f32 / 100.0;
        let val = val as i32 + offset;
        result.push(val);
    }

    result
}

/// Contains the range of possible DVs for a Pokemon given its stats.
/// The range is inclusive on both ends.
pub struct DvRanges {
    pub hp: Option<(usize, usize)>,
    pub attack: Option<(usize, usize)>,
    pub defense: Option<(usize, usize)>,
    pub speed: Option<(usize, usize)>,
    pub special: Option<(usize, usize)>,
}

impl DvRanges {
    pub fn new(stats: &Stats, dv_table: &DvTable) -> DvRanges {
        let hp = find_dv_range(&stats.hp, &dv_table.hp);
        let attack = find_dv_range(&stats.attack, &dv_table.attack);
        let defense = find_dv_range(&stats.defense, &dv_table.defense);
        let speed = find_dv_range(&stats.speed, &dv_table.speed);
        let special = find_dv_range(&stats.special, &dv_table.special);

        DvRanges {
            hp: hp,
            attack: attack,
            defense: defense,
            speed: speed,
            special: special,
        }
    }
}

pub fn find_dv_range(stat_val: &i32, dv_stat_pairs: &[i32; 16]) -> Option<(usize, usize)> {
    let mut start = -1;
    let mut end = -1;

    for (i, val) in dv_stat_pairs.iter().enumerate() {
        if *val == *stat_val as i32 {
            start = i as i32;
            break;
        }
    }

    if start == -1 {
        return None;
    }

    for (i, val) in dv_stat_pairs.iter().enumerate().rev() {
        if *val == *stat_val as i32 {
            end = i as i32;
            break;
        }
    }

    Some((start as usize, end as usize))
}

/// Returns the text summary of the Pokemon.
/// The result is formatted with line breaks ('\n') and spaces (' ').
/// It terminal printable.
pub fn summarize_pkmn_stats(
    record: &Pokemon,
    base_stats: &BaseStats,
    level: i32,
    stats: &Stats,
    dv_stats_table: &DvTable,
    dv_ranges: &DvRanges,
) -> String {
    let mut text_result = String::with_capacity(128);

    let hp = match dv_ranges.hp {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let attack = match dv_ranges.attack {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let defense = match dv_ranges.defense {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let speed = match dv_ranges.speed {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    let special = match dv_ranges.special {
        Some(r) => format!("{:>2} - {:>2}", r.0, r.1),
        None => String::from("Stat is not within expectations."),
    };

    text_result.push_str(&format!(
        "{: <} No.{: >03} :L{: <3}\n",
        record.name, record.ndex, level
    ));

    text_result.push_str(&format!("\n"));
    text_result.push_str(&format!("Stats       DVs [min:max]\n"));
    text_result.push_str(&format!(" HP: {:>3}    {}\n", stats.hp, hp));
    text_result.push_str(&format!("ATT: {:>3}    {}\n", stats.attack, attack));
    text_result.push_str(&format!("DEF: {:>3}    {}\n", stats.defense, defense));
    text_result.push_str(&format!("SPD: {:>3}    {}\n", stats.speed, speed));
    text_result.push_str(&format!("SPC: {:>3}    {}\n", stats.special, special));

    text_result.push_str(&format!("\n"));
    text_result.push_str(&format!("Base stats\n"));
    text_result.push_str(&format!(
        "{: >3}  {: >3}  {: >3}  {: >3}  {: >3}\n",
        " HP", "ATT", "DEF", "SPC", "SPD"
    ));
    text_result.push_str(&format!(
        "{: >3}  {: >3}  {: >3}  {: >3}  {: >3}  \n",
        base_stats.hp, base_stats.attack, base_stats.defense, base_stats.speed, base_stats.special,
    ));

    text_result.push_str(&format!("\n"));
    text_result.push_str(&format!("DV-Stats table\n"));

    text_result.push_str(&format!(
        "{: >4} {: >4} {: >4} {: >4} {: >4} {: >4}\n",
        "DV", "HP", "ATT", "DEF", "SPD", "SPC"
    ));

    for i in 0..16 {
        let special_char = "-";

        let hp_eq = if dv_stats_table.hp[i] == stats.hp {
            special_char
        } else {
            " "
        };
        let attack_eq = if dv_stats_table.attack[i] == stats.attack {
            special_char
        } else {
            " "
        };
        let defense_eq = if dv_stats_table.defense[i] == stats.defense {
            special_char
        } else {
            " "
        };
        let speed_eq = if dv_stats_table.speed[i] == stats.speed {
            special_char
        } else {
            " "
        };
        let special_eq = if dv_stats_table.special[i] == stats.special {
            special_char
        } else {
            " "
        };

        text_result.push_str(&format!(
            "{: >4} {: >4}{}{: >4}{}{: >4}{}{: >4}{}{: >4}{}\n",
            i,
            dv_stats_table.hp[i],
            hp_eq,
            dv_stats_table.attack[i],
            attack_eq,
            dv_stats_table.defense[i],
            defense_eq,
            dv_stats_table.speed[i],
            speed_eq,
            dv_stats_table.special[i],
            special_eq,
        ));
    }
    text_result
}
