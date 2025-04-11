
pub struct Position {
    pub x: i32,
    pub y: i32,
}
/// 各dotをu8で表現するキャンバス。各要素は 0 (off) または 1 (on) を持つ。
pub struct DotCanvas {
    width: usize,
    height: usize,
    data: Vec<u8>, // 各dotが1バイト。メモリ効率は低いがアクセスは高速。
}

impl DotCanvas {
    /// 新しいキャンバスを作成。すべてのdotは初期状態0（off）。
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    /// 指定した (x, y) のdotに値を設定する。valueは0か1とする。
    pub fn set(&mut self, x: i32, y: i32, value: u8) {
        assert!(0 <= x && x < self.width as i32 && y < self.height as i32, "Index out of bounds");
        self.data[(y * self.width as i32 + x) as usize] = value;
    }

    /// 指定した (x, y) のdotの値を取得する。
    pub fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width && y < self.height, "Index out of bounds");
        self.data[y * self.width + x]
    }

    pub fn draw_line(&mut self, p1: Position, p2: Position) {
        let dx = p2.x as i32 - p1.x as i32;
        let dy = p2.y as i32 - p1.y as i32;
        let steps = dx.abs().max(dy.abs());
        for i in 0..=steps {
            let x = p1.x as i32 + dx * i / steps;
            let y = p1.y as i32 + dy * i / steps;
            self.set(x, y, 1);
        }
    }
    pub fn draw_rect(&mut self, p1: Position, p2: Position) {
        for x in p1.x..=p2.x {
            self.set(x, p1.y, 1);
            self.set(x, p2.y, 1);
        }
        for y in p1.y..=p2.y {
            self.set(p1.x, y, 1);
            self.set(p2.x, y, 1);
        }
    }
    pub fn draw_circle(&mut self, p: Position, radius: i32) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;
        while x <= y {
            self.set(p.x + x, p.y + y, 1);
            self.set(p.x + x, p.y - y, 1);
            self.set(p.x - x, p.y + y, 1);
            self.set(p.x - x, p.y - y, 1);
            self.set(p.x + y, p.y + x, 1);
            self.set(p.x + y, p.y - x, 1);
            self.set(p.x - y, p.y + x, 1);
            self.set(p.x - y, p.y - x, 1);
            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }
    pub fn fill_rect(&mut self, p1: Position, p2: Position) {
        for y in p1.y..=p2.y {
            for x in p1.x..=p2.x {
                self.set(x, y, 1);
            }
        }
    }
    pub fn fill_circle(&mut self, center: Position, radius: i32) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;
        while x <= y {
            self.set(center.x + x, center.y + y, 1);
            self.set(center.x + x, center.y - y, 1);
            self.set(center.x - x, center.y + y, 1);
            self.set(center.x - x, center.y - y, 1);
            self.set(center.x + y, center.y + x, 1);
            self.set(center.x + y, center.y - x, 1);
            self.set(center.x - y, center.y + x, 1);
            self.set(center.x - y, center.y - x, 1);
            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }
    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x as i32, y as i32, 0);
            }
        }
    }
    pub fn clone(&self) -> DotCanvas {
        let mut new_canvas = DotCanvas::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                new_canvas.set(x as i32, y as i32, self.get(x, y));
            }
        }
        new_canvas
    }
    /// 2x4ドット毎にグループ化してUnicode Braille文字に変換する。
    /// 出力はキャンバスのBrailleパターン文字列。
    pub fn to_braille(&self) -> String {
        // Brailleセルは2x4ドット、キャンバス全体はセル数に換算
        let cell_cols = self.width / 2;
        let cell_rows = self.height / 4;
        let mut output = String::new();

        for cell_y in 0..cell_rows {
            for cell_x in 0..cell_cols {
                // 各Brailleセルは8ビット（各ドットのオン/オフ）で表現
                let mut cell: u8 = 0;
                for dy in 0..4 {
                    for dx in 0..2 {
                        let x = cell_x * 2 + dx;
                        let y = cell_y * 4 + dy;
                        let dot = self.get(x, y);
                        // dotがオンの場合、対応するビットをセットする。
                        // 以下は一般的なBrailleの点番号のマッピング例:
                        let bit = match (dx, dy) {
                            (0, 0) => 0x01, // dot1
                            (0, 1) => 0x02, // dot2
                            (0, 2) => 0x04, // dot3
                            (1, 0) => 0x08, // dot4
                            (1, 1) => 0x10, // dot5
                            (1, 2) => 0x20, // dot6
                            (0, 3) => 0x40, // dot7
                            (1, 3) => 0x80, // dot8
                            _ => 0,
                        };
                        if dot != 0 {
                            cell |= bit;
                        }
                    }
                }
                // Unicode Braille文字はU+2800から始まる
                let braille_char = std::char::from_u32(0x2800 + cell as u32)
                    .unwrap_or(' ');
                output.push(braille_char);
            }
            output.push('\n');
        }

        output
    }
}

// fn main() {
//     // 例: 160x160の疑似pixelキャンバス（dot単位）
//     let mut canvas = DotCanvas::new(160, 160);

//     // 任意の描画例: 中央付近に斜めの線を描く
//     for i in 0..160 {
//         canvas.set(i, i, 1);
//     }

//     // Braille文字列に変換して出力
//     let braille_art = canvas.to_braille();
//     println!("{}", braille_art);
// }
