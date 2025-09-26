#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self{ r, g ,b }
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
    
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        fn mix_channel(c1: u8, c2: u8, t: f32) -> u8 {
            let mixed = c1 as f32 + (c2 - c1) as f32 * t.clamp(0.0, 1.0);
            mixed.round().clamp(0.0, 255.0) as u8
        }

        Color {
            r: mix_channel(a.r, b.r, t),
            g: mix_channel(a.g, b.g, t),
            b: mix_channel(a.b, b.b, t),
        }
    }
}

use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}