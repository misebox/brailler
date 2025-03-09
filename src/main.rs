use clap::Parser;
use image::{self, imageops::resize, GrayImage};
use image::imageops::FilterType;
use std::error::Error;

#[macro_export]
macro_rules! measure_time {
    ($expr:expr) => {{
        let start = std::time::Instant::now();
        let result = $expr;
        let duration = start.elapsed();
        eprintln!("Execution time ({}): {:?}", stringify!($expr), duration);
        result
    }};
}

mod image_processing;
mod braille;
mod args;
mod dot_canvas;
mod position;
mod size;

use image_processing::*;
use braille::*;
use args::Args;
use dot_canvas::*;
use position::*;
use size::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    eprintln!("{:?}", args);

    // 入力画像ファイルパスとサイズ
    let img_path = &args.input;
    let args_size = args.size;
    let (mut cols, mut rows) = (args_size.0, args_size.1);

    let img: GrayImage = measure_time!(image::open(img_path)?.to_luma8());

    let (w, h) = img.dimensions();
    eprintln!("Image size: {}x{}", w, h);
    let ratio = w as f32 / h as f32 * 2f32;
    eprintln!("Ratio: {}", ratio);
    if cols == 0 && rows == 0 {
        cols = 60;  // Default cols
    }
    if cols == 0 {
        cols = (rows as f64 * ratio as f64) as u32;
    } else if rows == 0 {
        rows = (cols as f64 / ratio as f64) as u32;
    }
    eprintln!("Cols: {}, Rows: {}", cols, rows);


    let img = measure_time!(preprocess_image(
        &img,
        args.histogram,
        args.contrast,
        args.invert,
    ));

    // リサイズしてキャンバスに貼り付け
    let (width, height) = (cols * 2, rows * 4);
    let img = measure_time!(resize(&img, width, height, FilterType::Nearest));
    let img = measure_time!(binarize(&img, args.odith, args.fsdith, args.otsu));

    let output = measure_time!(generate_braille(&img, cols, rows));

    println!("{}", output);
    Ok(())
}
