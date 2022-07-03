use std::{
    array::IntoIter,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};

use image::Rgb;
use iter_fixed::{IntoIteratorFixed, IteratorFixed};

pub type RawColor = Rgb<u8>;

#[derive(Debug, Copy, Clone)]
pub struct Color([f64; 3]);

impl Color {
    pub const BLACK: Color = Color([0.0, 0.0, 0.0]);
    pub const ERR_COLOR: Color = Color([0.0, 1.0, 0.0]);

    fn cut_value(n: f64) -> f64 {
        n.clamp(0.0, 1.0)
    }

    fn convert_value(v: i32) -> f64 {
        Self::cut_value(v as f64 / 255.0)
    }

    fn cut(&self) -> Self {
        self.fixed_iter().map(Self::cut_value).into()
    }

    pub fn new(r: i32, g: i32, b: i32) -> Self {
        [r, g, b].into_iter_fixed().map(Self::convert_value).into()
    }

    pub fn raw(self) -> RawColor {
        self.into()
    }

    fn fixed_iter(&self) -> IteratorFixed<IntoIter<f64, 3>, 3> {
        self.0.into_iter_fixed()
    }

    fn diff(&self, rhs: &Self) -> f64 {
        self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(s, r)| (s - r).abs())
            .sum::<f64>()
            / 3.0
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
        let sum: Color = colors.into_iter().sum();
        sum / (len as f64)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.fixed_iter().zip(rhs.0).map(|(a, b)| a + b).into()
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
        self.fixed_iter().map(|a| a * rhs).into()
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
        self.fixed_iter().map(|a| a / rhs).into()
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.fixed_iter().zip(rhs.0).map(|(a, b)| a * b).into()
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::BLACK, Color::add)
    }
}

impl From<Color> for RawColor {
    fn from(color: Color) -> Self {
        Rgb(color
            .cut()
            .fixed_iter()
            .map(|x| (x * 255.0) as u8)
            .collect())
    }
}

impl From<[f64; 3]> for Color {
    fn from(v: [f64; 3]) -> Self {
        Self(v)
    }
}

impl<I: Iterator<Item = f64>> From<IteratorFixed<I, 3>> for Color {
    fn from(iter: IteratorFixed<I, 3>) -> Self {
        iter.collect::<[f64; 3]>().into()
    }
}
