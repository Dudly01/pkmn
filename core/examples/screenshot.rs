/// Finds the GameBoy on the "screenshot.png", shows images in windows, shows stats in terminal.
use image::io::Reader as ImageReader;
use show_image;

use core as pkmn;

#[show_image::main]
fn main() -> Result<(), String> {
    let window_gameboy = show_image::create_window("GameBoy", Default::default()).unwrap();

    let img_path = "../core/data/Yellow_summary_1.png";
    let image_initial = ImageReader::open(img_path).unwrap().decode().unwrap();

    window_gameboy
        .set_image("GameBoy", image_initial.clone())
        .unwrap();

    let scan_result = pkmn::utils::scan_img(image_initial);

    let text_output = match scan_result {
        Ok(text_output) => text_output,
        Err(error) => error,
    };

    println!("{}", text_output);

    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.
    // for event in window_gameboy.event_channel().unwrap() {
    //     if let show_image::event::WindowEvent::KeyboardInput(event) = event {
    //         println!("{:#?}", event);
    //         if event.input.key_code == Some(show_image::event::VirtualKeyCode::Escape)
    //             && event.input.state.is_pressed()
    //         {
    //             return Ok(());
    //         }
    //     }
    // }

    Ok(())
}
