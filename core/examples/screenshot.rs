//! Shows the specified image to the user and prints the result of the scan.

use core as pkmn;
use show_image;
use std::io;

#[show_image::main]
fn main() -> Result<(), String> {
    // File is checked at compile-time
    // Path is relative to the examples dir
    const EXAMPLE_IMG: &[u8] = include_bytes!("../data/Crystal_summary_1.png");
    let image = image::load_from_memory(EXAMPLE_IMG).expect("Failed to load image");

    let window_gameboy = show_image::create_window("Game Boy", Default::default()).unwrap();
    window_gameboy.set_image("Game Boy", image.clone()).unwrap();

    let scan_result = pkmn::utils::scan_img(image);
    let text_output = match scan_result {
        Ok(text_output) => text_output,
        Err(error) => error,
    };

    println!("{}", text_output);

    // Block till user reacts
    println!("Press Enter to exit.");
    let mut user_answer = String::new();
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    Ok(())
}
