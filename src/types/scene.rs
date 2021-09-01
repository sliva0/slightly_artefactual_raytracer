use std::sync::Arc;

use super::{
    object_types::LightSourceType, objects::DummyObject, Color, MarchingObjectType,
    MetaTracingObjectType, ObjectType, Point, TracingObjectType, Vector, EPSILON,
};

enum SdfCheckRes<'a> {
    Miss(f64),
    Hit(ObjectType<'a>),
}

pub struct Scene<'a> {
    marching_objs: Vec<MarchingObjectType<'a>>,
    tracing_objs: Vec<TracingObjectType<'a>>,
    meta_objs: Vec<MetaTracingObjectType<'a>>,
    lamps: Vec<LightSourceType<'a>>,
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
        lamps: Vec<LightSourceType<'a>>,
    ) -> Self {
        let mut new_self = Self {
            marching_objs,
            tracing_objs,
            meta_objs,
            lamps,
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

    fn march_ray(&self, start: Point, dir: Vector, max_depth: f64) -> Option<(ObjectType, f64)> {
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

    fn trace_ray(&self, start: Point, dir: Vector) -> Option<(ObjectType, f64)> {
        let mut distance = f64::INFINITY;
        let mut object_and_dist = None;

        for obj in self.tracing_objs.iter() {
            if let Some(dist) = obj.find_intersection(start, dir) {
                if dist < distance && dist > 0.0 {
                    object_and_dist = Some((obj.clone().upcast(), dist));
                    distance = dist;
                }
            }
        }
        object_and_dist
    }

    fn compute_lightning(&self, object: ObjectType, pos: Point, _dir: Vector) -> Color {
        let mut final_color = Color::new(0, 0, 0);
        let obj_color = object.get_color(pos);
        let normal = object.get_normal(pos, EPSILON);

        for source in self.lamps.iter() {
            if let Some(light_dir) = source.get_light_dir(pos) {
                let angle_cos = -light_dir * normal;
                if angle_cos <= 0.0 {
                    continue;
                }
                let mut brightness = source.get_brightness(pos);
                brightness *= angle_cos;
                final_color += obj_color * source.get_color(pos) * brightness;
            }
        }
        final_color
    }

    pub fn compute_ray(&self, start: Point, dir: Vector) -> Color {
        let mut object: ObjectType = Arc::new(DummyObject());
        let mut distance = f64::INFINITY;

        if let Some((obj, dist)) = self.trace_ray(start, dir) {
            object = obj;
            distance = dist;
        }
        if let Some((obj, dist)) = self.march_ray(start, dir, distance) {
            object = obj;
            distance = dist;
        }

        let pos = start + dir * distance;
        self.compute_lightning(object, pos, dir)
    }
}
