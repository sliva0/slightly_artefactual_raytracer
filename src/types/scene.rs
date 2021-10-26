use std::sync::Arc;

use super::*;

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
        for lamp in self.lamps.iter().map(Arc::clone) {
            self.tracing_objs.extend(lamp.build_schematic_objects());
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

    fn check_sdf(&self, pos: Point, check_schematic: bool) -> SdfCheckRes {
        let mut sdf = f64::INFINITY;

        for object in self.marching_objs.iter() {
            if check_schematic && object.is_shematic() {
                continue;
            }
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
            match self.check_sdf(pos, true) {
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
                if dist < distance && dist > EPSILON {
                    object_and_dist = Some((obj.clone().upcast(), dist - EPSILON));
                    distance = dist;
                }
            }
        }
        object_and_dist
    }

    fn compute_ray_trajectory(&self, start: Point, dir: Vector) -> (ObjectType, Point) {
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

    fn march_shadow_ray(&self, start: Point, dir: Vector, max_depth: f64) -> bool {
        let mut depth = 0.0;

        loop {
            let pos = start + (dir * depth);
            match self.check_sdf(pos, false) {
                SdfCheckRes::Hit(_) => return true,
                SdfCheckRes::Miss(sdf) => depth += sdf,
            }
            if depth > max_depth || depth == f64::INFINITY {
                return false;
            }
        }
    }

    fn trace_shadow_ray(&self, start: Point, dir: Vector, max_depth: f64) -> bool {
        for obj in self.tracing_objs.iter() {
            if obj.is_shematic() {
                continue;
            }

            if let Some(dist) = obj.find_intersection(start, dir) {
                if dist < max_depth && dist > EPSILON {
                    return true;
                }
            }
        }
        false
    }

    pub fn compute_shadow_ray(&self, start: Point, dir: Vector, max_depth: f64) -> bool {
        self.march_shadow_ray(start, dir, max_depth) || self.trace_shadow_ray(start, dir, max_depth)
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
                let brightness = source.get_brightness(pos);

                let diffuse_part = brightness * angle_cos;
                let diffuse_color = obj_color * src_color * (diffuse_part);

                let half_angle_dir = (light_dir + dir).normalize();
                let dot_prod = normal * half_angle_dir;
                let coef = mtrl.flare_intensity * brightness;
                let specular_part = dot_prod.powi(mtrl.smoothness) * coef;
                let specular_color = src_color * specular_part;

                final_color += diffuse_color + specular_color;
            }
        }
        final_color
    }

    fn compute_subray(&self, start: Point, dir: Vector, refl_limit: i32) -> Color {
        let (object, pos) = self.compute_ray_trajectory(start, dir);
        let specularity = object.get_material(pos).specularity;
        let color = self.compute_lightning(object.clone(), pos, dir);

        if refl_limit == 0 || specularity == 0.0 {
            color
        } else {
            let start = pos;
            let dir = dir.reflect(object.get_normal(pos));
            let refl_limit = refl_limit - 1;
            color * (1.0 - specularity) + self.compute_subray(start, dir, refl_limit) * specularity
        }
    }

    pub fn compute_ray(&self, start: Point, dir: Vector) -> Color {
        self.compute_subray(start, dir, self.reflection_limit)
    }
}
