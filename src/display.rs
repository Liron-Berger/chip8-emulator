use std::vec::Vec;
use std::fmt;


pub struct Display {
    pixels: Vec<Vec<bool>>,
}


impl Display {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 32;

    pub fn new() -> Display {
        Display {
            pixels: vec![vec![false; Display::WIDTH]; Display::HEIGHT],
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = "".to_string();
        for (i, row) in self.pixels.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                output += &format!("{}", self.pixels[i][j] as u8);
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

