use core::{gameboy::contour_to_position, position::Position};

/// Finds the GameBoy on the "screenshot.png", shows images in windows, shows stats in terminal.
use image::{io::Reader as ImageReader, GrayImage, Luma};
use imageproc::{contrast::threshold_mut, rect::Rect};
use show_image;

/// Returns the position of RBY Game Boy if found.
fn locate_screen_rby(img: &GrayImage) -> Option<Position> {

    let mut img = img.clone();
    *img.get_pixel_mut(0, 0) = Luma([0]);

    let contours = imageproc::contours::find_contours::<i32>(&img);

    let width_orig = 160;
    let height_orig = 144;

    let target_ratio = width_orig as f32 / height_orig as f32;
    let tolerance = 0.01;

    let mut candidates = Vec::with_capacity(4);

    for contour in &contours {
        let pos = contour_to_position(contour).unwrap();

        if pos.width < width_orig || pos.height < height_orig {
            continue; // Smaller than original size
        }

        let ratio = pos.width as f32 / pos.height as f32;
        if (ratio - target_ratio).abs() > tolerance {
            continue; // Not within tolerance
        }

        candidates.push(pos);
    }

    let largest_pos = candidates.iter().max_by_key(|p| p.width * p.height);

    let Some(largest_pos) = largest_pos else {
        return None; // There were no candidates to find the biggest of
    };

    Some(largest_pos.clone())
}

/// Returns the position of GSC Game Boy if found.
fn locate_screen_gsc(img: &GrayImage) -> Option<Position> {
    let contours = imageproc::contours::find_contours::<i32>(&img);

    let target_ratio = 160.0 / 62.0;
    let tolerance = 0.01;

    let mut candidates = Vec::with_capacity(4);

    for contour in &contours {
        let pos = contour_to_position(contour).unwrap();

        if pos.width < 160 || pos.height < 62 {
            continue; // Smaller than original size
        }

        let ratio = pos.width as f32 / pos.height as f32;
        if (ratio - target_ratio).abs() > tolerance {
            continue; // Not within tolerance
        }

        candidates.push(pos);
    }

    let largest_pos = candidates.iter().max_by_key(|p| p.width * p.height);

    let Some(largest_pos) = largest_pos else {
        return None; // There were no candidates to find the biggest of
    };

    Some(largest_pos.clone())
}

#[show_image::main]
fn main() -> Result<(), String> {
    let windows = [
        show_image::create_window("Main 1", Default::default()).unwrap(),
        show_image::create_window("2", Default::default()).unwrap(),
        show_image::create_window("3", Default::default()).unwrap(),
        show_image::create_window("4", Default::default()).unwrap(),
        show_image::create_window("5", Default::default()).unwrap(),
    ];

    use std::env;
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let paths = [
        "data/Yellow_summary_1.png",
        "data/Yellow_summary_2.png",
        "data/Crystal_summary_1.png",
        "data/Crystal_summary_2.png",
        "data/Crystal_summary_3.png",
    ];

    let imgs = [
        ImageReader::open(paths[0]).unwrap().decode().unwrap(),
        ImageReader::open(paths[1]).unwrap().decode().unwrap(),
        ImageReader::open(paths[2]).unwrap().decode().unwrap(),
        ImageReader::open(paths[3]).unwrap().decode().unwrap(),
        ImageReader::open(paths[4]).unwrap().decode().unwrap(),
    ];

    if paths.len() != imgs.len() || paths.len() != windows.len() {
        panic!("Mismatching number of paths and images.")
    }

    for threshold in (0..255).step_by(10) {
        println!("Threshold: {}", threshold);

        for i in 0..5 {
            let mut img = imgs[i].to_luma8();
            threshold_mut(&mut img, threshold);

            print!("{i} ");

            let rby_screen_pos = locate_screen_rby(&img);
            if let Some(screen_pos) = rby_screen_pos {
                let rect = Rect::at(screen_pos.x as i32, screen_pos.y as i32)
                    .of_size(screen_pos.width, screen_pos.height);

                let color = Luma([128]);

                imageproc::drawing::draw_hollow_rect_mut(&mut img, rect, color);

                print!(
                    "RBY x:{} y:{} w:{} h:{}",
                    screen_pos.x, screen_pos.y, screen_pos.width, screen_pos.height
                );
            }

            let gsc_screen_pos = locate_screen_gsc(&img);
            if let Some(screen_pos) = gsc_screen_pos {
                let height = screen_pos.width as f32 * 144.0 / 160.0;
                let height = height as i32;

                let rect = Rect::at(screen_pos.x as i32, screen_pos.y as i32)
                    .of_size(screen_pos.width, height as u32);

                let color = Luma([128]);

                imageproc::drawing::draw_hollow_rect_mut(&mut img, rect, color);

                print!(
                    "GSC x:{} y:{} w:{} h:{}",
                    screen_pos.x, screen_pos.y, screen_pos.width, height
                );
            }

            println!("");

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
