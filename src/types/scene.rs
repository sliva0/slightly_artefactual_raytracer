use super::*;

enum SdfResult {
    Miss(f64),
    Hit(f64, MarchingObjectType),
}

#[derive(Debug)]
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
    fn new_tracing(obj: &TracingObjectType, depth: f64, ray: Ray) -> Option<Self> {
        let object = obj.clone().upcast();
        let point = ray.point(depth);
        let normal = object.normal(point);
        let shift = normal * EPSILON.copysign(normal * ray.dir);

        Some(Self {
            object,
            depth,
            point: ray.point(depth - EPSILON),
            crossed_point: point + shift,
        })
    }

    fn new_marching(obj: &MarchingObjectType, error: f64, depth: f64, ray: Ray) -> Option<Self> {
        let object = obj.clone().upcast();
        let point = ray.point(depth);
        let normal = object.normal(point);
        let shift = normal * (error + EPSILON).copysign(normal * ray.dir);

        Some(Self {
            object,
            depth,
            point,
            crossed_point: point + shift,
        })
    }

    fn color(&self) -> Color {
        self.object.color(self.point)
    }
    fn normal(&self) -> Vector {
        self.object.normal(self.point)
    }
    fn material(&self) -> Material {
        self.object.material()
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
        for object in self.meta_objs.iter().cloned() {
            self.tracing_objs.extend(object.build_objects());
        }
        for lamp in self.lamps.iter().cloned() {
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

    fn sdf<const S: bool>(&self, pos: Point) -> SdfResult {
        let mut sdf = f64::INFINITY;

        for object in self.marching_objs.iter() {
            if !S && object.is_schematic() {
                continue;
            }
            sdf = sdf.min(object.sdf(pos).abs());
            if sdf < EPSILON {
                return SdfResult::Hit(sdf, object.clone());
            }
        }
        SdfResult::Miss(sdf)
    }

    fn march_ray<const S: bool>(&self, ray: Ray, max_depth: f64) -> Option<Hit> {
        let mut depth = EPSILON;

        loop {
            let pos = ray.point(depth);
            match self.sdf::<S>(pos) {
                SdfResult::Hit(sdf, obj) => return Hit::new_marching(&obj, sdf, depth, ray),
                SdfResult::Miss(sdf) => depth += sdf,
            }
            if depth > max_depth || depth.is_infinite() {
                return None;
            }
        }
    }

    fn cast_ray(&self, ray: Ray) -> Option<Hit> {
        let mut distance = f64::INFINITY;
        let mut hit = None;

        for obj in self.tracing_objs.iter() {
            if let Some(dist) = obj.find_intersection(ray) {
                if dist < distance && dist > EPSILON {
                    hit = Hit::new_tracing(obj, dist, ray);
                    distance = dist;
                }
            }
        }
        hit
    }

    fn march_shadow_ray(&self, ray: Ray, max_depth: f64) -> bool {
        self.march_ray::<false>(ray, max_depth).is_some()
    }

    fn cast_shadow_ray(&self, ray: Ray, max_depth: f64) -> bool {
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

    fn compute_ray(&self, ray: Ray) -> Hit {
        let hit = self.cast_ray(ray).unwrap_or_default();
        self.march_ray::<true>(ray, hit.depth).unwrap_or(hit)
    }

    pub fn compute_shadow_ray(&self, ray: Ray, max_depth: f64) -> bool {
        self.march_shadow_ray(ray, max_depth) || self.cast_shadow_ray(ray, max_depth)
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
            if let Some(light_dir) = source.light_dir(self, pos) {
                let angle_cos = -light_dir * normal;
                if angle_cos <= 0.0 {
                    continue;
                }
                let src_color = source.color(pos);
                let brightness = source.brightness(pos);

                let diffuse_color = obj_color * src_color * (mtrl.diffuse * brightness * angle_cos);

                let half_angle_dir = (light_dir + dir).normalize();
                let specular_mp = (normal * half_angle_dir).powi(mtrl.shininess); // multiplier
                let specular_color = src_color * (specular_mp * mtrl.specular * brightness);

                final_color += diffuse_color + specular_color;
            }
        }
        final_color
    }

    fn compute_reflected_case(&self, ray: Ray, hit: &Hit, context: &RayContext) -> Color {
        let refl_ray = ray.reflect(hit.point, hit.normal());
        let refl_context = context.reflected_subray_context();
        self.trace_subray(refl_ray, refl_context)
    }

    fn compute_refracted_case(&self, ray: Ray, hit: Hit, context: &RayContext) -> Color {
        let refl_color = self.compute_reflected_case(ray, &hit, context);
        let normal = hit.normal();
        let refr_context = context.refracted_subray_context(hit.object);
        match ray.compute_reflectance_and_refract(
            normal,
            context.refr_index,
            refr_context.refr_index,
            hit.crossed_point,
        ) {
            None => refl_color, // total internal reflection
            Some((reflectance, refr_ray)) => {
                let refr_color = self.trace_subray(refr_ray, refr_context);
                refr_color * (1.0 - reflectance) + refl_color * reflectance
            }
        }
    }

    fn trace_subray(&self, ray: Ray, context: RayContext) -> Color {
        let hit = self.compute_ray(ray);
        let color = self.compute_lightning(&hit, ray.dir);

        if context.limit_reached() {
            return color;
        }
        match hit.material().m_type {
            Common => color,
            Reflective { reflectance } => {
                let refl_color = self.compute_reflected_case(ray, &hit, &context);
                color * (1.0 - reflectance) + refl_color * reflectance
            }
            Refractive {  surface_transparency, index: _ } => {
                let refr_color = self.compute_refracted_case(ray, hit, &context);
                color * (1.0 - surface_transparency) + refr_color * surface_transparency
            }
        }
    }

    pub fn trace_ray(&self, ray: Ray) -> Color {
        self.trace_subray(ray, RayContext::new(self.reflection_limit))
    }
}
