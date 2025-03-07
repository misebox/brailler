use image::{imageops::resize, GrayImage, Luma, ImageBuffer};
use image::imageops::FilterType;
use std::env;
use std::error::Error;


use imageproc::contrast::equalize_histogram;

use image::GenericImage;

pub fn invert_image(input: &GrayImage) -> GrayImage {
    let mut img = input.clone();
    for pixel in img.pixels_mut() {
        pixel[0] = 255 - pixel[0];
    }
    img
}

/// 入力画像の最小値と最大値を使って、各ピクセルを線形変換することで
/// 0〜255 の範囲に広げる（コントラストストレッチ）処理を行う関数
pub fn contrast_stretch(input: &GrayImage) -> GrayImage {
    // 画像全体の最小値と最大値を求める
    let (mut min_val, mut max_val) = (255u8, 0u8);
    for pixel in input.pixels() {
        let v = pixel[0];
        if v < min_val {
            min_val = v;
        }
        if v > max_val {
            max_val = v;
        }
    }

    // 最大値と最小値が同じ場合は（定数画像の場合）そのまま返す
    if min_val == max_val {
        return input.clone();
    }
    
    let range = max_val - min_val;
    
    // 新しい画像に対して各ピクセルを線形変換して設定する
    let mut output = input.clone();
    for pixel in output.pixels_mut() {
        // (pixel - min) / range * 255 を計算
        let normalized = ((pixel[0].saturating_sub(min_val)) as f32 * 255.0 / range as f32)
            .round() as u8;
        *pixel = Luma([normalized]);
    }
    
    output
}


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

pub fn save_image(img: &GrayImage, path: &str) {
    img.save(path).expect("画像の保存に失敗しました");
}

/// 画像を処理して、ブライル文字に変換する
///
/// - `input`: グレースケール画像
/// - 戻り値: ヒストグラム平坦化された画像
pub fn process_image(input: &GrayImage) -> GrayImage {
    // ヒストグラム平坦化でコントラスト強調
    let img = equalize_histogram(input);
    let img = invert_image(&img);
    let img = contrast_stretch(&img);
    // saveimage
    save_image(&img, "normalized.png");
    img
}

pub fn put_image_into_canvas(img: &GrayImage, width: u32, height: u32) -> GrayImage {

    // 指定サイズのキャンバスを白で作成
    let mut canvas: GrayImage = ImageBuffer::from_pixel(width, height, Luma([255u8]));

    // リサイズ画像をキャンバスに貼り付け
    canvas.copy_from(img, 0, 0)
          .expect("画像の貼り付けに失敗しました");

    canvas
}

// generate_braille 関数をタプルリストを使う形に変更
fn generate_braille(img: &GrayImage, cols: u32, rows: u32) -> String {
    let width = img.width();
    let buffer = img.as_raw();
    let mut output = String::with_capacity((cols * rows + rows) as usize);
    // タプル: (dx, dy, ブライルビット)
    let offsets = [
        (0, 0, 0x01),
        (1, 0, 0x08),
        (0, 1, 0x02),
        (1, 1, 0x10),
        (0, 2, 0x04),
        (1, 2, 0x20),
        (0, 3, 0x40),
        (1, 3, 0x80),
    ];
    for cell_y in 0..rows {
        let base_y = cell_y * 4;
        for cell_x in 0..cols {
            let base_x = cell_x * 2;
            let mut braille_value = 0u8;
            for &(dx, dy, bit) in offsets.iter() {
                let idx = ((base_y + dy) * width + base_x + dx) as usize;
                if buffer[idx] < 128 {
                    braille_value |= bit;
                }
            }
            output.push(std::char::from_u32(0x2800 + braille_value as u32).unwrap_or(' '));
        }
        output.push('\n');
    }
    output
}

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数から画像パスを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return Ok(());
    }

    let img_path = &args[1];
    let (cols, rows) = if args.len() == 4 {
        let _cols = args[2].parse::<u32>().unwrap();
        if _cols < 1 {
            eprintln!("Columns must be greater than 0");
            return Ok(());
        }
        if _cols % 2 != 0 {
            eprintln!("Columns must be even number");
            return Ok(());
        }
        let _rows = args[3].parse::<u32>().unwrap();
        if _rows < 1 {
            eprintln!("Rows must be greater than 0");
            return Ok(());
        }
        if _rows % 4 != 0 {
            eprintln!("Rows must be multiple of 4");
            return Ok(());
        }
        (_cols, _rows)
    } else {
        (80, 40)
    };

    // 画像を開いてグレースケール化
    let img = measure_time!(image::open(img_path)?.to_luma8());

    let img = measure_time!(process_image(&img));

    // 160x160 にリサイズ（FilterType::Nearest でピクセル感を維持）
    let (width, height) = (cols * 2, rows * 4);
    let img = measure_time!(resize(&img, width, height, FilterType::Nearest));

    // ブライル文字列生成処理を関数に切り出し
    let output = measure_time!(generate_braille(&img, cols, rows));

    println!("{}", output);
    Ok(())
}

