// implement the Pixel struct and traits below

use std::{clone::Clone, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    r: u8, g: u8, b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel{r, g, b}
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

// implement the Image struct and traits below

pub struct Image {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let mut pixels: Vec<Pixel> = Vec::with_capacity(width * height);
        pixels.fill(Pixel::new(255, 255, 255));
        Image{width, height, pixels}
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.pixels.get(self.width * y + x)
    }
}
