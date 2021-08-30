use std::sync::Arc;

use super::{
    objects, Color, MarchingObjectType, MetaTracingObjectType, ObjectType, Point,
    TracingObjectType, EPSILON,
};

enum SdfCheckRes<'a> {
    Miss(f64),
    Hit(ObjectType<'a>),
}

pub struct Scene<'a> {
    marching_objs: Vec<MarchingObjectType<'a>>,
    tracing_objs: Vec<TracingObjectType<'a>>,
    meta_objs: Vec<MetaTracingObjectType<'a>>,
}
impl<'a> Scene<'a> {
    fn build_meta_objects(&mut self) {
        for object in self.meta_objs.iter().map(Arc::clone) {
            self.tracing_objs.extend(object.build_objects());
        }
    }

    pub fn new(
        marching_objs: Vec<MarchingObjectType<'a>>,
        tracing_objs: Vec<TracingObjectType<'a>>,
        meta_objs: Vec<MetaTracingObjectType<'a>>,
    ) -> Self {
        let mut new_self = Self {
            marching_objs,
            tracing_objs,
            meta_objs,
        };
        new_self.build_meta_objects();
        new_self
    }

    fn check_sdf(&self, pos: Point) -> SdfCheckRes {
        let mut sdf = f64::INFINITY;

        for object in self.marching_objs.iter() {
            sdf = sdf.min(object.check_sdf(pos));
            if sdf < EPSILON {
                return SdfCheckRes::Hit(object.clone().upcast());
            }
        }
        SdfCheckRes::Miss(sdf)
    }

    fn march_ray(&self, start: Point, dir: Point, max_depth: f64) -> Option<(ObjectType, f64)> {
        let mut depth = 0.0;

        loop {
            let pos = start + (dir * depth);
            match self.check_sdf(pos) {
                SdfCheckRes::Hit(obj) => return Some((obj, depth)),
                SdfCheckRes::Miss(sdf) => depth += sdf,
            }
            if depth > max_depth || depth == f64::INFINITY {
                return None;
            }
        }
    }

    fn trace_ray(&self, start: Point, dir: Point) -> (ObjectType, Point) {
        let mut distance = f64::INFINITY;
        let mut object: ObjectType = Arc::new(objects::DummyObject());

        for obj in self.tracing_objs.iter() {
            match obj.find_intersection(start, dir) {
                Some(dist) if dist < distance => {
                    object = obj.clone().upcast();
                    distance = dist;
                }
                _ => (),
            };
        }
        match self.march_ray(start, dir, distance) {
            Some((obj, dist)) => {
                object = obj;
                distance = dist;
            }
            None => (),
        }

        (object, start + dir * distance)
    }

    pub fn compute_ray(&self, start: Point, dir: Point) -> Color {
        let (object, pos) = self.trace_ray(start, dir);

        
        object.get_color(pos)
    }
}
