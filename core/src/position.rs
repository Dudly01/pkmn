use imageproc::contours::Contour;
use std::cmp::{max, min};
use std::convert::TryFrom;

/// The inclusive bounding box of something.
///
/// The elements on the border are part of the thing. Therefore a single pixel
/// would contain the pixel coordinates and the height and width of 1.
#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl TryFrom<&Contour<i32>> for Position {
    type Error = &'static str;

    /// Finds the bounding box of the contour points.
    fn try_from(contour: &Contour<i32>) -> Result<Self, Self::Error> {
        if contour.points.len() < 1 {
            return Err("no points within contour");
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

        let pos = Position {
            x: x_min as u32,
            y: y_min as u32,
            width: width as u32,
            height: height as u32,
        };
        Ok(pos)
    }
}
