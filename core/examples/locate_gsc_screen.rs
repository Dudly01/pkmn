/// Finds the GameBoy on the "screenshot.png", shows images in windows, shows stats in terminal.
use image::io::Reader as ImageReader;
use imageproc::contrast::threshold_mut;
use show_image;

#[show_image::main]
fn main() -> Result<(), String> {
    let windows = [
        show_image::create_window("Main 1", Default::default()).unwrap(),
        show_image::create_window("2", Default::default()).unwrap(),
        show_image::create_window("3", Default::default()).unwrap(),
    ];

    use std::env;
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let paths = [
        "data/Crystal_summary_1.png",
        "data/Crystal_summary_2.png",
        "data/Crystal_summary_3.png",
    ];

    let imgs = [
        ImageReader::open(paths[0]).unwrap().decode().unwrap(),
        ImageReader::open(paths[1]).unwrap().decode().unwrap(),
        ImageReader::open(paths[2]).unwrap().decode().unwrap(),
    ];

    for threshold in (0..255).step_by(10) {

        println!("Threshold: {}", threshold);

        for i in 0..3 {
            let mut img = imgs[i].to_luma8();
            threshold_mut(&mut img, threshold);

            let window = &windows[i];
            window.set_image(i.to_string(), img).unwrap();
        }

        for event in windows[0].event_channel().unwrap() {
            if let show_image::event::WindowEvent::KeyboardInput(event) = event {
                if event.input.key_code == Some(show_image::event::VirtualKeyCode::Escape)
                    && event.input.state.is_pressed()
                {
                    return Ok(());
                }

                if event.input.key_code == Some(show_image::event::VirtualKeyCode::Space)
                    && event.input.state.is_pressed()
                {
                    break;
                }
            }
        }
    }

    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.

    Ok(())
}
