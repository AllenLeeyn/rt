use std::fs::File;
use std::io::{BufWriter, Write};

use crate::color::*;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: Vec::with_capacity(width * height) }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let idx = y * self.width + x;
        if idx < self.pixels.len() {
            self.pixels[idx] = color;
        } else {
            self.pixels.push(color);
        }
    }
    
    pub fn save_ppm(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for pixel in &self.pixels {
            writeln!(writer, "{}", pixel)?;
        }

        Ok(())
    }
}

use std::fmt;
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write PPM header
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for pixel in &self.pixels {
            writeln!(f, "{}", pixel)?;
        }

        Ok(())
    }
}
