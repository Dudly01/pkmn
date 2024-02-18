//! Helper example for developing Python debug scripts.

use core::{position::Position, roi::Roi};

#[show_image::main]
fn main() {
    const EXAMPLE_IMG: &[u8] = include_bytes!("../data/Yellow_summary_1.png"); // Compile-time file check

    let img_dyn = image::load_from_memory(EXAMPLE_IMG).expect("could not load Nicknaming_I.png");

    let img_buff = img_dyn.clone().to_rgb32f();

    let img_buff_luma = img_dyn.to_luma8();

    let width = img_buff.width();
    let height = img_buff.height();
    let img_buff_data = img_buff.as_raw();

    let img_buff_addr = img_buff_data.as_ptr();

    let pos_ndex = Position {
        x: 24,
        y: 56,
        width: 23,
        height: 7,
    };
    let roi = Roi {
        img: &img_dyn.to_luma8(),
        pos: pos_ndex,
    };

    println!("Done");
}
