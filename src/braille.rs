use image::GrayImage;

pub fn generate_braille(img: &GrayImage, cols: u32, rows: u32) -> String {
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
