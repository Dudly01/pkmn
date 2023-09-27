//! Prototype app, locating the Gen I-II summary screens.
use core::gameboy::{search_screen_gsc, search_screen_rby};
use image::{io::Reader as ImageReader, Luma};
use imageproc::{contrast::threshold_mut, rect::Rect};
use show_image;

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
            *img.get_pixel_mut_checked(0, 0).unwrap() = Luma([0]);
            let contours = imageproc::contours::find_contours::<i32>(&img);

            print!("{i} ");

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
