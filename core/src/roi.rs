use crate::position::Position;
use image::{GrayImage, Luma};

/// The Region of Interest (ROI) of a image::GreyImage.
pub struct Roi<'a> {
    pub img: &'a GrayImage,
    pub pos: Position,
}

impl<'a> Roi<'a> {
    pub fn iter(&self) -> RoiIter<'_> {
        RoiIter {
            roi: self,
            x: 0,
            y: 0,
        }
    }

    /// Returns a reference to the grayscale image.
    pub fn img(&self) -> &'a GrayImage {
        self.img
    }

    /// Returns the section of the grayscale image that is of interest.
    pub fn pos(&self) -> &Position {
        &self.pos
    }
}

/// The pixel iterator of a Roi.
pub struct RoiIter<'a> {
    roi: &'a Roi<'a>,
    x: u32,
    y: u32,
}

impl<'a> Iterator for RoiIter<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.roi.pos.height {
            return None;
        }

        let offset_x = &self.roi.pos.x;
        let offset_y = &self.roi.pos.y;

        let Luma([pixel_intensity]) = self.roi.img.get_pixel(offset_x + self.x, offset_y + self.y);

        self.x += 1;
        if self.x == self.roi.pos.width {
            self.x = 0;
            self.y += 1;
        }

        Some(pixel_intensity)
    }
}

#[cfg(test)]
mod tests {
    use image::GrayImage;

    use super::Roi;
    use crate::position::Position;

    #[test]
    fn roi_iteration() {
        // Arrange
        let width = 4;
        let height = 3;
        let mut data = Vec::<u8>::with_capacity(width * height);
        for i in 0..(width * height) {
            data.push(i as u8);
        }
        let img = GrayImage::from_raw(width as u32, height as u32, data).unwrap();
        let pos = Position {
            x: 1,
            y: 2,
            width: 2,
            height: 1,
        };
        let roi = Roi {
            img: &img,
            pos: pos,
        };
        let expected_pixels: Vec<u8> = vec![9, 10];

        // Act
        let pixels: Vec<u8> = roi.iter().cloned().collect();

        // Assert
        assert_eq!(pixels, expected_pixels);
    }
}
