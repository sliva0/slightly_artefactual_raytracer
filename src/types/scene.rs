use std::sync::Arc;

use super::{
    objects::DummyObject, Color, LightSourceType, MarchingObjectType, MetaTracingObjectType,
    ObjectType, Point, TracingObjectType, Vector, EPSILON,
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
    reflection_limit: i32,
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
        reflection_limit: i32,
    ) -> Self {
        let mut new_self = Self {
            marching_objs,
            tracing_objs,
            meta_objs,
            lamps,
            reflection_limit,
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

    fn compute_lightning(&self, object: ObjectType, pos: Point, dir: Vector) -> Color {
        let obj_color = object.get_color(pos);
        if object.is_shematic() {
            return obj_color;
        }

        let normal = object.get_normal(pos);
        let mtrl = object.get_material(pos);
        let mut final_color = obj_color * mtrl.ambient;

        for source in self.lamps.iter() {
            if let Some(light_dir) = source.get_light_dir(self, pos) {
                let angle_cos = -light_dir * normal;
                if angle_cos <= 0.0 {
                    continue;
                }
                let src_color = source.get_color(pos);

                let diffuse_part = source.get_brightness(pos) * angle_cos;
                let diffuse_color = obj_color * src_color * (diffuse_part);

                let half_angle_dir = (light_dir + dir).normalize();
                let dot_prod = normal * half_angle_dir;
                let specular_part = dot_prod.powi(mtrl.smoothness) * mtrl.flare_intensity;
                let specular_color = src_color * specular_part;

                final_color += diffuse_color + specular_color;
            }
        }
        final_color
    }

    fn compute_ray(&self, start: Point, dir: Vector) -> (ObjectType, Point) {
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
        (object, pos)
    }

    pub fn compute_ray_reflections(&self, mut start: Point, mut dir: Vector) -> Color {
        let mut final_color = Color::new(0, 0, 0);
        let mut refl_cnt = self.reflection_limit;
        let mut refl_reserve = 1.0;

        loop {
            let (object, pos) = self.compute_ray(start, dir);
            let specularity = object.get_material(pos).specularity;
            let color = self.compute_lightning(object.clone(), pos, dir);

            if refl_cnt == 0 || specularity == 0.0 {
                return final_color + color * refl_reserve;
            }
            refl_cnt -= 1;
            let refl_coef = refl_reserve * (1.0 - specularity);
            refl_reserve -= refl_coef;
            final_color += color * refl_coef;

            start = pos;
            dir = dir.reflect(object.get_normal(pos));
        }
    }
}
