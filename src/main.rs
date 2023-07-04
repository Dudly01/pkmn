use image::io::Reader as ImageReader;
use image::{DynamicImage, GrayImage, RgbImage};
use imageproc::contrast::threshold;
use scrap::{Capturer, Display};
use show_image::{create_window, event};

use std::time::{Duration, Instant};

use pokemon_dv_calculator as pkmn;

#[show_image::main]
fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    // let window_initial = create_window("Initial", Default::default()).unwrap();
    // let window_grey = create_window("Greyscale", Default::default()).unwrap();
    // let window_threshold = create_window("Threshold", Default::default()).unwrap();
    // let window_erode = create_window("Erode", Default::default()).unwrap();
    let window_gameboy = create_window("GameBoy", Default::default()).unwrap();

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

        let image_initial = ImageReader::open("screenshot.png")
            .unwrap()
            .decode()
            .unwrap();

        let image_gray: GrayImage = image_initial.clone().into_luma8();

        let image_threshold = threshold(&image_gray, 200);

        let erode_size = 1;
        let image_erode = imageproc::morphology::erode(
            &image_threshold,
            imageproc::distance_transform::Norm::LInf,
            erode_size,
        );

        let contours = imageproc::contours::find_contours::<i32>(&image_erode);

        let potential_rects = pkmn::get_possible_screens(&contours);

        let largest_rect = potential_rects
            .iter()
            .max_by_key(|rect| rect.width() * rect.height());

        let image_screen = match largest_rect {
            Some(r) => {
                let image_screen = image_initial.clone().crop(
                    r.left() as u32 - erode_size as u32,
                    r.top() as u32 - erode_size as u32,
                    r.width() + 2 * erode_size as u32,
                    r.height() + 2 * erode_size as u32,
                );
                image_screen
            }
            None => {
                let rgb: RgbImage = RgbImage::new(160, 144);
                DynamicImage::ImageRgb8(rgb)
            }
        };

        window_gameboy
            .set_image("GameBoy", image_screen.clone())
            .unwrap();

        // Print keyboard events until Escape is pressed, then exit.
        // If the user closes the window, the channel is closed and the loop also exits.
        let time_wait = Instant::now();
        for event in window_gameboy.event_channel().unwrap() {
            if let event::WindowEvent::KeyboardInput(event) = event {
                println!("{:#?}", event);
                if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                    && event.input.state.is_pressed()
                {
                    return;
                }

                if event.input.key_code == Some(event::VirtualKeyCode::S)
                    && event.input.state.is_pressed()
                {
                    image_screen
                        .save("gameboy.png")
                        .expect("Could not save image");
                }
            }
            // if time_wait.elapsed().as_millis() > 50 {
            //     break;
            // }
        }
    }
}
