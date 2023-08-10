/// Helper example for developing Python debug scripts.

use image::io::Reader as ImageReader;

#[show_image::main]
fn main() {
    let img_path = "../Yellow_summary_1.png";

    let img_dyn = ImageReader::open(img_path).unwrap().decode().unwrap();

    let img_buff = img_dyn.to_rgb32f();

    let width = img_buff.width();
    let height = img_buff.height();
    let img_buff_data = img_buff.as_raw();

    let img_buff_addr = img_buff_data.as_ptr();

    println!("Done");
}
