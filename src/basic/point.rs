use std::{
    array::IntoIter,
    ops::{Add, BitXor, Div, Mul, Neg, Shr, Sub},
    slice::Iter,
};

use iter_fixed::{IntoIteratorFixed, IteratorFixed};

#[derive(Debug, Copy, Clone)]
pub struct Point([f64; 3]);
pub type Vector = Point;

pub const ORIGIN: Point = Point::new(0.0, 0.0, 0.0);

pub const BASIS: [Vector; 3] = [
    Vector::new(1.0, 0.0, 0.0),
    Vector::new(0.0, 1.0, 0.0),
    Vector::new(0.0, 0.0, 1.0),
];

impl Point {
    fn fixed_iter(&self) -> IteratorFixed<IntoIter<f64, 3>, 3> {
        self.0.into_iter_fixed()
    }

    fn map_binary_op<F: Fn(f64, f64) -> f64>(&self, rhs: Self, f: F) -> Self {
        self.fixed_iter().zip(rhs.0).map(|(a, b)| f(a, b)).into()
    }

    fn map_with_number<F: Fn(f64, f64) -> f64>(&self, rhs: f64, f: F) -> Self {
        self.fixed_iter().map(|a| f(a, rhs)).into()
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn dist(self, rhs: Self) -> f64 {
        (self >> rhs).abs()
    }

    ///pairwise coordinate multiplication
    pub fn pmul(&self, rhs: Self) -> Self {
        self.map_binary_op(rhs, f64::mul)
    }

    pub fn iter(&self) -> Iter<'_, f64> {
        self.0.iter()
    }

    pub fn sum(&self) -> f64 {
        self.iter().sum()
    }
}

impl Vector {
    pub fn abs(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let abs = self.abs();
        if abs.is_normal() {
            self / abs
        } else {
            ORIGIN
        }
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.pmul(rhs).sum()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = rhs.0;
        Self([y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2])
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (self * normal * 2.0)
    }

    pub fn compute_reflectance_and_refract(
        self,
        mut normal: Self,
        n1: f64,
        n2: f64,
    ) -> Option<(f64, Self)> {
        if self * normal > 0.0 {
            normal = -normal;
        }

        let n = n1 / n2;
        let cos_i = -self * normal;
        let sin_t2 = n * n * (1.0 - cos_i * cos_i);
        if sin_t2 >= 1.0 {
            return None; //total internal reflection
        }
        let cos_t = (1.0 - sin_t2).sqrt();
        let r1 = (n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t); //Fresnel equations
        let r2 = (n2 * cos_i - n1 * cos_t) / (n2 * cos_i + n1 * cos_t);

        let refracted = self * n + normal * (n * cos_i - cos_t); //refracted ray
        Some(((r1 * r1 + r2 * r2) / 2.0, refracted))
    }
}

impl From<[f64; 3]> for Point {
    fn from(v: [f64; 3]) -> Self {
        Self(v)
    }
}

impl<I: Iterator<Item = f64>> From<IteratorFixed<I, 3>> for Point {
    fn from(iter: IteratorFixed<I, 3>) -> Self {
        iter.collect::<[f64; 3]>().into()
    }
}

impl IntoIterator for Point {
    type Item = f64;
    type IntoIter = IntoIter<f64, 3>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        self.fixed_iter().zip(rhs.0).map(|(a, b)| a + b).into()
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Vector) -> Self {
        self.map_binary_op(rhs, f64::sub)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        self.map_with_number(rhs, f64::mul)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Vector {
        self.map_with_number(rhs, f64::div)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Mul for Vector {
    type Output = f64;
    fn mul(self, rhs: Self) -> f64 {
        self.dot(rhs)
    }
}

impl BitXor for Vector {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        self.cross(rhs)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr for Point {
    type Output = Vector;
    fn shr(self, rhs: Self) -> Vector {
        rhs - self
    }
}
