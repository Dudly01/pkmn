//! Text characters.
//!
//! Functionality for handling characters found in Pokemon Gen I and II.
//! Includes Latin letters, ligatures, digits, symbols and punctuation.
//!
//! Sources:  
//! https://bulbapedia.bulbagarden.net/wiki/Text_entry_in_the_Pok%C3%A9mon_games  
//! https://bulbapedia.bulbagarden.net/wiki/Text_entry_(Generation_II)

use crate::position::Position;
use crate::roi::Roi;
use image::imageops::invert;
use image::GrayImage;
use imageproc::contrast::threshold_mut;
use std::collections::HashMap;
use std::ops::Deref;

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

/// A map from the CharBitmaps to the characters.
///
/// The characters include:
/// - Latin letters
/// - ligatures (PK, MN etc.)
/// - digits
/// - symbols and punctuation (?, ! etc.)
pub struct Charset {
    chars: HashMap<CharBitmap, &'static str>,
}

const IMG_NICKNAMING_1: &[u8] = include_bytes!("../data/images/Yellow_nicknaming_upper.png");
const IMG_NICKNAMING_2: &[u8] = include_bytes!("../data/images/Crystal_nicknaming_upper.png");

impl Charset {
    pub fn new() -> Charset {
        let mut chars = HashMap::<CharBitmap, &str>::new();

        let img_nicknaming =
            image::load_from_memory(IMG_NICKNAMING_1).expect("could not load Nicknaming_I.png");
        let mut img_nicknaming = img_nicknaming.to_luma8();
        threshold_mut(&mut img_nicknaming, 200); // Needed as black is 7 white is 23x
        invert(&mut img_nicknaming);
        let img_nicknaming = img_nicknaming;

        let char_positions = [
            ("A", 0, 0),
            ("B", 0, 1),
            ("C", 0, 2),
            ("D", 0, 3),
            ("E", 0, 4),
            ("F", 0, 5),
            ("G", 0, 6),
            ("H", 0, 7),
            ("I", 0, 8),
            ("J", 1, 0),
            ("K", 1, 1),
            ("L", 1, 2),
            ("M", 1, 3),
            ("N", 1, 4),
            ("O", 1, 5),
            ("P", 1, 6),
            ("Q", 1, 7),
            ("R", 1, 8),
            ("S", 2, 0),
            ("T", 2, 1),
            ("U", 2, 2),
            ("V", 2, 3),
            ("W", 2, 4),
            ("X", 2, 5),
            ("Y", 2, 6),
            ("Z", 2, 7),
            (" ", 2, 8),
            ("-", 4, 0),
        ];
        for (char, row, col) in char_positions {
            let char_pos = Position {
                x: 16 + col * 16,
                y: 40 + row * 16,
                width: 7,
                height: 7,
            };

            let roi = Roi {
                img: &img_nicknaming,
                pos: char_pos,
            };

            let bitmap = CharBitmap::from_roi(&roi).unwrap();

            chars.insert(bitmap, char);
        }

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

        let char = "/";
        let code = CharBitmap::from_pixels(&[
            0, 0, 0, 0, 0, 0, 1, //
            0, 0, 0, 0, 0, 1, 0, //
            0, 0, 0, 0, 1, 0, 0, //
            0, 0, 0, 1, 0, 0, 0, //
            0, 0, 1, 0, 0, 0, 0, //
            0, 1, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, 0, 0, //
        ])
        .unwrap();
        chars.insert(code, char);

        // GSC has a slightly modified set of character

        let img_nicknaming =
            image::load_from_memory(IMG_NICKNAMING_2).expect("could not load Nicknaming_II.png");
        let mut img_nicknaming = img_nicknaming.to_luma8();
        threshold_mut(&mut img_nicknaming, 200); // Needed as black is 7 white is 23x
        invert(&mut img_nicknaming);
        let img_nicknaming = img_nicknaming;

        let char_positions = [
            ("A", 0, 0),
            ("B", 0, 1),
            ("C", 0, 2),
            ("D", 0, 3),
            ("E", 0, 4),
            ("F", 0, 5),
            ("G", 0, 6),
            ("H", 0, 7),
            ("I", 0, 8),
            ("J", 1, 0),
            ("K", 1, 1),
            ("L", 1, 2),
            ("M", 1, 3),
            ("N", 1, 4),
            ("O", 1, 5),
            ("P", 1, 6),
            ("Q", 1, 7),
            ("R", 1, 8),
            ("S", 2, 0),
            ("T", 2, 1),
            ("U", 2, 2),
            ("V", 2, 3),
            ("W", 2, 4),
            ("X", 2, 5),
            ("Y", 2, 6),
            ("Z", 2, 7),
            (" ", 2, 8),
        ];
        for (char, row, col) in char_positions {
            let char_pos = Position {
                x: 16 + col * 16,
                y: 64 + row * 16,
                width: 7,
                height: 7,
            };

            let roi = Roi {
                img: &img_nicknaming,
                pos: char_pos,
            };

            let bitmap = CharBitmap::from_roi(&roi).unwrap();

            chars.insert(bitmap, char);
        }

        let char = "2";
        let code = CharBitmap::from_pixels(&[
            0, 0, 0, 0, 0, 0, 0, //
            0, 1, 1, 1, 1, 1, 0, //
            1, 1, 0, 0, 0, 1, 1, //
            0, 0, 0, 0, 1, 1, 1, //
            0, 1, 1, 1, 1, 1, 0, //
            1, 1, 1, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, 1, 1, //
        ])
        .unwrap();
        chars.insert(code, char);

        let char = "5";
        let code = CharBitmap::from_pixels(&[
            0, 0, 0, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, 1, 0, //
            1, 0, 0, 0, 0, 0, 0, //
            1, 1, 1, 1, 1, 1, 0, //
            0, 0, 0, 0, 0, 1, 1, //
            1, 1, 0, 0, 0, 1, 1, //
            0, 1, 1, 1, 1, 1, 0, //
        ])
        .unwrap();
        chars.insert(code, char);

        Charset { chars: chars }
    }
}

impl Deref for Charset {
    type Target = HashMap<CharBitmap, &'static str>;

    fn deref(&self) -> &Self::Target {
        &self.chars
    }
}

/// Reads a character from 7x7 pixel large region of a `GrayImage`.
pub fn read_char(img: &GrayImage, pos: &Position, chars: &Charset) -> Result<&'static str, String> {
    if pos.width != 7 || pos.height != 7 {
        return Err("incorrect Roi dimensions".to_string());
    }

    let roi = Roi {
        img: img,
        pos: pos.clone(),
    };

    let bitmap = CharBitmap::from_roi(&roi)?;

    let char = chars.get(&bitmap).ok_or("could not recognize character")?;
    Ok(*char)
}

/// Reads the characters from the field.
pub fn read_field(img: &GrayImage, pos: &Position, chars: &Charset) -> Result<String, String> {
    if pos.height != 7 || (pos.width + 1) % 8 != 0 {
        return Err("incorrect Roi dimensions".to_string());
    }

    let char_count = (pos.width + 1) / 8;
    let mut result = String::with_capacity(char_count as usize);

    for i in 0..char_count {
        let offset_x = i * (7 + 1);
        let char_pos = Position {
            x: pos.x + offset_x,
            y: pos.y,
            width: 7,
            height: 7,
        };

        let char = read_char(img, &char_pos, chars)
            .map_err(|err| format!("could not read character #{i}: {err}"))?;

        result.extend(char.chars());
    }

    Ok(result)
}
