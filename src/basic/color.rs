use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};

use derive_more::{Deref, DerefMut, From, Into};
use image::Rgb;
use iter_fixed::IntoIteratorFixed;

use super::primitive::*;

/// Color representation from [`image`] library.
pub type RawColor = Rgb<u8>;

impl From<Color> for RawColor {
    fn from(color: Color) -> Self {
        Rgb(color.into_iter_fixed().map(color_to_raw).collect())
    }
}

fn color_from_raw(v: u8) -> f64 {
    (v as f64 / 255.0).clamp(0.0, 1.0)
}

fn color_to_raw(v: f64) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0) as u8
}

#[derive(Debug, Copy, Clone, From, Into, Deref, DerefMut)]
pub struct Color([f64; 3]);

impl Color {
    pub const BLACK: Color = Color([0.0, 0.0, 0.0]);
    pub const ERR_COLOR: Color = Color([0.0, 1.0, 0.0]);

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b].into_iter_fixed().map(color_from_raw).collect())
    }

    /// Calculates difference between colors.
    ///
    /// Returns [`f64`] from 0.0 to 1.0 (if color is normalized).
    fn diff(&self, rhs: &Self) -> f64 {
        self.into_iter()
            .zip(**rhs)
            .map(|(s, r)| (s - r).abs())
            .sum::<f64>()
            / (self.len() as f64)
    }

    /// Calculates max difference between each element from [`Color`] slice.
    pub fn max_diff(colors: &[Self]) -> f64 {
        let mut max_diff = 0f64;

        for i in colors.iter() {
            for j in colors.iter() {
                max_diff = max_diff.max(i.diff(j));
            }
        }
        max_diff
    }

    /// Calculate average color from [`Vec<Color>`].
    pub fn average_color(colors: Vec<Self>) -> Self {
        let len = colors.len();
        let sum: Color = colors.into_iter().sum();
        sum / (len as f64)
    }
}

primitive_operators!(
    Color;
    [rhs=Self] {
        Add(add) + AddAssign(add_assign);
        Mul(mul);
    }
    [rhs=f64] {
        Mul(mul) + MulAssign(mul_assign);
        Div(div);
    }
);

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::BLACK, Color::add)
    }
}

// impl AddAssign<Color> for Color {
//     fn add_assign(&mut self, rhs: Color) {
//         self.iter_mut()
//             .zip(rhs)
//             .for_each(|(a, b)| f64::add_assign(a, b));
//     }
// }
