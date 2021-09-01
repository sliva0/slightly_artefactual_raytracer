use image::Rgb;
use std::ops::{Add, AddAssign, Mul, MulAssign};

pub type RawColor = Rgb<u8>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}
impl Color {
    pub const ERR_COLOR: Color = Color {
        r: 0.0,
        g: 255.0,
        b: 0.0,
    };

    fn cut_f64(n: f64) -> f64 {
        const MIN: f64 = 0.0;
        const MAX: f64 = 255.0;

        match n {
            n if n > MAX => MAX,
            n if n < MIN => MIN,
            n => n,
        }
    }
    fn cut(&self) -> Self {
        Self {
            r: Self::cut_f64(self.r),
            g: Self::cut_f64(self.g),
            b: Self::cut_f64(self.b),
        }
    }
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self {
            r: r as f64,
            g: g as f64,
            b: b as f64,
        }
        .cut()
    }
    pub fn raw(self) -> RawColor {
        self.into()
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}
impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r / 255.0,
            g: self.g * rhs.g / 255.0,
            b: self.b * rhs.b / 255.0,
        }
    }
}
impl From<Color> for RawColor {
    fn from(color: Color) -> Self {
        let color = color.cut();
        Rgb([color.r as u8, color.g as u8, color.b as u8])
    }
}