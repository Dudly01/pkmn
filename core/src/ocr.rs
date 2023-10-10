use crate::char::{CharBitmap, Charset};
use crate::position::Position;
use crate::roi::Roi;
use image::GrayImage;

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
