/// The inclusive bounding box of something.
/// The elements on the border are part of the thing.
#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
