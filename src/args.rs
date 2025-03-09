use clap::{Parser, ArgGroup}; // Modified import to include ArgGroup
use crate::size::Size; // Added import for Size

use clap::ValueEnum;

#[derive(ValueEnum, PartialEq, Clone, Debug)]
pub enum ContrastOption {
    None,
    Stretch,
    Equalize,
}

#[derive(ValueEnum, PartialEq, Clone, Debug)]
pub enum BinarizeOption {
    None,
    Odith,
    Fsdith,
    Otsu,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input image file path
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// Output size({width}x{height})
    #[arg(short, long, default_value = "0x0")]
    pub size: Size,

    /// Contrast option
    #[arg(long, default_value = "none")]
    pub contrast: ContrastOption,

    /// 画像の色を反転する
    #[arg(long)]
    pub invert: bool,

    /// Binarize option
    #[arg(long, default_value = "none")]
    pub binarize: BinarizeOption,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,

}
