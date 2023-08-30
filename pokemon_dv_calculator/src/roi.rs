//! Region of Interest
//!
//! Helper class to ease iterating the pixels of a section of a greyscale image.

use crate::position::Position;
use image::{GrayImage, Luma};

/// The Region of Interest of a GreyImage.
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

        let Luma([pixel_intensity]) = self.roi.img.get_pixel(self.x, self.y);

        self.x += 1;
        if self.x == self.roi.img.width() {
            self.x = 0;
            self.y += 1;
        }

        Some(pixel_intensity)
    }
}
