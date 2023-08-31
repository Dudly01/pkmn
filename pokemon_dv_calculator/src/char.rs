//! Text characters.
//!
//! Functionality for handling characters found in Pokemon Gen I and II.
//! Includes Latin letters, ligatures, digits, symbols and punctuation.
//!
//! Sources:
//! https://bulbapedia.bulbagarden.net/wiki/Text_entry_in_the_Pok%C3%A9mon_games
//! https://bulbapedia.bulbagarden.net/wiki/Text_entry_(Generation_II)

use crate::roi::Roi;
use std::collections::HashMap;

/// Stores the 7x7 binary image of a character as a u64 value.
///
/// The 0th element is the LSB.
/// The background bits have the value 0.
#[derive(PartialEq, Eq, Hash)]
pub struct CharBitmap(u64);

impl CharBitmap {
    /// Creates a CharBitmap from pixels.
    ///
    /// The 0th pixel will be the LSB.
    /// A bit will be 0 if the pixel value is 0, 1 otherwise.
    pub fn from_pixels(pixels: &[u8]) -> Result<CharBitmap, &str> {
        if pixels.len() != 49 {
            return Err("Expected exactly 49 items in the sequence.");
        }

        let bitmap = pixels
            .iter()
            .enumerate()
            .map(|(idx, x)| ((*x != 0) as u64) << idx) // Shift left by idx
            .fold(0, |acc, x| acc | x); // Bitwise OR

        Ok(CharBitmap(bitmap))
    }

    pub fn from_roi(roi: &Roi) -> Result<CharBitmap, String> {
        let pos = roi.pos();
        if pos.width != 7 || pos.height != 7 {
            let msg = format!("Width and height needs to be 7. Got {:?}", pos);
            return Err(msg);
        }

        let bitmap = roi
            .iter()
            .enumerate()
            .map(|(idx, x)| ((*x != 0) as u64) << idx) // Shift left by idx
            .fold(0, |acc, x| acc | x); // Bitwise OR

        Ok(CharBitmap(bitmap))
    }

    /// Returns the Hamming distance of two CharBitmaps.
    /// It is the number of positions the bits differ.
    pub fn hamming_dist(&self, rhs: &CharBitmap) -> u32 {
        let diff = self.0 ^ rhs.0;
        let hamming_dist = diff.count_ones();
        hamming_dist
    }
}

/// Initializes the map connecting the bitmaps to the chars.
pub fn init_chars() -> HashMap<CharBitmap, &'static str> {
    let mut chars = HashMap::<CharBitmap, &str>::new();

    let char = "0";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 1, 0, 0, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 0, 0, 1, 0, //
        0, 0, 1, 1, 1, 0, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "1";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 1, 1, 1, 1, 1, 1, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "2";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 0, 0, 0, 1, 1, 1, //
        0, 1, 1, 1, 1, 0, 0, //
        1, 1, 1, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 1, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "3";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 0, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "4";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 1, 1, 1, 0, //
        0, 0, 1, 1, 1, 1, 0, //
        0, 1, 1, 0, 1, 1, 0, //
        1, 1, 0, 0, 1, 1, 0, //
        1, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "5";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        0, 0, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "6";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "7";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 0, 0, 0, //
        0, 0, 1, 1, 0, 0, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "8";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = "9";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    let char = " ";
    let code = CharBitmap::from_pixels(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
    ])
    .unwrap();
    chars.insert(code, char);

    chars
}