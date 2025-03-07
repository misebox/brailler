use clap::Parser;
use image::{imageops::resize, GrayImage, Luma, ImageBuffer};
use image::imageops::FilterType;
use std::str::FromStr;
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

/// 入力のグレイスケール画像に Ordered Dithering を適用し、
/// スクリーントーン風の 2 色（二値）画像に変換します。
///
/// 4x4 の Bayer 行列を使用して、各ピクセルに局所的なしきい値を与えます。
pub fn ordered_dither(input: &GrayImage) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut output = GrayImage::new(width, height);
    // 4x4 Bayer 行列（各要素は 0～15 の値）
    const MATRIX: [[u8; 4]; 4] = [
        [ 0,  8,  2, 10],
        [12,  4, 14,  6],
        [ 3, 11,  1,  9],
        [15,  7, 13,  5],
    ];
    let matrix_size = 4;
    // 各セルのしきい値は、(value + 0.5)/16 * 255 で求める
    for y in 0..height {
        for x in 0..width {
            let pixel = input.get_pixel(x, y);
            let intensity = pixel[0];
            let i = (x as usize) % matrix_size;
            let j = (y as usize) % matrix_size;
            let threshold = (((MATRIX[j][i] as f32 + 0.5) / 16.0) * 255.0).round() as u8;
            let new_value = if intensity > threshold { 255 } else { 0 };
            output.put_pixel(x, y, Luma([new_value]));
        }
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
fn process_image(input: &GrayImage, opts: &Args) -> GrayImage {
    // ヒストグラム平坦化でコントラスト強調
    let img = if opts.histogram {
        equalize_histogram(input)
    } else {
        input.clone()
    };
    let img = if opts.contrast {
        contrast_stretch(&img)
    } else {
        img
    };
    let img = if opts.invert{
        invert_image(&img)
    } else {
        img
    };
    let img = if opts.dither {
        ordered_dither(&img)
    } else {
        img
    };
    // saveimage
    save_image(&img, "processed.png");
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

#[derive(Debug, Clone)]
struct Size(u32, u32);

impl ToString for Size {
    fn to_string(&self) -> String {
        format!("{}x{}", self.0, self.1)
    }
}

impl FromStr for Size {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 'x' または ',' で分割
        let separators = ['x', ','];
        let parts: Vec<&str> = s.split(|c| separators.contains(&c)).collect();
        if parts.len() != 2 {
            return Err("サイズは WIDTHxHEIGHT または WIDTH,HEIGHT の形式で指定してください".into());
        }
        let width = parts[0].trim().parse::<u32>().map_err(|e| e.to_string())?;
        let height = parts[1].trim().parse::<u32>().map_err(|e| e.to_string())?;
        Ok(Size(width, height))
    }
}

/// 画像処理を行うプログラム
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 入力画像ファイルパス
    #[arg(value_name = "INPUT")]
    input: String,

    // /// 出力画像ファイルパス
    // #[arg(value_name = "OUTPUT")]
    // output: String,

    /// 出力画像の列数（横幅）と行数（縦幅）
    #[arg(short, long, default_value = "80x40")]
    size: Size,

    /// Ordered dither を適用する
    #[arg(long)]
    dither: bool,

    /// コントラストストレッチ（Min-Max 正規化）を適用する
    #[arg(long)]
    contrast: bool,

    /// 画像の色を反転する
    #[arg(long)]
    invert: bool,

    /// ヒストグラム平坦化を適用する
    #[arg(long)]
    histogram: bool,

    /// Otsu の方法による二値化を適用する
    #[arg(long)]
    otsu: bool,

}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // // 各フラグの状態を表示（デバッグ用）
    // eprintln!("Input: {}", &args.input);
    // // eprintln!("Output: {}", args.output);
    // eprintln!(
    //     "Apply size: {}, dither: {}, contrast: {}, invert: {}, histogram: {}, otsu: {}",
    //     args.size.to_string(),
    //     args.dither,
    //     args.contrast,
    //     args.invert,
    //     args.histogram,
    //     args.otsu,
    // );

    // 入力画像ファイルパス
    let img_path = &args.input;

    // 出力画像のサイズ
    let Size(cols, rows) = args.size;

    // 画像を開いてグレースケール化
    let img = measure_time!(image::open(img_path)?.to_luma8());

    let img = measure_time!(process_image(&img, &args));

    // 160x160 にリサイズ（FilterType::Nearest でピクセル感を維持）
    let (width, height) = (cols * 2, rows * 4);
    let img = measure_time!(resize(&img, width, height, FilterType::Nearest));

    // ブライル文字列生成処理を関数に切り出し
    let output = measure_time!(generate_braille(&img, cols, rows));

    println!("{}", output);
    Ok(())
}

