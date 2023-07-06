use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::Rgba;
use image::{DynamicImage, GrayImage, RgbImage};
use imageproc::contrast::threshold;
use imageproc::drawing::draw_hollow_rect;
use imageproc::rect::Rect;
use pkmn::{create_char_bitmaps, match_field};
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

    let known_chars = create_char_bitmaps();

    // let window_initial = create_window("Initial", Default::default()).unwrap();
    // let window_grey = create_window("Greyscale", Default::default()).unwrap();
    // let window_threshold = create_window("Threshold", Default::default()).unwrap();
    // let window_erode = create_window("Erode", Default::default()).unwrap();
    let window_gameboy = create_window("GameBoy", Default::default()).unwrap();
    let window_roi = create_window("Region of Interest", Default::default()).unwrap();
    let window_debug = create_window("Debug", Default::default()).unwrap();

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
        //             panic!("Error: '{}'", error);
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

        let screen_candidates = pkmn::find_screen_candidates(&contours);

        let largest_candidate = screen_candidates
            .iter()
            .max_by_key(|rect| rect.width() * rect.height());

        let mut found_screen = false;

        let image_screen = match largest_candidate {
            Some(r) => {
                let image_screen = image_initial.clone().crop(
                    r.left() as u32 - erode_size as u32,
                    r.top() as u32 - erode_size as u32,
                    r.width() + 2 * erode_size as u32,
                    r.height() + 2 * erode_size as u32,
                );
                found_screen = true;
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

        let mut img_screen_small = image_screen.resize_exact(160, 144, FilterType::Nearest);

        let field_width: u32 = 23;
        let field_height: u32 = 7;

        let x_pkmn_no: u32 = 24;
        let y_pkmn_no: u32 = 56;
        let img_pkmn_no = img_screen_small.crop(x_pkmn_no, y_pkmn_no, field_width, field_height);

        // window_debug
        //     .set_image("Debug", img_pkmn_no.clone())
        //     .unwrap();

        // let pkmn_no = match_field(img_pkmn_no, &known_chars).unwrap();
        // println!("No: '{}'", pkmn_no);

        let x_level: u32 = 120;
        let y_level: u32 = 16;
        let img_level = img_screen_small.crop(x_level, y_level, field_width, field_height);
        let level = match_field(img_level, &known_chars).unwrap();
        println!("level: '{}'", level);

        let x_hp: u32 = 150 - field_width + 1;
        let y_hp: u32 = 39 - field_height;
        let img_hp = img_screen_small.crop(x_hp, y_hp, field_width, field_height);
        let hp = match_field(img_hp, &known_chars).unwrap();
        println!("hp: '{}'", hp);

        let x_attack: u32 = 70 - field_width + 1;
        let y_attack: u32 = 87 - field_height;
        let img_attack = img_screen_small.crop(x_attack, y_attack, field_width, field_height);
        let attack = match_field(img_attack, &known_chars).unwrap();
        println!("attack: '{}'", attack);

        let x_defense: u32 = 70 - field_width + 1;
        let y_defense: u32 = 103 - field_height;
        let img_defense = img_screen_small.crop(x_defense, y_defense, field_width, field_height);
        let defense = match_field(img_defense, &known_chars).unwrap();
        println!("defense: '{}'", defense);

        let x_speed: u32 = 70 - field_width + 1;
        let y_speed: u32 = 119 - field_height;
        let img_speed = img_screen_small.crop(x_speed, y_speed, field_width, field_height);
        let speed = match_field(img_speed, &known_chars).unwrap();
        println!("speed: '{}'", speed);

        let x_special: u32 = 70 - field_width + 1;
        let y_special: u32 = 135 - field_height;
        let img_special = img_screen_small.crop(x_special, y_special, field_width, field_height);
        let special = match_field(img_special, &known_chars).unwrap();
        println!("special: '{}'", special);

        let img_roi = img_screen_small;

        let img_roi = draw_hollow_rect(
            &img_roi,
            Rect::at(x_pkmn_no as i32, y_pkmn_no as i32).of_size(field_width, field_height),
            Rgba([0, 255, 0, 255]),
        );

        let img_roi = draw_hollow_rect(
            &img_roi,
            Rect::at(x_level as i32, y_level as i32).of_size(field_width, field_height),
            Rgba([0, 255, 0, 255]),
        );

        let img_roi = draw_hollow_rect(
            &img_roi,
            Rect::at(x_hp as i32, y_hp as i32).of_size(field_width, field_height),
            Rgba([0, 255, 0, 255]),
        );

        let img_roi = draw_hollow_rect(
            &img_roi,
            Rect::at(x_attack as i32, y_attack as i32).of_size(field_width, field_height),
            Rgba([0, 255, 0, 255]),
        );

        let img_roi = draw_hollow_rect(
            &img_roi,
            Rect::at(x_defense as i32, y_defense as i32).of_size(field_width, field_height),
            Rgba([0, 255, 0, 255]),
        );

        let img_roi = draw_hollow_rect(
            &img_roi,
            Rect::at(x_speed as i32, y_speed as i32).of_size(field_width, field_height),
            Rgba([0, 255, 0, 255]),
        );

        let img_roi = draw_hollow_rect(
            &img_roi,
            Rect::at(x_special as i32, y_special as i32).of_size(field_width, field_height),
            Rgba([0, 255, 0, 255]),
        );

        window_roi.set_image("Stats", img_roi.clone()).unwrap();

        // Print keyboard events until Escape is pressed, then exit.
        // If the user closes the window, the channel is closed and the loop also exits.
        let time_wait = Instant::now();
        for event in window_roi.event_channel().unwrap() {
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
