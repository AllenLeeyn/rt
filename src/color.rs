#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {

    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const RED:   Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE:  Color = Color { r: 0, g: 0, b: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
    pub const CYAN:   Color = Color { r: 0, g: 255, b: 255 };
    pub const MAGENTA:Color = Color { r: 255, g: 0, b: 255 };
    pub const GRAY:   Color = Color { r: 128, g: 128, b: 128 };
    pub const ORANGE: Color = Color { r: 255, g: 165, b: 0 };

    pub const PASTEL_RED:     Color = Color { r: 255, g: 179, b: 186 };
    pub const PASTEL_GREEN:   Color = Color { r: 186, g: 255, b: 201 };
    pub const PASTEL_BLUE:    Color = Color { r: 186, g: 225, b: 255 };
    pub const PASTEL_YELLOW:  Color = Color { r: 255, g: 255, b: 186 };
    pub const PASTEL_PURPLE:  Color = Color { r: 220, g: 190, b: 255 };
    pub const PASTEL_ORANGE:  Color = Color { r: 255, g: 209, b: 170 };
    pub const PASTEL_PINK:    Color = Color { r: 255, g: 192, b: 203 };
    pub const PASTEL_CYAN:    Color = Color { r: 174, g: 255, b: 252 };
    pub const PASTEL_LIME:    Color = Color { r: 210, g: 255, b: 173 };
    pub const PASTEL_GRAY:    Color = Color { r: 200, g: 200, b: 200 };

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
            let mixed = c1 as f32 + (c2 as f32 - c1 as f32) * t.clamp(0.0, 1.0);
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

use std::ops::Mul;
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        fn scale(c: u8, f: f32) -> u8 {
            (c as f32 * f.clamp(0.0, 1.0)).round().clamp(0.0, 255.0) as u8
        }

        Color {
            r: scale(self.r, rhs),
            g: scale(self.g, rhs),
            b: scale(self.b, rhs),
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        fn blend(a: u8, b: u8) -> u8 {
            ((a as u16 * b as u16) / 255) as u8
        }

        Color {
            r: blend(self.r, rhs.r),
            g: blend(self.g, rhs.g),
            b: blend(self.b, rhs.b),
        }
    }
}

use std::ops::AddAssign;

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r = self.r.saturating_add(other.r);
        self.g = self.g.saturating_add(other.g);
        self.b = self.b.saturating_add(other.b);
    }
}
