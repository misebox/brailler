mod size;
mod utilities;

use std::io::Write;
use std::io;
use std::error::Error;
use clap::Parser;
use image::{self, GrayImage};
use braille::*;

use brailler::file_type;
use brailler::args;
use brailler::braille;
use brailler::scriptify;
use brailler::image_processing;

#[cfg(feature="video")]
use brailler::video;


# [cfg(not(feature="video"))]
pub fn process_video(ftype: file_type::FileType, img_path: &str, args: args::Args) {
    eprintln!("動画処理は無効です")
}
# [cfg(feature="video")]
pub fn process_video(ftype: file_type::FileType, img_path: &str, args: args::Args) {
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

    let avg_wait = std::time::Duration::from_secs_f32(1.0 / video_data.fps);

    if args.scriptify.is_empty() {
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
        // カンマ区切りの文字列に変換
        let output = video_data.frames.iter().map(
            |img|
                generate_braille(img, cols, rows))
                .collect::<Vec<_>>()
                .join(",\n");
        // スクリプト出力
        let wait_sec = (1.0 / video_data.fps) as f32;
        if let Ok(script) = scriptify::generate_bash_script_for_video(&output, wait_sec) {
            scriptify::save_script(&script, &args.scriptify)?;
            eprintln!("Script file is created: {}", args.scriptify);
        } else {
            eprintln!("Failed to generate script");
        }
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
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
        let img = image_processing::process_image(
            &img,
            cols,
            rows,
            args.contrast,
            args.invert,
            args.binarize,
        );

        let output = measure_time!(generate_braille(&img, cols, rows));

        if args.scriptify.is_empty() {
            println!("{}", output);
        } else {
            // スクリプト出力
            if let Ok(script) = scriptify::generate_bash_script_for_image(&output) {
                scriptify::save_script(&script, &args.scriptify)?;
                eprintln!("Script file is created: {}", args.scriptify);
            } else {
                eprintln!("Failed to generate script");
            }
        }
    } else if ftype == file_type::FileType::Video {
        // 動画処理
        process_video(ftype, &img_path, args);
    } else {
        eprintln!("Unsupported file type: {:?}", ftype);
        return Ok(());
    }

    Ok(())
}
