use imageproc::contours::Contour;
use imageproc::rect::Rect;
use std::cmp::{max, min};

/// Returns the inclusive bounding box of a contour.
pub fn get_bounding_box(contour: &Contour<i32>) -> Result<Rect, &str> {
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

/// Finds the GameBoy screen candidates within the contours.
/// Candidates have a minimum size of 160x140 and a ratio of ~10:9.
pub fn find_screen_candidates(contours: &Vec<Contour<i32>>) -> Vec<Rect> {
    let target_ratio = 10.0 / 9.0;
    let tolerance = 0.01;

    let mut potential_rects: Vec<Rect> = Vec::with_capacity(8);
    for contour in contours {
        let bbox = get_bounding_box(&contour).unwrap();

        if bbox.width() < 160 || bbox.height() < 144 {
            continue; // Smaller than original resolution
        }

        let ratio = bbox.width() as f32 / bbox.height() as f32;
        if (ratio - target_ratio).abs() > tolerance {
            continue; // Not within tolerance
        }

        potential_rects.push(bbox);
    }
    potential_rects
}
