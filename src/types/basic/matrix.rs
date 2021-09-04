use std::ops::Mul;

use super::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Matrix(Vector, Vector, Vector);

impl Matrix {
    pub fn from_vectors(a: Vector, b: Vector, c: Vector) -> Self {
        Matrix(a, b, c)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.0 * rhs,
            y: self.1 * rhs,
            z: self.2 * rhs,
        }
    }
}
