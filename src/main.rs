use args::Args;
use args::BinarizeOption;
use args::ContrastOption;
use clap::Parser;
use image::imageops::FilterType;
use image::{self, GrayImage, imageops::resize};
use std::env;
use std::error::Error;
use std::sync::LazyLock;
mod args;
mod braille;
mod dot_canvas;
pub mod file_type;
mod image_processing;
mod position;
mod size;
use braille::*;
use dot_canvas::*;
use image_processing::*;
use position::*;
use size::*;

/// 環境変数 key を bool として取得する関数
/// 指定されていなかったり、解釈できない場合は false を返します
fn get_env_bool(key: &str) -> bool {
    env::var(key)
        .map(|s| {
            match s.to_lowercase().as_str() {
                "true" | "1" | "yes" => true,
                "false" | "0" | "no" => false,
                _ => false, // 不明な値は false とする
            }
        })
        .unwrap_or(false)
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

#[allow(clippy::let_and_return)]
fn image_to_braille(
    img: GrayImage,
    cols: u32,
    rows: u32,
    contrast_opt: ContrastOption,
    invert_opt: bool,
    binarize_opt: BinarizeOption,
) -> String {
    let img = measure_time!(preprocess_image(&img, contrast_opt, invert_opt,));
    // リサイズしてキャンバスに貼り付け
    let (width, height) = (cols * 2, rows * 4);
    let img = measure_time!(resize(&img, width, height, FilterType::Nearest));
    // ピクセルを二値化する
    let img = measure_time!(binarize(&img, binarize_opt));
    let output = measure_time!(generate_braille(&img, cols, rows));
    output
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // 入力画像ファイルパスとサイズ
    let img_path = &args.input;
    let (mut cols, mut rows) = (args.size.0, args.size.1);
    // ファイル種別を判定
    let file_type = file_type::infer_type(img_path);
    if file_type != file_type::FileType::Image {
        eprintln!("Unsupported file type: {:?}", file_type);
        return Ok(());
    }

    let img: GrayImage = measure_time!(image::open(img_path)?.to_luma8());

    let (w, h) = img.dimensions();
    let ratio = w as f32 / h as f32 * 2f32;
    if cols == 0 && rows == 0 {
        cols = 60; // Default cols
    }
    if cols == 0 {
        cols = (rows as f64 * ratio as f64) as u32;
    } else if rows == 0 {
        rows = (cols as f64 / ratio as f64) as u32;
    }
    if args.verbose {
        eprintln!("{:?}", args);
        eprintln!("Input: {}", img_path);
        eprintln!("File type: {:?}", file_type);
        eprintln!("Image size: {}x{}", w, h);
        eprintln!("Specified Size: {:?}", args.size);
        eprintln!("Ratio: {}", ratio);
        eprintln!("Cols: {}, Rows: {}", cols, rows);
    }
    let output = image_to_braille(
        img,
        cols,
        rows,
        args.contrast,
        args.invert,
        args.binarize,
    );

    println!("{}", output);
    Ok(())
}
