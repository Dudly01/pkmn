use image::GrayImage;

use crate::char::{CharBitmap, Charset};
use crate::position::Position;
use crate::roi::Roi;

/// Reads a character from 7x7 pixel large region of a `GrayImage`.
pub fn read_char(img: &GrayImage, pos: &Position, chars: &Charset) -> Result<&'static str, String> {
    if pos.width != 7 || pos.height != 7 {
        return Err("Invalid Roi dimensions.".to_string());
    }

    let roi = Roi {
        img: img,
        pos: pos.clone(),
    };

    let bitmap = CharBitmap::from_roi(&roi)?;

    let char = chars.get(&bitmap);
    let Some(char) = char else {
        return Err("Did not find exact match".to_string());
    };

    Ok(*char)
}

/// Reads the characters from the field.
pub fn read_field(roi: &Roi, chars: &Charset) -> Result<String, String> {
    let pos = roi.pos();
    if pos.height != 7 || (pos.width + 1) % 8 != 0 {
        return Err("Input dimensions are incorrect.".to_string());
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

        let roi = Roi {
            img: roi.img(),
            pos: char_pos,
        };

        let char = read_char(roi.img(), &char_pos, chars)?;
        result.extend(char.chars());
    }

    Ok(result)
}
