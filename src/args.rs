use clap::{Parser, ArgGroup}; // Modified import to include ArgGroup
use crate::size::Size; // Added import for Size

use clap::ValueEnum;

#[derive(ValueEnum, PartialEq, Clone, Debug)]
pub enum ContrastOption {
    none,
    stretch,
    equalize,
}

#[derive(ValueEnum, PartialEq, Clone, Debug)]
pub enum BinarizeOption {
    none,
    odith,
    fsdith,
    otsu,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// 入力画像ファイルパス
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// 出力画像の列数（横幅）と行数（縦幅）
    #[arg(short, long, default_value = "0x0")]
    pub size: Size,

    /// コントラスト (ContrastOption::none, ContrastOption::stretch, ContrastOption::equalize)
    #[arg(long, default_value = "none")]
    pub contrast: ContrastOption,

    /// 画像の色を反転する
    #[arg(long)]
    pub invert: bool,

    /// 2値化 (BinarizeOption::None, BinarizeOption::odith, BinarizeOption::fsdith, BinarizeOption::otsu)
    #[arg(long, default_value = "none")]
    pub binarize: BinarizeOption,
}
