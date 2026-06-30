use std::ops::{Add, BitXor, Div, Mul, Neg, Shr, Sub};

use derive_more::{Deref, DerefMut, From, Into};
use iter_fixed::IntoIteratorFixed;

use super::primitive::*;

#[derive(Debug, Copy, Clone, From, Into, Deref, DerefMut)]
pub struct Point([f64; 3]);
pub type Vector = Point;

impl Point {
    pub const ORIGIN: Point = Point([0.0, 0.0, 0.0]);

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        [x, y, z].into()
    }

    pub fn dist(self, rhs: Self) -> f64 {
        (self >> rhs).abs()
    }

    ///pairwise coordinate multiplication
    pub fn pairwise_mul(&self, rhs: Self) -> Self {
        self.map_binary_op(rhs, f64::mul)
    }
}

impl Vector {
    pub const BASIS: [Vector; 3] = [
        Point([1.0, 0.0, 0.0]),
        Point([0.0, 1.0, 0.0]),
        Point([0.0, 0.0, 1.0]),
    ];

    pub fn abs(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let abs = self.abs();
        if abs.is_normal() {
            self / abs
        } else {
            Self::ORIGIN
        }
    }

    pub fn dot_mul(&self, rhs: Self) -> f64 {
        self.pairwise_mul(rhs).iter().sum()
    }

    pub fn cross_mul(&self, rhs: Self) -> Self {
        let [xa, ya, za] = **self;
        let [xb, yb, zb] = *rhs;
        Self([ya * zb - za * yb, za * xb - xa * zb, xa * yb - ya * xb])
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

primitive_operators!(
    Point;
    [rhs=Self] {
        Add(add);
        Sub(sub);
    }
    [rhs=f64] {
        Mul(mul);
        Div(div);
    }
);

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Mul for Vector {
    type Output = f64;
    fn mul(self, rhs: Self) -> f64 {
        self.dot_mul(rhs)
    }
}

impl BitXor for Vector {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        self.cross_mul(rhs)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr for Point {
    type Output = Vector;
    fn shr(self, rhs: Self) -> Vector {
        rhs - self
    }
}
