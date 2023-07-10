use image::{DynamicImage, Luma};
use imageproc::contrast::threshold;

use crate::utils::Position;

/// A bitmap of 7x7 pixels depicting the individual symbols in Pokemon RBY.
pub struct SymbolBitmap {
    pub vals: [u8; 49],
}

impl SymbolBitmap {
    /// Creates a bitmap from a lazy array. In a lazy array, 0 corresponds to
    /// the white background, everything else to the foreground (the letter).
    /// This is exact opposite of a binary image, where 255 correponds to the
    /// foreground and 0 to the background.
    pub fn from_lazy_array(arr: &[u8; 49]) -> SymbolBitmap {
        let x = arr
            .iter()
            .map(|&n| if n == 0 { 255 } else { 0 })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        SymbolBitmap { vals: x }
    }
}

/// Returns the symbol-bitmap pairs as two vectors.
/// The elements with the same index correpsond to one another.
pub fn create_symbol_bitmaps() -> (Vec<String>, Vec<SymbolBitmap>) {
    let mut symbols: Vec<String> = Vec::with_capacity(11);
    let mut bitmaps: Vec<SymbolBitmap> = Vec::with_capacity(11);

    let symbol = "0".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 1, 0, 0, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 0, 0, 1, 0, //
        0, 0, 1, 1, 1, 0, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "1".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 1, 1, 1, 1, 1, 1, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "2".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 0, 0, 0, 1, 1, 1, //
        0, 1, 1, 1, 1, 0, 0, //
        1, 1, 1, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 1, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "3".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
        0, 0, 1, 1, 1, 0, 0, //
        0, 0, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "4".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 1, 1, 1, 0, //
        0, 0, 1, 1, 1, 1, 0, //
        0, 1, 1, 0, 1, 1, 0, //
        1, 1, 0, 0, 1, 1, 0, //
        1, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "5".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        0, 0, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "6".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "7".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        1, 1, 1, 1, 1, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 0, 0, 0, 1, 1, 0, //
        0, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 0, 0, 0, //
        0, 0, 1, 1, 0, 0, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "8".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = "9".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 1, 1, 1, 1, 1, 0, //
        1, 1, 0, 0, 0, 1, 1, //
        1, 1, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 1, //
        0, 0, 0, 0, 0, 1, 1, //
        0, 1, 1, 1, 1, 1, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    let symbol = " ".to_string();
    let bitmap = SymbolBitmap::from_lazy_array(&[
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, //
    ]);
    symbols.push(symbol);
    bitmaps.push(bitmap);

    (symbols, bitmaps)
}

/// Reads the symbol on a 7x7 image.
/// Uses a naive matching algorithm, where the bitmap with the least difference
/// is chosen as match.
pub fn read_symbol(
    img: DynamicImage,
    symbol_bitmaps: &(Vec<String>, Vec<SymbolBitmap>),
) -> Result<String, &'static str> {
    if img.width() != 7 || img.height() != 7 {
        return Err("Mismatching dimensions for image and bitmap.");
    }

    let img_grey = &img.to_luma8();
    let img_binary = threshold(&img_grey, 200);

    let mut diff_counts: Vec<i32> = Vec::with_capacity(symbol_bitmaps.0.len());
    for bitmap in &symbol_bitmaps.1 {
        let mut current_diffs = 0;
        for (a, Luma([b])) in bitmap.vals.iter().zip(img_binary.pixels()) {
            if *a != *b {
                current_diffs += 1;
            }
        }
        diff_counts.push(current_diffs)
    }

    let min_index = diff_counts
        .iter()
        .enumerate()
        .min_by_key(|&(_, &value)| value)
        .map(|(index, _)| index)
        .unwrap();

    let best_match = symbol_bitmaps.0[min_index].clone();

    Ok(best_match)
}

/// Reads the symbols on a 7nx7 image, where n is a positive integer,
/// The text is returned as-is, without cleaning.
pub fn read_line(
    img: &DynamicImage,
    symbol_bitmaps: &(Vec<String>, Vec<SymbolBitmap>),
) -> Result<String, &'static str> {
    if img.height() != 7 || (img.width() + 1) % 8 != 0 {
        return Err("Input dimensions are incorrect.");
    }

    let symbol_count = (img.width() + 1) / 8;
    let mut symbols = Vec::with_capacity(symbol_count as usize);

    for i in 0..symbol_count {
        let x = i * (7 + 1);
        let y = 0;
        let width = 7;
        let height = 7;

        let img_symbol = img.clone().crop(x, y, width, height);
        let symbol = read_symbol(img_symbol, symbol_bitmaps).unwrap();
        symbols.push(symbol);
    }

    let line: String = symbols.into_iter().collect();

    Ok(line)
}

/// Reads the symbols on the section of the image.
/// The section is expected to contain one line of text.
/// The text is returned as-is, without cleaning.
pub fn read_image_section(
    img: &DynamicImage,
    pos: &Position,
    symbol_bitmaps: &(Vec<String>, Vec<SymbolBitmap>),
) -> Result<String, &'static str> {
    if pos.height != 7 || (pos.width + 1) % 8 != 0 {
        return Err("Incorrect position dimensions.");
    }

    if pos.x + pos.width >= img.width() || pos.y + pos.height >= img.height() {
        return Err("Section is outside of image boundaries.");
    }

    let img_section = img.clone().crop(pos.x, pos.y, pos.width, pos.height);
    let symbols = read_line(&img_section, symbol_bitmaps);
    symbols
}