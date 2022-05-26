use std::ops::{Add, BitXor, Div, Mul, Neg, Shr, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Vector = Point;

pub const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const BASIS: [Vector; 3] = [
    Vector { x: 1.0, ..ORIGIN },
    Vector { y: 1.0, ..ORIGIN },
    Vector { z: 1.0, ..ORIGIN },
];

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn dist(self, rhs: Self) -> f64 {
        (self >> rhs).abs()
    }
    ///pairwise coordinate multiplication
    pub fn pmul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
    ///pairwise coordinate division
    pub fn pdiv(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }
}
impl Vector {
    pub fn abs(self) -> f64 {
        (self * self).sqrt()
    }
    #[allow(illegal_floating_point_literal_pattern)]
    pub fn normalize(self) -> Self {
        match self.abs() {
            0.0 => ORIGIN,
            abs => self / abs,
        }
    }
    pub fn dot(self, rhs: Self) -> f64 {
        self.pmul(rhs).sum()
    }
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (self * normal * 2.0)
    }
    pub fn compute_reflectance_and_refract(
        self,
        normal: Self,
        n1: f64,
        n2: f64,
    ) -> (f64, Option<Self>) {
        let n = n1 / n2;
        let cos_i = -self * normal;
        let sin_t2 = n * n * (1.0 - cos_i * cos_i);
        if sin_t2 >= 1.0 {
            return (1.0, None); //total internal reflection
        }
        let cos_t = (1.0 - sin_t2).sqrt();
        let r1 = (n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t); //Fresnel equations
        let r2 = (n2 * cos_i - n1 * cos_t) / (n2 * cos_i + n1 * cos_t);

        let refracted = self * n + normal * (n * cos_i - cos_t); //refracted ray
        ((r1 * r1 + r2 * r2) / 2.0, Some(refracted))
    }
}

impl From<Point> for [f64; 3] {
    fn from(p: Point) -> Self {
        [p.x, p.y, p.z]
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Vector) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Vector {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
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
impl Shr for Point {
    type Output = Vector;
    fn shr(self, rhs: Self) -> Vector {
        rhs - self
    }
}
