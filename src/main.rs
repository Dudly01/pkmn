use image::io::Reader as ImageReader;
use image::{imageops, DynamicImage, GrayImage, ImageBuffer, RgbImage};
use imageproc::contours::Contour;
use imageproc::contrast::threshold;
use imageproc::rect::Rect;
use scrap::{Capturer, Display};
use show_image::{create_window, event};
use std::cmp::{max, min};
use std::ffi::c_float;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::{Duration, Instant};

/// Returns the inclusive bounding rectangle for the points of the contour.
fn get_bounding_box(contour: &Contour<i32>) -> Result<Rect, &str> {
    if contour.points.len() < 1 {
        return Err("Contour contains no points!");
    }

    let curr_point = &contour.points[0];
    let mut x_min = curr_point.x;
    let mut x_max = curr_point.x;
    let mut y_min = curr_point.y;
    let mut y_max = curr_point.y;

    for point in &contour.points[1..] {
        x_min = min(x_min, point.x);
        x_max = max(x_max, point.x);
        y_min = min(y_min, point.y);
        y_max = max(y_max, point.y);
    }

    let width = x_max - x_min + 1;
    let height = y_max - y_min + 1;
    let rectangle = Rect::at(x_min, y_min).of_size(width as u32, height as u32);
    Ok(rectangle)
}

fn get_possible_screens(contours: &Vec<Contour<i32>>) -> Vec<Rect> {
    // Find potential contours
    let mut potential_rects: Vec<Rect> = Vec::with_capacity(8);
    for contour in contours {
        let current_rect = get_bounding_box(&contour).unwrap();

        if current_rect.width() < 160 || current_rect.height() < 144 {
            continue; // Too small size
        }

        let target_ratio = 10.0 / 9.0;
        let width_height_ratio = current_rect.width() as f32 / current_rect.height() as f32;
        if (width_height_ratio - target_ratio).abs() > 0.01 {
            continue; // Ratio is not within tolerance
        }

        potential_rects.push(current_rect);
    }
    potential_rects
}

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
    let window_erode = create_window("Erode", Default::default()).unwrap();
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

        window_initial
            .set_image("image-001", image_initial.clone())
            .unwrap();

        // Covert to greyscale
        let image_gray: GrayImage = image_initial.clone().into_luma8();
        window_grey.set_image("Grey", image_gray.clone()).unwrap();

        // Threhsold to find white section
        let image_threshold = threshold(&image_gray, 200);
        window_threshold
            .set_image("Grey", image_threshold.clone())
            .unwrap();

        let erode_size = 1;
        let image_erode = imageproc::morphology::erode(
            &image_threshold,
            imageproc::distance_transform::Norm::LInf,
            erode_size,
        );
        window_erode
            .set_image("Eroded", image_erode.clone())
            .unwrap();

        // Find contours
        let contours = imageproc::contours::find_contours::<i32>(&image_erode);

        // Find potential contours
        let potential_rects = get_possible_screens(&contours);

        // Find biggest contour
        let largest_rect = potential_rects
            .iter()
            .max_by_key(|rect| rect.width() * rect.height());

        let image_screen = match largest_rect {
            Some(r) => {
                let image_screen = image_initial.clone().crop(
                    r.left() as u32 - erode_size as u32,
                    r.top() as u32 - erode_size as u32,
                    r.width() +  2 * erode_size as u32,
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
        for event in window_initial.event_channel().unwrap() {
            if let event::WindowEvent::KeyboardInput(event) = event {
                println!("{:#?}", event);
                if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                    && event.input.state.is_pressed()
                {
                    return;
                }
            }
            // if time_wait.elapsed().as_millis() > 50 {
            //     break;
            // }
        }
    }
}
