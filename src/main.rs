use clap::Parser;
use image::{self, imageops::resize, GrayImage};
use image::imageops::FilterType;
use std::error::Error;
use std::env;
use std::sync::LazyLock;

/// 環境変数 key を bool として取得する関数
/// 指定されていなかったり、解釈できない場合は false を返します
fn get_env_bool(key: &str) -> bool {
    env::var(key).map(|s| {
        match s.to_lowercase().as_str() {
            "true" | "1" | "yes" => true,
            "false" | "0" | "no" => false,
            _ => false, // 不明な値は false とする
        }
    }).unwrap_or(false)
}
static MEASURE_TIME: LazyLock<bool> = LazyLock::new(|| get_env_bool("MEASURE_TIME"));

#[macro_export]
macro_rules! measure_time {
    ($expr:expr) => {{
        if *MEASURE_TIME {
            let start = std::time::Instant::now();
            let result = $expr;
            let duration = start.elapsed();
            eprintln!("Execution time ({}): {:?}", stringify!($expr), duration);
            result
        } else {
            $expr
        }
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
    if args.verbose {
        eprintln!("{:?}", args);
    }

    // 入力画像ファイルパスとサイズ
    let img_path = &args.input;
    let args_size = args.size;
    let (mut cols, mut rows) = (args_size.0, args_size.1);

    let img: GrayImage = measure_time!(image::open(img_path)?.to_luma8());

    let (w, h) = img.dimensions();
    if args.verbose {
        eprintln!("Image size: {}x{}", w, h);
    }
    let ratio = w as f32 / h as f32 * 2f32;
    if args.verbose {
        eprintln!("Ratio: {}", ratio);
    }
    if cols == 0 && rows == 0 {
        cols = 60;  // Default cols
    }
    if cols == 0 {
        cols = (rows as f64 * ratio as f64) as u32;
    } else if rows == 0 {
        rows = (cols as f64 / ratio as f64) as u32;
    }
    if args.verbose {
        eprintln!("Cols: {}, Rows: {}", cols, rows);
    }


    let img = measure_time!(preprocess_image(
        &img,
        args.contrast,
        args.invert,
    ));

    // リサイズしてキャンバスに貼り付け
    let (width, height) = (cols * 2, rows * 4);
    let img = measure_time!(resize(&img, width, height, FilterType::Nearest));

    // ピクセルを二値化する
    let img = measure_time!(binarize(&img, args.binarize));

    let output = measure_time!(generate_braille(&img, cols, rows));

    println!("{}", output);
    Ok(())
}
