use std::sync::Arc;

use super::{Color, Point};

pub trait Upcast {
    fn upcast<'a>(self: Arc<Self>) -> Arc<dyn Object + 'a>
    where
        Self: 'a;
}
impl<T: Object> Upcast for T {
    fn upcast<'a>(self: Arc<Self>) -> Arc<dyn Object + 'a>
    where
        Self: 'a,
    {
        self
    }
}

pub trait Object: Upcast {
    fn get_color(&self, pos: Point) -> Color;
    fn get_normal(&self, pos: Point, eps: f64) -> Point;
    //fn get_material(&self, pos: Point) -> Material;
}

pub trait MarchingObject: Object {
    fn check_sdf(&self, pos: Point) -> f64;

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

pub trait TracingObject: Object {
    fn find_intersection(&self, pos: Point) -> f64;
}

pub struct Room {
    pub size: f64,
    pub square_size: f64,
    pub colors: (Color, Color),
}

impl Object for Room {
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

    fn get_normal(&self, pos: Point, eps: f64) -> Point {
        MarchingObject::get_normal(self, pos, eps)
    }
}

impl MarchingObject for Room {
    fn check_sdf(&self, pos: Point) -> f64 {
        let arr: [f64; 3] = pos.into();
        self.size - arr.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
}

struct Plane {
    ///plane normal
    n: Point,
    ///d from plane equation (ax+by+cz+d=0), plane shift
    d: f64,
}
impl Plane {
    fn new(v1: Point, v2: Point, v3: Point) -> Self {
        let n = (v1 >> v2) ^ (v1 >> v3);
        Self {
            n,
            d: 0.0, //-n.scalar_mul(v1),
        }
    }
}
