use std::fmt;


pub struct Display {
    pub pixels: [[u8; Display::WIDTH]; Display::HEIGHT],
}


impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn new() -> Display {
        Display {
            pixels: [[0; Display::WIDTH]; Display::HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [[0; Display::WIDTH]; Display::HEIGHT];
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8, pixel: u8) -> u8 {
        self.pixels[y as usize % Display::HEIGHT][x as usize % Display::WIDTH] ^= pixel;
        (self.pixels[y as usize % Display::HEIGHT][x as usize % Display::WIDTH] ^ pixel) & pixel
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = "".to_string();
        for (i, row) in self.pixels.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                output += &format!("{}", self.pixels[i][j] as u8);
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

