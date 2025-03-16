use clap::{Parser, ArgGroup}; // Modified import to include ArgGroup
use crate::size::Size; // Added import for Size

use clap::ValueEnum;

#[derive(ValueEnum, PartialEq, Clone, Debug, Copy)]
pub enum ContrastOption {
    None,
    Stretch,
    Equalize,
}

#[derive(ValueEnum, PartialEq, Clone, Debug, Copy)]
pub enum BinarizeOption {
    None,
    Odith,
    Fsdith,
    Otsu,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
    /// Input image file path or video file path
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// Output size({width}x{height})
    #[arg(short, long, default_value = "0x0")]
    pub size: Size,

    /// Contrast option
    #[arg(long, default_value = "none")]
    pub contrast: ContrastOption,

    /// Invert dot color
    #[arg(long)]
    pub invert: bool,

    /// Binarize option
    #[arg(long, default_value = "none")]
    pub binarize: BinarizeOption,

    /// Generate a bash script that shows the braille text
    #[arg(long, default_value = "")]
    pub scriptify: String,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,

}
