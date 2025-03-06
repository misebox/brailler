use image::{imageops::resize, GenericImageView, GrayImage, Luma, ImageBuffer};
use image::imageops::FilterType;
use std::env;
use std::error::Error;


use imageproc::contrast::equalize_histogram;
use imageproc::edges::canny;

/// 入力画像をヒストグラム平坦化してコントラストを強調した後、
/// Canny エッジ検出を適用して輪郭を抽出した二値画像を返す。
///
/// - `input`: グレースケール画像
/// - 戻り値: エッジが白（255）、背景が黒（0）の二値画像
pub fn process_image(input: &GrayImage) -> GrayImage {
    // ヒストグラム平坦化でコントラスト強調
    let normalized = equalize_histogram(input);
    return normalized;
}


fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数から画像パスを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return Ok(());
    }
    let img_path = &args[1];

    // 画像を開いてグレースケール化
    let img = image::open(img_path)?.to_luma8();

    let img = process_image(&img);

    // 160x160 にリサイズ（FilterType::Nearest でピクセル感を維持）
    let img = resize(&img, 160, 160, FilterType::Nearest);

    // 2x4 のブロックを1セルとして扱うので、出力は80x40セルになる
    let cols = 80;
    let rows = 40;
    let mut output = String::new();

    // 各セル毎に処理
    for cell_y in 0..rows {
        for cell_x in 0..cols {
            let mut braille_value = 0u8;
            // ブロック内の各ピクセル (dx: 0..2, dy: 0..4)
            for dy in 0..4 {
                for dx in 0..2 {
                    let pixel_x = cell_x * 2 + dx;
                    let pixel_y = cell_y * 4 + dy;
                    // 画像は GrayImage (Luma<u8>) なので、ピクセル値は [0, 255]
                    let Luma([p]) = img.get_pixel(pixel_x as u32, pixel_y as u32);
                    // 閾値（128 未満なら「黒」とみなす）
                    if *p < 128 {
                        // Unicode ブライルパターンのドット番号対応：
                        // (0,0) → dot1 (0x01)
                        // (1,0) → dot4 (0x08)
                        // (0,1) → dot2 (0x02)
                        // (1,1) → dot5 (0x10)
                        // (0,2) → dot3 (0x04)
                        // (1,2) → dot6 (0x20)
                        // (0,3) → dot7 (0x40)
                        // (1,3) → dot8 (0x80)
                        let bit = match (dx, dy) {
                            (0, 0) => 0x01,
                            (1, 0) => 0x08,
                            (0, 1) => 0x02,
                            (1, 1) => 0x10,
                            (0, 2) => 0x04,
                            (1, 2) => 0x20,
                            (0, 3) => 0x40,
                            (1, 3) => 0x80,
                            _ => 0,
                        };
                        braille_value |= bit;
                    }
                }
            }
            // ブライルパターン文字は U+2800 + braille_value
            let braille_char = std::char::from_u32(0x2800 + braille_value as u32)
                .unwrap_or(' ');
            output.push(braille_char);
        }
        output.push('\n');
    }

    println!("{}", output);
    Ok(())
}

