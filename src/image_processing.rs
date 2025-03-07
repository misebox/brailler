use image::{GrayImage, Luma, ImageBuffer, GenericImage};
use imageproc::contrast::equalize_histogram;

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

// 画像処理パイプライン
pub fn process_image(input: &GrayImage, histogram: bool, contrast: bool, invert: bool, dither: bool) -> GrayImage {
    let img = if histogram {
        equalize_histogram(input)
    } else {
        input.clone()
    };
    let img = if contrast { contrast_stretch(&img) } else { img };
    let img = if invert { invert_image(&img) } else { img };
    let img = if dither { ordered_dither(&img) } else { img };
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
