use std::str::FromStr;
use std::fmt;

// // Define the Size type
// #[derive(Debug, Clone)]
// pub struct Size {
//     pub width: u32,
//     pub height: u32,
// }

#[derive(Debug, Clone)]
pub struct Size(pub u32, pub u32);

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
