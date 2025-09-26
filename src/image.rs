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
        Self { width, height, pixels: Vec::new() }
    }

    pub fn add_pixel(&mut self, color: Color) {
        self.pixels.push(color);
    }
    
    pub fn save_ppm(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Manually write using Unix-style newlines
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
