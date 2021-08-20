use super::Point;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Matrix([[f64; 3]; 3]);

impl Matrix {
    pub fn _new() -> Self {
        let z = [0.0, 0.0, 0.0];
        Matrix([z, z, z])
    }

    pub fn from_points(a: Point, b: Point, c: Point) -> Self {
        Matrix([a.into(), b.into(), c.into()])
    }

    fn arr_mul(arr1: [f64; 3], arr2: [f64; 3]) -> f64 {
        arr1.iter().zip(arr2).map(|(x, y)| x * y).sum()
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        let arr = rhs.into();
        Point {
            x: Self::arr_mul(arr, self.0[0]),
            y: Self::arr_mul(arr, self.0[1]),
            z: Self::arr_mul(arr, self.0[2]),
        }
    }
}
