use criterion::{black_box, criterion_group, criterion_main, Criterion};

use image::io::Reader as ImageReader;
use pokemon_dv_calculator as pkmn;

fn scan_img(img_path: &str) -> Result<String, String> {
    let img = ImageReader::open(img_path).unwrap().decode().unwrap();
    let scan_result = pkmn::utils::scan_img(img);
    scan_result
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Summary Screen 1", |b| {
        b.iter(|| scan_img(black_box("../Yellow_summary_1.png")))
    });
    c.bench_function("Summary Screen 2", |b| {
        b.iter(|| scan_img(black_box("../Yellow_summary_2.png")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
