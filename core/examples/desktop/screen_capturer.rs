use image::{Rgb, RgbImage};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::{thread, time::Duration};

/// A wrapper for simplifying acquiring the frame of the primary display.
pub struct ScreenCapturer {
    capturer: Capturer,
    width: i32,
    height: i32,
    img: RgbImage,
}

impl ScreenCapturer {
    pub fn for_primary_display() -> Result<ScreenCapturer, &'static str> {
        let Ok(display) = Display::primary() else {
            return Err("Could not find primary display");
        };

        let Ok(capturer) = Capturer::new(display) else {
            return Err("Could not begin capture");
        };

        let width = capturer.width() as i32;
        let height = capturer.height() as i32;

        let screen_capturer = ScreenCapturer {
            capturer: capturer,
            width: width,
            height: height,
            img: RgbImage::new(width as u32, height as u32),
        };

        Ok(screen_capturer)
    }

    /// Returns the next frame from the capturer.
    pub fn next_frame(&mut self) -> Result<&RgbImage, String> {
        let one_frame = Duration::from_secs_f32(1.0 / 60.0);
        loop {
            let buffer = match self.capturer.frame() {
                Ok(buffer) => buffer,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        // Keep spinning
                        thread::sleep(one_frame);
                        continue;
                    } else {
                        let msg = format!("Error: '{}'", error);
                        return Err(msg);
                    }
                }
            };

            // Convert BGRA buffer
            let stride = buffer.len() / self.height as usize;
            for y in 0..self.height as usize {
                for x in 0..self.width as usize {
                    let i = stride * y + 4 * x;
                    let pixel = Rgb([buffer[i + 2], buffer[i + 1], buffer[i]]);
                    self.img.put_pixel(x as u32, y as u32, pixel);
                }
            }

            return Ok(&self.img);
        }
    }
}
