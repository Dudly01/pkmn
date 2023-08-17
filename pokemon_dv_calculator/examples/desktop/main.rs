/// Finds the GameBoy on the primary display and shows the result in the terminal.
pub mod screen_capturer;

use crossterm::{
    cursor,
    style::Print,
    terminal::{self, Clear},
    ExecutableCommand, Result,
};
use image::DynamicImage;
use std::io::stdout;

use pokemon_dv_calculator as pkmn;

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout
        .execute(terminal::SetTitle("PKMN DV calc"))?
        .execute(cursor::Hide)?;

    let capturer = screen_capturer::ScreenCapturer::for_primary_display();
    let Ok(mut capturer) = capturer else {
        panic!("There was an error in capturing the primary display.");
    };

    loop {
        let img_screen = capturer.next_frame();
        let Ok(img_screen) = img_screen else {
            panic!("There was an error retrieving the display frame.")
        };
        let img_screen = DynamicImage::ImageRgb8(img_screen.clone());
        let scan_result = pkmn::utils::scan_img(img_screen);

        let text_output = match scan_result {
            Ok(text_output) => text_output,
            Err(error) => error,
        };

        stdout
            .execute(Clear(terminal::ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?;

        println!("{}", text_output);
    }
}
