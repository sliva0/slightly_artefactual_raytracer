use super::{Color, Point};

pub trait SceneObject {
    fn check_sdf(&self, pos: Point) -> f64;
    fn get_color(&self, pos: Point) -> Color;
}

trait Surfaced {
    fn sdf_derivative(&self, pos: Point, delta: Point) -> f64;
    fn get_normal(&self, pos: Point, eps: f64) -> Point;
}

impl<T: SceneObject> Surfaced for T {
    fn sdf_derivative(&self, pos: Point, delta: Point) -> f64 {
        self.check_sdf(pos + delta) - self.check_sdf(pos - delta)
    }

    fn get_normal(&self, pos: Point, eps: f64) -> Point {
        let p0 = Point::new();
        Point {
            x: self.sdf_derivative(pos, Point { x: eps, ..p0 }),
            y: self.sdf_derivative(pos, Point { y: eps, ..p0 }),
            z: self.sdf_derivative(pos, Point { z: eps, ..p0 }),
        }
    }
}

pub struct Room {
    pub size: f64,
    pub square_size: f64,
    pub colors: (Color, Color),
}

impl SceneObject for Room {
    fn check_sdf(&self, pos: Point) -> f64 {
        let arr: [f64; 3] = pos.into();
        self.size - arr.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
    fn get_color(&self, pos: Point) -> Color {
        let arr: [f64; 3] = pos.into();
        let sum: i32 = arr
            .iter()
            .map(|x| ((x + self.size) / self.square_size).floor() as i32)
            .sum();
        match sum % 2 {
            1 => self.colors.0,
            _ => self.colors.1,
        }
    }
}
