use super::*;

enum SdfResult {
    Miss(f64),
    Hit(f64, MarchingObjectType),
}

struct Hit {
    object: ObjectType,
    depth: f64,
    point: Point,
    crossed_point: Point,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            object: DummyObject::new(),
            depth: f64::INFINITY,
            point: ORIGIN,
            crossed_point: ORIGIN,
        }
    }
}

impl Hit {
    fn new_tracing(obj: &TracingObjectType, depth: f64, ray: Ray) -> Self {
        Self {
            object: obj.clone().upcast(),
            depth,
            point: ray.get_point(depth - EPSILON),
            crossed_point: ray.get_point(depth),
        }
    }

    fn new_marching(obj: &MarchingObjectType, error: f64, depth: f64, ray: Ray) -> Self {
        let object = obj.clone().upcast();
        let point = ray.get_point(depth);
        let normal = object.get_normal(point);
        let crossed_point = point + normal * (error + EPSILON).copysign(normal * ray.dir);
        Self {
            object,
            depth,
            point,
            crossed_point,
        }
    }

    fn color(&self) -> Color {
        self.object.get_color(self.point)
    }
    fn normal(&self) -> Vector {
        self.object.get_normal(self.point)
    }
    fn material(&self) -> Material {
        self.object.get_material(self.point)
    }
}

pub struct Scene {
    marching_objs: Vec<MarchingObjectType>,
    tracing_objs: Vec<TracingObjectType>,
    meta_objs: Vec<MetaTracingObjectType>,
    lamps: Vec<LightSourceType>,
    reflection_limit: i32,
}

impl Scene {
    fn build_meta_objects(&mut self) {
        for object in self.meta_objs.to_vec() {
            self.tracing_objs.extend(object.build_objects());
        }
        for lamp in self.lamps.to_vec() {
            self.tracing_objs.extend(lamp.build_schematic_objects());
        }
    }

    pub fn new(
        marching_objs: Vec<MarchingObjectType>,
        tracing_objs: Vec<TracingObjectType>,
        meta_objs: Vec<MetaTracingObjectType>,
        lamps: Vec<LightSourceType>,
        reflection_limit: i32,
    ) -> Self {
        let mut scene = Self {
            marching_objs,
            tracing_objs,
            meta_objs,
            lamps,
            reflection_limit,
        };
        scene.build_meta_objects();
        scene
    }

    fn get_sdf<const S: bool>(&self, pos: Point) -> SdfResult {
        let mut sdf = f64::INFINITY;

        for object in self.marching_objs.iter() {
            if !S && object.is_schematic() {
                continue;
            }
            sdf = sdf.min(object.get_sdf(pos));
            if sdf < EPSILON {
                return SdfResult::Hit(sdf, object.clone());
            }
        }
        SdfResult::Miss(sdf)
    }

    fn march_ray<const S: bool>(&self, ray: Ray, max_depth: f64) -> Option<Hit> {
        let mut depth = EPSILON;

        loop {
            let pos = ray.get_point(depth);
            match self.get_sdf::<S>(pos) {
                SdfResult::Hit(sdf, obj) => return Some(Hit::new_marching(&obj, sdf, depth, ray)),
                SdfResult::Miss(sdf) => depth += sdf,
            }
            if depth > max_depth || depth.is_infinite() {
                return None;
            }
        }
    }

    fn trace_ray(&self, ray: Ray) -> Option<Hit> {
        let mut distance = f64::INFINITY;
        let mut hit = None;

        for obj in self.tracing_objs.iter() {
            if let Some(dist) = obj.find_intersection(ray) {
                if dist < distance && dist > EPSILON {
                    hit = Some(Hit::new_tracing(&obj, dist, ray));
                    distance = dist;
                }
            }
        }
        hit
    }

    fn trace_shadow_ray(&self, ray: Ray, max_depth: f64) -> bool {
        for obj in self.tracing_objs.iter() {
            if obj.is_schematic() {
                continue;
            }

            if let Some(dist) = obj.find_intersection(ray) {
                if dist < max_depth && dist > EPSILON {
                    return true;
                }
            }
        }
        false
    }

    fn cast_ray(&self, ray: Ray) -> Hit {
        let hit = self.trace_ray(ray).unwrap_or_default();
        self.march_ray::<true>(ray, hit.depth).unwrap_or(hit)
    }

    pub fn compute_shadow_ray(&self, ray: Ray, max_depth: f64) -> bool {
        self.march_ray::<false>(ray, max_depth).is_some() || self.trace_shadow_ray(ray, max_depth)
    }

    fn compute_lightning(&self, hit: &Hit, dir: Vector) -> Color {
        let obj_color = hit.color();
        if hit.object.is_schematic() {
            return obj_color;
        }

        let normal = hit.normal();
        let mtrl = hit.material();
        let pos = hit.point;
        let mut final_color = obj_color * mtrl.ambient;

        for source in self.lamps.iter() {
            if let Some(light_dir) = source.get_light_dir(self, pos) {
                let angle_cos = -light_dir * normal;
                if angle_cos <= 0.0 {
                    continue;
                }
                let src_color = source.get_color(pos);
                let brightness = source.get_brightness(pos);

                let diffuse_color = obj_color * src_color * (mtrl.diffuse * brightness * angle_cos);

                let half_angle_dir = (light_dir + dir).normalize();
                let specular_mp = (normal * half_angle_dir).powi(mtrl.shininess); // multiplier
                let specular_color = src_color * (specular_mp * mtrl.specular * brightness);

                final_color += diffuse_color + specular_color;
            }
        }
        final_color
    }

    fn compute_reflected_ray(&self, hit: &Hit, ray: Ray, refl_limit: i32) -> Color {
        let ray = ray.reflect(hit.point, hit.normal());
        self.compute_subray(ray, refl_limit - 1)
    }

    fn compute_subray(&self, ray: Ray, refl_limit: i32) -> Color {
        let hit = self.cast_ray(ray);
        let color = self.compute_lightning(&hit, ray.dir);

        if refl_limit == 0 {
            return color;
        }
        match hit.material().type_ {
            DefaultType => color,
            ReflectiveType { reflectance } => {
                let r_color = self.compute_reflected_ray(&hit, ray, refl_limit);
                color * (1.0 - reflectance) + r_color * reflectance
            }
            RefractiveType {
                index: _,
                transparency: _,
            } => Color::ERR_COLOR,
        }
    }

    pub fn compute_ray(&self, ray: Ray) -> Color {
        self.compute_subray(ray, self.reflection_limit)
    }
}
