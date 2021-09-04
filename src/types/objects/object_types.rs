use std::sync::Arc;

use super::*;

pub trait Upcast: Sync + Send {
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

#[allow(unused_variables)]
pub trait Object: Upcast + Sync + Send {
    fn get_color(&self, pos: Point) -> Color;
    fn get_normal(&self, pos: Point) -> Vector;
    fn get_material(&self, pos: Point) -> Material;
    fn is_shematic(&self) -> bool {
        false
    }
}

pub trait MarchingObject: Object {
    fn check_sdf(&self, pos: Point) -> f64;

    fn sdf_derivative(&self, pos: Point, delta: Vector) -> f64 {
        self.check_sdf(pos + delta) - self.check_sdf(pos - delta)
    }

    fn get_normal(&self, pos: Point) -> Vector {
        let p0 = Point::new();
        Vector {
            x: self.sdf_derivative(pos, Vector { x: EPSILON, ..p0 }),
            y: self.sdf_derivative(pos, Vector { y: EPSILON, ..p0 }),
            z: self.sdf_derivative(pos, Vector { z: EPSILON, ..p0 }),
        }
    }
}

pub trait TracingObject: Object {
    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64>;
}

pub trait MetaTracingObject: Sync + Send {
    fn get_color(&self, pos: Point) -> Color;
    fn get_material(&self, pos: Point) -> Material;
    fn build_objects<'a>(self: Arc<Self>) -> Vec<TracingObjectType<'a>>;
}

pub trait LightSource: Sync + Send {
    fn _get_light_dir(&self, pos: Point) -> Vector;
    fn _get_brightness(&self, pos: Point) -> f64;

    fn get_dist(&self, pos: Point) -> f64;
    fn get_color(&self, pos: Point) -> Color;

    fn build_schematic_objects<'a>(self: Arc<Self>) -> Vec<TracingObjectType<'a>>;

    fn get_light_dir(&self, scene: &Scene, pos: Point) -> Option<Vector> {
        let dir = self._get_light_dir(pos);
        let dist = self.get_dist(pos);
        if scene.compute_shadow_ray(pos, -dir, dist) {
            None
        } else {
            Some(dir)
        }
    }
    fn get_brightness(&self, pos: Point) -> f64 {
        let dist = self.get_dist(pos);
        self._get_brightness(pos) / (dist * dist)
    }
}

pub type ObjectType<'a> = Arc<dyn Object + 'a>;
pub type MarchingObjectType<'a> = Arc<dyn MarchingObject + 'a>;
pub type TracingObjectType<'a> = Arc<dyn TracingObject + 'a>;
pub type MetaTracingObjectType<'a> = Arc<dyn MetaTracingObject + 'a>;
pub type LightSourceType<'a> = Arc<dyn LightSource + 'a>;
