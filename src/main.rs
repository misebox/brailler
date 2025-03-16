mod args;
mod dot_canvas;
mod file_type;
mod position;
mod size;
mod braille;

use std::io::Write;
use std::{io, sync::LazyLock};
use std::env;
use std::error::Error;
use args::Args;
use clap::Parser;
use image::{self, GrayImage};
use braille::*;

use dot_canvas::*;
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
        if *$crate::MEASURE_TIME {
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

use image_processing::*;
mod image_processing;
mod video;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // 入力画像ファイルパスとサイズ
    let img_path = args.input.clone();
    // ファイル種別を判定
    let ftype = file_type::infer_type(&img_path);
    if ftype == file_type::FileType::Image {
        let img: GrayImage = measure_time!(image::open(img_path.clone())?.to_luma8());

        let (w, h) = img.dimensions();
        let ratio = w as f32 / h as f32 * 2f32;
        let (mut _cols, mut _rows) = (args.size.0, args.size.1);
        let (cols, rows) = convert_size(w, h, _cols, _rows);

        if args.verbose {
            eprintln!("{:?}", args);
            eprintln!("Input: {}", img_path);
            eprintln!("File type: {:?}", ftype);
            eprintln!("Image size: {}x{}", w, h);
            eprintln!("Ratio: {}", ratio);
            eprintln!("Project Size: {:?}", args.size);
            eprintln!("Cols: {}, Rows: {}", cols, rows);
        }
        let img = process_image(
            &img,
            cols,
            rows,
            args.contrast,
            args.invert,
            args.binarize,
        );

        let output = measure_time!(generate_braille(&img, cols, rows));
        println!("{}", output);
    } else if ftype == file_type::FileType::Video {
        // 動画の処理
        let video_data = video::load_frames(
            &img_path,
            args.clone(),
        )?;
        let (cols, rows) = (video_data.size.0 / 2, video_data.size.1 / 4);

        if args.verbose {
            eprintln!("{:?}", args);
            eprintln!("Input: {}", img_path);
            eprintln!("File type: {:?}", ftype);
            eprintln!("Ratio: {}", video_data.ratio);
            eprintln!("Specified Size: {:?}", args.size);
            eprintln!("Image size: {}", video_data.size);
            eprintln!("Image FPS: {}", video_data.fps);
            eprintln!("Cols: {}, Rows: {}", cols, rows);
        }
        // wait for key input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let avg_wait = std::time::Duration::from_secs_f32(1.0 / video_data.fps);
        for img in video_data.frames {
            let start = std::time::Instant::now();
            let output = measure_time!(generate_braille(&img, cols, rows));
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
            println!("{}", output);
            io::stdout().flush().unwrap();
            // sleep
            let elapsed= std::time::Instant::now().duration_since(start);
            if elapsed < avg_wait {
                std::thread::sleep(avg_wait - elapsed);
            }
        }

    } else {
        eprintln!("Unsupported file type: {:?}", ftype);
        return Ok(());
    }

    Ok(())
}
