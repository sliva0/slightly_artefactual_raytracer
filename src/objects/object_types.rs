use std::{fmt::Debug, sync::Arc};

use iter_fixed::IntoIteratorFixed;

use super::*;
use crate::SceneObjects;

pub trait Upcast: Sync + Send {
    fn upcast<'a>(self: Arc<Self>) -> Arc<dyn Object + 'a>
    where
        Self: 'a;
}
impl<T: Object + Sized> Upcast for T {
    fn upcast<'a>(self: Arc<Self>) -> Arc<dyn Object + 'a>
    where
        Self: 'a,
    {
        self
    }
}

pub trait Object: Upcast + Debug {
    fn color(&self, pos: Point) -> Color;
    fn normal(&self, pos: Point) -> Vector;
    fn material(&self) -> Material;
    fn is_schematic(&self) -> bool {
        false
    }
}

pub trait MarchingObject: Object {
    fn sdf(&self, pos: Point) -> f64;

    //SDF derivative
    fn sdf_drv(&self, pos: Point, delta: Vector) -> f64 {
        self.sdf(pos + delta) - self.sdf(pos - delta)
    }

    fn sdf_normal(&self, pos: Point) -> Vector {
        BASIS.into_iter_fixed().map(|x| self.sdf_drv(pos, x)).into()
    }
}

pub trait TracingObject: Object {
    fn find_intersection(&self, ray: Ray) -> Option<f64>;
}

pub trait MetaTracingObject: Sync + Send + Debug {
    fn build_objects(self: Arc<Self>) -> Vec<TracingObjectType>;
}

pub trait ReferenceObject: MetaTracingObject {
    fn color(&self, pos: Point) -> Color;
    fn material(&self) -> Material;
}

impl<T: Object + MetaTracingObject> ReferenceObject for T {
    fn color(&self, pos: Point) -> Color {
        Object::color(self, pos)
    }

    fn material(&self) -> Material {
        Object::material(self)
    }
}

pub trait LightSource: Sync + Send {
    fn _light_dir(&self, pos: Point) -> Vector;
    fn _brightness(&self, pos: Point) -> f64;

    fn dist(&self, pos: Point) -> f64;
    fn color(&self, pos: Point) -> Color;

    fn build_schematic_objects(self: Arc<Self>) -> Vec<TracingObjectType>;

    fn light_dir(&self, scene_objs: &SceneObjects, pos: Point) -> Option<Vector> {
        let dir = self._light_dir(pos);
        let dist = self.dist(pos);
        if scene_objs.compute_shadow_ray(Ray::new(pos, -dir), dist) {
            None
        } else {
            Some(dir)
        }
    }
    fn brightness(&self, pos: Point) -> f64 {
        let dist = self.dist(pos);
        self._brightness(pos) / (dist * dist)
    }
}

pub type ObjectType = Arc<dyn Object>;
pub type MarchingObjectType = Arc<dyn MarchingObject>;
pub type TracingObjectType = Arc<dyn TracingObject>;
pub type MetaTracingObjectType = Arc<dyn MetaTracingObject>;
pub type LightSourceType = Arc<dyn LightSource>;
