//! Functionality to format data

use crate::learnset::Learnset;
use crate::moves::{GscMoves, Move, Moves};
use crate::stats::DvRange;

/// Returns the header of the stat table.
pub fn fmt_stat_header() -> String {
    let h = format!(
        "{:>4}  {:>4}  {:>5}  {}\n",
        "Stat", "Base", "Value", "DV Range"
    );
    h
}

/// Returns a row of the stat table.
pub fn fmt_stat_row(stat: &str, base_stat: &i32, stat_value: &i32, dv_range: &DvRange) -> String {
    let h = format!(
        "{:>4}  {:>4}  {:>5}  {:>3} - {:>2}\n",
        stat, base_stat, stat_value, dv_range.min, dv_range.max
    );
    h
}

pub fn fmt_move_header() -> String {
    format!(
        "{:<15}  {:<8}  {:<12}  {:>3}  {:>4}  {:>2}  {}",
        "Move", "Type", "Cat", "Pow", "Acc", "PP", "Desc"
    )
}

pub fn fmt_move(move_: Option<&Move>) -> String {
    match move_ {
        Some(m) => {
            format!(
                "{:<15}  {:<8}  {:<12}  {:>3}  {:>3}%  {:>2}  {}",
                m.name, m.type_, m.category, m.power, m.accuracy, m.pp, m.description
            )
        }
        None => {
            format!(
                "{:<15}  {:<8}  {:<12}  {:>3}  {:>3}   {:>2}  {}",
                "-", "-", "-", "-", "-", "-", "MOVE DATA NOT FOUND"
            )
        }
    }
}

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset is the same among game versions.
fn fmt_shared_learnset_table(learnset: &Learnset, moves: &Moves) -> Result<String, String> {
    let mut t = String::with_capacity(256);

    // Pokemon
    t.push_str(&format!(
        "No.{} {} learnset\n",
        learnset.ndex, learnset.pokemon
    ));

    // Header
    t.push_str(&format!("{:<3}  {}\n", "Lvl", fmt_move_header()));

    // Moves
    for row in learnset.by_leveling_up.iter().skip(1) {
        let move_name = &row[1];
        let move_ = moves.get(move_name);

        let rt = format!("{:<3}  {}\n", row[0], fmt_move(move_));
        t.push_str(&rt);
    }

    Ok(t)
}

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset differs among game versions.
fn fmt_divering_learnset_table(learnset: &Learnset, moves: &Moves) -> Result<String, String> {
    let mut t = String::with_capacity(256);

    // Pokemon
    t.push_str(&format!(
        "No.{} {} learnset\n",
        learnset.ndex, learnset.pokemon
    ));

    // Header
    t.push_str(&format!("{:<3}  {:<3}  {}\n", "RB", "Y", fmt_move_header()));

    // Moves
    for row in learnset.by_leveling_up.iter().skip(1) {
        let move_name = &row[2];
        let move_ = moves.get(move_name);

        let rt = format!("{:<3}  {:<3}  {}\n", row[0], row[1], fmt_move(move_));

        t.push_str(&rt);
    }

    Ok(t)
}

/// Returns the string with the formatted "By leveling up" learnset.
pub fn fmt_learnset(learnset: &Learnset, moves: &Moves) -> Result<String, String> {
    let level_up_table = &learnset.by_leveling_up;
    let col_count = level_up_table[0].len();
    for row in level_up_table {
        if row.len() != col_count {
            return Err(format!("Mismatching column count for {}", learnset.pokemon));
        }
    }

    let t = match col_count {
        2 => fmt_shared_learnset_table(learnset, moves),
        3 => fmt_divering_learnset_table(learnset, moves),
        _ => Err(format!(
            "Expected column count of 2 or 3, got {}",
            col_count
        )),
    };

    t
}

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset is the same among game versions.
fn fmt_gsc_shared_learnset_table(learnset: &Learnset, moves: &GscMoves) -> Result<String, String> {
    let mut t = String::with_capacity(256);

    // Pokemon
    t.push_str(&format!(
        "No.{} {} learnset\n",
        learnset.ndex, learnset.pokemon
    ));

    // Header
    t.push_str(&format!("{:<3}  {}\n", "Lvl", fmt_move_header()));

    // Moves
    for row in learnset.by_leveling_up.iter().skip(1) {
        let move_name = &row[1];
        let move_ = moves.get(move_name);

        let rt = format!("{:<3}  {}\n", row[0], fmt_move(move_));
        t.push_str(&rt);
    }

    Ok(t)
}

/// Returns a formatted "By leveling up" learnset table.
/// For the cases when the learnset differs among game versions.
fn fmt_gsc_divering_learnset_table(
    learnset: &Learnset,
    moves: &GscMoves,
) -> Result<String, String> {
    let mut t = String::with_capacity(256);

    // Pokemon
    t.push_str(&format!(
        "No.{} {} learnset\n",
        learnset.ndex, learnset.pokemon
    ));

    // Header
    t.push_str(&format!("{:<3}  {:<3}  {}\n", "RB", "Y", fmt_move_header()));

    // Moves
    for row in learnset.by_leveling_up.iter().skip(1) {
        let move_name = &row[2];
        let move_ = moves.get(move_name);

        let rt = format!("{:<3}  {:<3}  {}\n", row[0], row[1], fmt_move(move_));

        t.push_str(&rt);
    }

    Ok(t)
}

/// Returns the string with the formatted "By leveling up" learnset.
pub fn fmt_gsc_learnset(learnset: &Learnset, moves: &GscMoves) -> Result<String, String> {
    let level_up_table = &learnset.by_leveling_up;
    let col_count = level_up_table[0].len();
    for row in level_up_table {
        if row.len() != col_count {
            return Err(format!("Mismatching column count for {}", learnset.pokemon));
        }
    }

    let t = match col_count {
        2 => fmt_gsc_shared_learnset_table(learnset, moves),
        3 => fmt_gsc_divering_learnset_table(learnset, moves),
        _ => Err(format!(
            "Expected column count of 2 or 3, got {}",
            col_count
        )),
    };

    t
}
