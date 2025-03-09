use clap::{Parser, ArgGroup}; // Modified import to include ArgGroup
use crate::size::Size; // Added import for Size

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("binarization")
        .args(&["otsu", "odith", "fsdith"])
        .multiple(false)
))]
pub struct Args {
    /// 入力画像ファイルパス
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// 出力画像の列数（横幅）と行数（縦幅）
    #[arg(short, long, default_value = "0x0")]
    pub size: Size,

    /// コントラストストレッチを適用する
    #[arg(long)]
    pub contrast: bool,

    /// ヒストグラム平坦化を適用する
    #[arg(long)]
    pub histogram: bool,

    /// 画像の色を反転する
    #[arg(long)]
    pub invert: bool,

    /// Ordered dither を適用する
    #[arg(long)]
    pub odith: bool,

    /// Floyd-steinburg dither を適用する
    #[arg(long)]
    pub fsdith: bool,

    /// Otsu の方法による二値化を適用する
    #[arg(long)]
    pub otsu: bool,
}
