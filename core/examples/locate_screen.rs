//! A dev app used for creating the Game Boy screen-locating functionality.

use core::gameboy::{search_screen_gsc, search_screen_rby};
use image::{io::Reader as ImageReader, Luma};
use imageproc::{contrast::threshold_mut, rect::Rect};
use show_image::{self};
use std::io;

#[show_image::main]
fn main() -> Result<(), String> {
    // Relative to crate root
    let image_paths = [
        "data/Yellow_summary_1.png",
        "data/Yellow_summary_2.png",
        "data/Crystal_summary_1.png",
        "data/Crystal_summary_2.png",
        "data/Crystal_summary_3.png",
    ];

    if image_paths.len() == 0 {
        panic!("No images to load!");
    }

    let images: Vec<_> = image_paths
        .iter()
        .map(|&path| ImageReader::open(path).unwrap().decode().unwrap())
        .collect();

    let windows: Vec<_> = image_paths
        .iter()
        .enumerate()
        .map(|(i, &path)| {
            let window_title = format!("Image #{i}: '{path}'");
            show_image::create_window(&window_title, Default::default()).unwrap()
        })
        .collect();

    // Show original images
    println!("Showing original images");
    for i in 0..images.len() {
        let window = &windows[i];
        let image = images[i].clone();
        let path = &image_paths[i];
        let title = format!("Image #{i}: '{path}'");

        window.set_image(title, image).unwrap();
    }

    // Wait for user input
    loop {
        println!("Continue? [Y/n]");

        let mut user_answer = String::new();

        io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");

        let user_answer = user_answer.trim();

        match user_answer {
            "" | "Y" | "y" => break,
            "N" | "n" => return Ok(()),
            _ => continue,
        }
    }

    let threshold_values: Vec<u8> = (0..255).step_by(10).collect();

    for threshold_value in threshold_values {
        println!("Applying a binary threshold of {threshold_value}");
        println!("Found game versions and screen positions:");
        for i in 0..images.len() {
            let mut img = images[i].to_luma8();
            threshold_mut(&mut img, threshold_value);
            *img.get_pixel_mut_checked(0, 0).unwrap() = Luma([0]);
            let contours = imageproc::contours::find_contours::<i32>(&img);

            print!("  #{i}: ");

            let screen_candidates = search_screen_rby(&contours);
            let rby_screen_pos = screen_candidates.iter().max_by_key(|&p| p.width);
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

            let screen_candidates = search_screen_gsc(&contours);
            let gsc_screen_pos = screen_candidates.iter().max_by_key(|&p| p.width);
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

        // Wait for user input
        loop {
            println!("Continue? [Y/n]");

            let mut user_answer = String::new();

            io::stdin()
                .read_line(&mut user_answer)
                .expect("Failed to read line");

            let user_answer = user_answer.trim();

            match user_answer {
                "" | "Y" | "y" => break,
                "N" | "n" => return Ok(()),
                _ => continue,
            }
        }
    }

    println!("Done");
    Ok(())
}
