use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};

use image::Rgb;

pub type RawColor = Rgb<u8>;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}
impl Color {
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
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

    fn diff(&self, rhs: &Self) -> f64 {
        let sub = [
            (self.r - rhs.r).abs(),
            (self.g - rhs.g).abs(),
            (self.b - rhs.b).abs(),
        ];
        sub.iter().sum::<f64>() / (255.0 * 3.0)
    }

    pub fn colors_diff(colors: &Vec<Self>) -> f64 {
        let mut max_diff = 0f64;
        for i in colors.iter() {
            for j in colors.iter() {
                max_diff = max_diff.max(i.diff(j));
            }
        }
        max_diff
    }

    pub fn colors_avg(colors: Vec<Self>) -> Self {
        let len = colors.len();
        colors.into_iter().sum::<Color>() / len as f64
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

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
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

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::BLACK, |a, b| a + b)
    }
}

impl From<Color> for RawColor {
    fn from(color: Color) -> Self {
        let color = color.cut();
        Rgb([color.r as u8, color.g as u8, color.b as u8])
    }
}
