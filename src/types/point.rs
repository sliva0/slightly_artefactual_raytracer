use std::ops::{Add, BitXor, Div, Mul, Neg, Shr, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn abs(self) -> f64 {
        (self * self).sqrt()
    }
    pub fn dist(self, rhs: Self) -> f64 {
        (self >> rhs).abs()
    }
    #[allow(illegal_floating_point_literal_pattern)]
    pub fn normalize(self) -> Self {
        match self.abs() {
            0.0 => Self::new(),
            abs => self / abs,
        }
    }
    pub fn scalar(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
impl From<Point> for [f64; 3] {
    fn from(p: Point) -> Self {
        [p.x, p.y, p.z]
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Mul for Point {
    type Output = f64;
    fn mul(self, rhs: Self) -> f64 {
        self.scalar(rhs)
    }
}
impl BitXor for Point {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        self.cross(rhs)
    }
}
impl Shr for Point {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self {
        rhs - self
    }
}
