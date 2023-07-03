use scrap::{Capturer, Display};
use show_image::{create_window, event, ImageInfo, ImageView};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

#[show_image::main]
fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        // Convert BGRA buffer into dense RGB array
        let mut dense_array: Vec<u8> = Vec::with_capacity(w * h * 3);
        let stride = buffer.len() / h;
        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                dense_array.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i]]);
            }
        }

        // Show original image
        let image = ImageView::new(ImageInfo::rgb8(w as u32, h as u32), &dense_array);

        let window = create_window("image", Default::default()).unwrap();
        window.set_image("image-001", &image).unwrap();

        // Print keyboard events until Escape is pressed, then exit.
        // If the user closes the window, the channel is closed and the loop also exits.
        for event in window.event_channel().unwrap() {
            if let event::WindowEvent::KeyboardInput(event) = event {
                println!("{:#?}", event);
                if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                    && event.input.state.is_pressed()
                {
                    break;
                }
            }
        }

        break;
    }
}
