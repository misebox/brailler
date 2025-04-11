use image::imageops::{resize, FilterType};
use image::{GrayImage, Luma, ImageBuffer, GenericImage};
use imageproc::contrast::equalize_histogram;
use imageproc::contrast::otsu_level;

use crate::args;
use crate::measure_time;


// 画像の色反転
pub fn invert_image(input: &GrayImage) -> GrayImage {
    let mut img = input.clone();
    for pixel in img.pixels_mut() {
        pixel[0] = 255 - pixel[0];
    }
    img
}

// コントラストストレッチ（Min-Max正規化）
pub fn contrast_stretch(input: &GrayImage) -> GrayImage {
    let (mut min_val, mut max_val) = (255u8, 0u8);
    for pixel in input.pixels() {
        let v = pixel[0];
        if v < min_val { min_val = v; }
        if v > max_val { max_val = v; }
    }
    if min_val == max_val { return input.clone(); }
    let range = max_val - min_val;
    let mut output = input.clone();
    for pixel in output.pixels_mut() {
        let normalized = ((pixel[0].saturating_sub(min_val)) as f32 * 255.0 / range as f32)
            .round() as u8;
        *pixel = Luma([normalized]);
    }
    output
}

// Ordered Dithering（4x4 Bayer行列）
pub fn ordered_dither(input: &GrayImage) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut output = GrayImage::new(width, height);
    const MATRIX: [[u8; 4]; 4] = [
        [ 0,  8,  2, 10],
        [12,  4, 14,  6],
        [ 3, 11,  1,  9],
        [15,  7, 13,  5],
    ];
    let matrix_size = 4;
    for y in 0..height {
        for x in 0..width {
            let intensity = input.get_pixel(x, y)[0];
            let i = (x as usize) % matrix_size;
            let j = (y as usize) % matrix_size;
            let threshold = (((MATRIX[j][i] as f32 + 0.5) / 16.0) * 255.0).round() as u8;
            let new_value = if intensity > threshold { 255 } else { 0 };
            output.put_pixel(x, y, Luma([new_value]));
        }
    }
    output
}

/// Floyd–Steinberg の誤差拡散法によるハーフトーン処理を行う関数
///
/// 入力のグレイスケール画像から、各画素を 0 または 255 に変換し、
/// 誤差を周囲に分散させることで、ディザリング（ハーフトーン）画像を作成します。
pub fn floyd_steinberg_dither(input: &GrayImage) -> GrayImage {
    let (width, height) = input.dimensions();
    let w = width as usize;
    let h = height as usize;
    
    // 入力画像の各ピクセル値を f32 に変換してバッファに展開
    let mut buffer: Vec<f32> = input.pixels().map(|p| p[0] as f32).collect();
    
    // 出力画像用バッファ
    let mut output = GrayImage::new(width, height);
    
    // 画素を左上から右下に向かって走査
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let old_value = buffer[idx];
            // しきい値は128.0。これを超えれば255、未満なら0
            let new_value = if old_value >= 128.0 { 255.0 } else { 0.0 };
            let error = old_value - new_value;
            
            // 出力画像に値をセット
            output.put_pixel(x as u32, y as u32, Luma([new_value as u8]));
            
            // 誤差を各隣接画素に分配する
            // 右: error * 7/16
            if x + 1 < w {
                buffer[y * w + (x + 1)] += error * 7.0 / 16.0;
            }
            // 左下: error * 3/16
            if x > 0 && y + 1 < h {
                buffer[(y + 1) * w + (x - 1)] += error * 3.0 / 16.0;
            }
            // 真下: error * 5/16
            if y + 1 < h {
                buffer[(y + 1) * w + x] += error * 5.0 / 16.0;
            }
            // 右下: error * 1/16
            if x + 1 < w && y + 1 < h {
                buffer[(y + 1) * w + (x + 1)] += error * 1.0 / 16.0;
            }
        }
    }
    
    output
}

/// 入力画像に大津の方法を適用して、最適な閾値で二値化する関数
pub fn binarize_with_otsu(input: &GrayImage) -> GrayImage {
    // 大津の方法で閾値を求める
    let threshold = otsu_level(input);
    
    // 求めた閾値で画像を二値化
    let mut output = input.clone();
    for pixel in output.pixels_mut() {
        // threshold未満なら黒、以上なら白
        pixel[0] = if pixel[0] < threshold { 0 } else { 255 };
    }
    
    output
}


#[allow(clippy::let_and_return)]
// 画像処理パイプライン
pub fn preprocess_image(
    input: &GrayImage,
    contrast_opt: args::ContrastOption,
    invert_opt: bool,
) -> GrayImage {
    let img = input.clone();
    let img = if contrast_opt == args::ContrastOption::Stretch { contrast_stretch(&img) } else { img };
    let img = if contrast_opt == args::ContrastOption::Equalize { equalize_histogram(&img) } else { img };
    let img = if !invert_opt { invert_image(&img) } else { img };
    img
}

// 画像処理パイプライン
#[allow(clippy::let_and_return)]
pub fn binarize(
    input: &GrayImage,
    binarize_opt: args::BinarizeOption,
) -> GrayImage {

    let img = input.clone();
    let img = if binarize_opt == args::BinarizeOption::Odith {
        ordered_dither(&img)
    } else if binarize_opt == args::BinarizeOption::Fsdith {
        floyd_steinberg_dither(&img)
    } else if binarize_opt == args::BinarizeOption::Otsu {
        binarize_with_otsu(&img)
    } else {
        img
    };
    img
}

pub fn process_image(
    img: &GrayImage,
    cols: u32,
    rows: u32,
    contrast_opt: args::ContrastOption,
    invert_opt: bool,
    binarize_opt: args::BinarizeOption,
) -> GrayImage {
    let img = measure_time!(preprocess_image(&img, contrast_opt, invert_opt,));
    // リサイズしてキャンバスに貼り付け
    let (width, height) = (cols * 2, rows * 4);
    let img = measure_time!(resize(&img, width, height, FilterType::Nearest));
    // ピクセルを二値化する
    let img = measure_time!(binarize(&img, binarize_opt));
    img
}


// 画像をファイルに保存
pub fn save_image(img: &GrayImage, path: &str) {
    img.save(path).expect("画像の保存に失敗しました");
}

// キャンバスに画像を貼り付け
pub fn put_image_into_canvas(img: &GrayImage, width: u32, height: u32) -> GrayImage {
    let mut canvas: GrayImage = ImageBuffer::from_pixel(width, height, Luma([255u8]));
    canvas.copy_from(img, 0, 0).expect("画像の貼り付けに失敗しました");
    canvas
}
