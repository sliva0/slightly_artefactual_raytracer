use std::ops::Mul;

use super::{Vector, BASIS};

#[derive(Copy, Clone)]
pub struct Matrix {
    x: Vector,
    y: Vector,
    z: Vector,
}

impl Matrix {
    fn cos_sin(angle: f64) -> (f64, f64) {
        let angle = angle.to_radians();
        (angle.cos(), angle.sin())
    }

    pub fn new(x: Vector, y: Vector, z: Vector) -> Self {
        Self { x, y, z }
    }

    pub fn new_y_rotation(angle: f64) -> Self {
        let (cos, sin) = Self::cos_sin(angle);
        Self::new(
            Vector::new(cos, 0.0, sin),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(-sin, 0.0, cos),
        )
    }

    pub fn new_x_rotation(angle: f64) -> Self {
        let (cos, sin) = Self::cos_sin(angle);
        Self::new(
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, cos, -sin),
            Vector::new(0.0, sin, cos),
        )
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let [cx, cy, cz] = BASIS;
        let (cx, cy, cz) = (rhs * cx, rhs * cy, rhs * cz);

        Self::new(
            Vector::new(self.x * cx, self.x * cy, self.x * cz),
            Vector::new(self.y * cx, self.y * cy, self.y * cz),
            Vector::new(self.z * cx, self.z * cy, self.z * cz),
        )
    }
}
