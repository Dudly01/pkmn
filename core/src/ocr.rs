//! The OCR related functionality.
//!
//! This module enables reading texts from Pokemon RBY and GSC.
//! A character can refer to digits, Latin letters, ligatures, symbols and punctuation.
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

/// Encodes the binary image of a text character as a u64 value.
///
/// A character occupies a 7x7 region on the image. These pixels are encoded
/// as a u64 value, where the n-th bit correponds to the n-th pixel in
/// row-major order. The bits of the background have a value of 0.
/// Pixels with a non-zero value are part of the foreground.
#[derive(PartialEq, Eq, Hash)]
pub struct CharBitmap(u64);

impl CharBitmap {
    /// Encodes the sequence of pixels.
    pub fn from_pixels(pixels: &[u8]) -> Result<CharBitmap, String> {
        if pixels.len() != 49 {
            let msg = format!("Expected 49 pixels, got {:?}", pixels.len());
            return Err(msg);
        }

        let bitmap = pixels
            .iter()
            .enumerate()
            .map(|(idx, x)| ((*x != 0) as u64) << idx) // Shift left by idx
            .fold(0, |acc, x| acc | x); // Bitwise OR

        Ok(CharBitmap(bitmap))
    }

    /// Encodes the pixels of the Region of Interest (RoI).
    pub fn from_roi(img: &GrayImage, pos: &Position) -> Result<CharBitmap, String> {
        if pos.width != 7 || pos.height != 7 {
            let msg = format!(
                "Expected width and height to be 7, got {:?} and {:?}",
                pos.width, pos.height
            );
            return Err(msg);
        }

        let roi = Roi {
            img: &img,
            pos: pos.clone(),
        };

        let bitmap = roi
            .iter()
            .enumerate()
            .map(|(idx, x)| ((*x != 0) as u64) << idx) // Shift left by idx
            .fold(0, |acc, x| acc | x); // Bitwise OR

        Ok(CharBitmap(bitmap))
    }

    /// Returns the number of bits the two CharBitmaps differ.
    pub fn hamming_dist(&self, rhs: &CharBitmap) -> u32 {
        let diff = self.0 ^ rhs.0;
        let hamming_dist = diff.count_ones();
        hamming_dist
    }
}

/// Enables to decode a character on the screen.
///
/// Contains upper case letters and digits for Pokemon RBY and GSC.
/// No other character was necessery yet.
pub struct CharTable {
    chars: HashMap<CharBitmap, &'static str>,
}

const IMG_NICKNAMING_1: &[u8] = include_bytes!("../data/images/Yellow_nicknaming_upper.png");
const IMG_NICKNAMING_2: &[u8] = include_bytes!("../data/images/Crystal_nicknaming_upper.png");

impl CharTable {
    pub fn new() -> CharTable {
        let mut chars = HashMap::<CharBitmap, &str>::new();

        let img_nicknaming =
            image::load_from_memory(IMG_NICKNAMING_1).expect("failed to load image");
        let mut img_nicknaming = img_nicknaming.to_luma8();
        threshold_mut(&mut img_nicknaming, 200); // Needed as black is 7 white is 23x
        invert(&mut img_nicknaming); // Background should have the value of 0
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

            let bitmap = CharBitmap::from_roi(&img_nicknaming, &char_pos).unwrap();

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
            image::load_from_memory(IMG_NICKNAMING_2).expect("failed to load image");
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

            let bitmap = CharBitmap::from_roi(&img_nicknaming, &char_pos).unwrap();

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

        CharTable { chars: chars }
    }
}

impl Deref for CharTable {
    type Target = HashMap<CharBitmap, &'static str>;

    fn deref(&self) -> &Self::Target {
        &self.chars
    }
}

/// Reads a character from a 7x7 pixel large region of an image.
pub fn read_char(
    img: &GrayImage,
    pos: &Position,
    chars: &CharTable,
) -> Result<&'static str, String> {
    if pos.width != 7 || pos.height != 7 {
        let msg = format!(
            "Expected width and height to be 7, got {:?} and {:?}",
            pos.width, pos.height
        );
        return Err(msg);
    }

    let bitmap = CharBitmap::from_roi(img, &pos)?;

    let &char = chars.get(&bitmap).ok_or("character not recognized")?;
    Ok(char)
}

/// Reads one line of text from the image.
///
/// A character is 7 pixels wide and high. There is a single pixel of space
/// between characters.
pub fn read_field(img: &GrayImage, pos: &Position, chars: &CharTable) -> Result<String, String> {
    if pos.height != 7 {
        let msg = format!("Expected height to be 7, got {:?}", pos.height);
        return Err(msg);
    }
    if (pos.width + 1) % 8 != 0 {
        let msg = format!("Invalid width of {:?}", pos.width);
        return Err(msg);
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
