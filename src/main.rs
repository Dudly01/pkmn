use image::{DynamicImage, GrayImage, RgbImage};
use image::io::Reader as ImageReader;
use imageproc::contrast::threshold;
use scrap::{Capturer, Display};
use show_image::{create_window, event};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::{Duration, Instant};

#[show_image::main]
fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    let window_initial = create_window("Initial", Default::default()).unwrap();
    let window_grey = create_window("Greyscale", Default::default()).unwrap();
    let window_threshold = create_window("Threshold", Default::default()).unwrap();

    loop {
        // // Wait until there's a frame.
        // let buffer = match capturer.frame() {
        //     Ok(buffer) => buffer,
        //     Err(error) => {
        //         if error.kind() == WouldBlock {
        //             // Keep spinning.
        //             thread::sleep(one_frame);
        //             continue;
        //         } else {
        //             panic!("Error: {}", error);
        //         }
        //     }
        // };

        // // Convert BGRA buffer into dense RGB array
        // let mut raw_pixels: Vec<u8> = Vec::with_capacity(w * h * 3);
        // let stride = buffer.len() / h;
        // for y in 0..h {
        //     for x in 0..w {
        //         let i = stride * y + 4 * x;
        //         raw_pixels.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i]]);
        //     }
        // }

        // Show initial image
        // let image_initial = RgbImage::from_raw(w as u32, h as u32, raw_pixels).unwrap();

        let image_initial = ImageReader::open("screenshot.png").unwrap().decode().unwrap();

        window_initial
            .set_image("image-001", image_initial.clone())
            .unwrap();

        // Covert to greyscale
        let image_gray: GrayImage = image_initial.into_luma8();
        window_grey.set_image("Grey", image_gray.clone()).unwrap();

        // Threhsold to find white section
        let image_binary = threshold(&image_gray, 200);
        window_threshold
            .set_image("Grey", image_binary.clone())
            .unwrap();

        // Print keyboard events until Escape is pressed, then exit.
        // If the user closes the window, the channel is closed and the loop also exits.
        let time_wait = Instant::now();
        for event in window_initial.event_channel().unwrap() {
            if let event::WindowEvent::KeyboardInput(event) = event {
                println!("{:#?}", event);
                if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                    && event.input.state.is_pressed()
                {
                    return;
                }
            }
            if time_wait.elapsed().as_millis() > 50 {
                break;
            }
        }
    }
}
