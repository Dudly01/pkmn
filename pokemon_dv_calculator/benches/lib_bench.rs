/// Benchmark for the lib.
use criterion::*;

use image::{io::Reader as ImageReader, GrayImage};
use pokemon_dv_calculator as pkmn;

const SUMMARY_SCREEN_1_PATH: &str = "../Yellow_summary_1.png";
const SUMMARY_SCREEN_2_PATH: &str = "../Yellow_summary_2.png";

fn locate_screen(c: &mut Criterion) {
    let mut group = c.benchmark_group("locate-screen");

    for scale in [1, 2, 4, 8].iter() {
        let id = format!("scale-{}x", scale);
        group.bench_function(id, |b| {
            let img = ImageReader::open(SUMMARY_SCREEN_1_PATH)
                .unwrap()
                .decode()
                .unwrap();
            let img = img.resize_exact(
                img.width() * scale,
                img.height() * scale,
                image::imageops::FilterType::Nearest,
            );
            b.iter(|| pkmn::gameboy::locate_screen(&img));
        });
    }

    group.finish();
}

/// Prepares the Gameboy screen image for the layout tests.
///
/// Loads the image from the given path, coverts it to greyscale, thresholds and lastly inverts it.
/// This is the currently acceptable format for the functions.
fn init_img_for_layout_tests(img_path: &str) -> GrayImage {
    let img = ImageReader::open(img_path).unwrap().decode().unwrap();
    let mut img = img.to_luma8();
    imageproc::contrast::threshold_mut(&mut img, 200);
    image::imageops::invert(&mut img);
    img
}

fn verify_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify-layout");
    group.significance_level(0.1).sample_size(1000);

    group.bench_function("summary-screen-1", |b| {
        let img = init_img_for_layout_tests(SUMMARY_SCREEN_1_PATH);
        let chars = pkmn::char::init_chars();
        let screen_layout = pkmn::gameboy::StatScreen1Layout::new();
        b.iter(|| screen_layout.verify_layout(&img, &chars));
    });
    group.bench_function("summary-screen-2", |b| {
        let img = init_img_for_layout_tests(SUMMARY_SCREEN_2_PATH);
        let chars = pkmn::char::init_chars();
        let screen_layout = pkmn::gameboy::StatScreen2Layout::new();
        b.iter(|| screen_layout.verify_layout(&img, &chars));
    });
    group.finish();
}

fn read_screen(c: &mut Criterion) {
    let mut group = c.benchmark_group("read-layout");

    group.bench_function("summary-screen-1", |b| {
        let img = init_img_for_layout_tests(SUMMARY_SCREEN_1_PATH);
        let chars = pkmn::char::init_chars();
        let screen_layout = pkmn::gameboy::StatScreen1Layout::new();
        b.iter(|| screen_layout.read_fields(&img, &chars));
    });
    group.bench_function("summary-screen-2", |b| {
        let img = init_img_for_layout_tests(SUMMARY_SCREEN_2_PATH);
        let chars = pkmn::char::init_chars();
        let screen_layout = pkmn::gameboy::StatScreen2Layout::new();
        b.iter(|| screen_layout.read_fields(&img, &chars));
    });
    group.finish();
}

fn ocr(c: &mut Criterion) {
    let mut group = c.benchmark_group("ocr-approaches");
    group.significance_level(0.1).sample_size(500);

    group.bench_function("summary-screen-1-new", |b| {
        let img = init_img_for_layout_tests(SUMMARY_SCREEN_1_PATH);
        let chars = pkmn::char::init_chars();
        let screen_layout = pkmn::gameboy::StatScreen1Layout::new();
        b.iter(|| screen_layout.read_fields(&img, &chars));
    });

    group.bench_function("summary-screen-1-old", |b| {
        let symbols = pkmn::ocr::create_symbol_bitmaps();
        let screen_layout = pkmn::gameboy::StatScreen1Layout::new();
        let img = ImageReader::open(SUMMARY_SCREEN_1_PATH)
            .unwrap()
            .decode()
            .unwrap();
        b.iter(|| screen_layout.read_content(&img, &symbols));
    });

    group.bench_function("summary-screen-2-new", |b| {
        let img = init_img_for_layout_tests(SUMMARY_SCREEN_2_PATH);
        let chars = pkmn::char::init_chars();
        let screen_layout = pkmn::gameboy::StatScreen2Layout::new();
        b.iter(|| screen_layout.read_fields(&img, &chars));
    });

    group.bench_function("summary-screen-2-old", |b| {
        let img = ImageReader::open(SUMMARY_SCREEN_2_PATH)
            .unwrap()
            .decode()
            .unwrap();
        let symbols = pkmn::ocr::create_symbol_bitmaps();
        let screen_layout = pkmn::gameboy::StatScreen2Layout::new();
        b.iter(|| screen_layout.read_content(&img, &symbols));
    });

    group.finish();
}

criterion_group!(benches, locate_screen, verify_layout, read_screen, ocr);
criterion_main!(benches);
