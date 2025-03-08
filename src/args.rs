use clap::Parser;
use crate::size::Size; // Added import for Size

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
