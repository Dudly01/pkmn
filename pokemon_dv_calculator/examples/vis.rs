/// Helper example for developing Python debug scripts.
use image::io::Reader as ImageReader;
use pokemon_dv_calculator::{position::Position, roi::Roi};

#[show_image::main]
fn main() {
    let img_path = "../Yellow_summary_1.png";

    let img_dyn = ImageReader::open(img_path).unwrap().decode().unwrap();

    let img_buff = img_dyn.clone().to_rgb32f();

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
