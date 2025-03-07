use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Size(pub u32, pub u32);

use std::fmt;

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.0, self.1)
    }
}

impl FromStr for Size {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// 入力画像ファイルパス
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// 出力画像の列数（横幅）と行数（縦幅）
    #[arg(short, long, default_value = "80x40")]
    pub size: Size,

    /// Ordered dither を適用する
    #[arg(long)]
    pub dither: bool,

    /// コントラストストレッチを適用する
    #[arg(long)]
    pub contrast: bool,

    /// 画像の色を反転する
    #[arg(long)]
    pub invert: bool,

    /// ヒストグラム平坦化を適用する
    #[arg(long)]
    pub histogram: bool,

    /// Otsu の方法による二値化を適用する
    #[arg(long)]
    pub otsu: bool,
}
