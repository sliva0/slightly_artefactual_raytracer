use std::sync::Arc;

use super::{Color, Point, Vector};

pub type ObjectType<'a> = Arc<dyn Object + 'a>;
pub type MarchingObjectType<'a> = Arc<dyn MarchingObject + 'a>;
pub type TracingObjectType<'a> = Arc<dyn TracingObject + 'a>;
pub type MetaTracingObjectType<'a> = Arc<dyn MetaTracingObject + 'a>;

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
    fn get_normal(&self, pos: Point, eps: f64) -> Vector;
    //fn get_material(&self, pos: Point) -> Material;
}

pub trait MarchingObject: Object {
    fn check_sdf(&self, pos: Point) -> f64;

    fn sdf_derivative(&self, pos: Point, delta: Vector) -> f64 {
        self.check_sdf(pos + delta) - self.check_sdf(pos - delta)
    }

    fn get_normal(&self, pos: Point, eps: f64) -> Vector {
        let p0 = Point::new();
        Vector {
            x: self.sdf_derivative(pos, Vector { x: eps, ..p0 }),
            y: self.sdf_derivative(pos, Vector { y: eps, ..p0 }),
            z: self.sdf_derivative(pos, Vector { z: eps, ..p0 }),
        }
    }
}

pub trait TracingObject: Object {
    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64>;
}

pub trait MetaTracingObject {
    fn get_color(&self, pos: Point) -> Color;
    fn build_objects<'a>(self: Arc<Self>) -> Vec<TracingObjectType<'a>>;
}
