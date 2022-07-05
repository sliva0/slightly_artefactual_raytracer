#![allow(dead_code)]

use super::*;

#[derive(Debug)]
pub struct Union {
    objects: Vec<MarchingObjectType>,
}

impl Union {
    pub fn new(objects: Vec<MarchingObjectType>) -> Self {
        assert!(!objects.is_empty(), "Union must be non-empty");
        Self { objects }
    }

    pub fn new_lens(
        pos: Point,
        dir: Vector,
        lens_radius: f64,
        thickness: f64,
        color: Color,
        material: Material,
    ) -> Self {
        let th2 = thickness / 2.0;
        let tan_t = lens_radius / th2;
        let cos_t = 1.0 / (1.0 + tan_t * tan_t).sqrt();

        let radius = lens_radius.hypot(th2) / 2.0 / cos_t;
        let shift = dir.normalize() * (radius - th2);
        Self::new(vec![
            Sphere::new(pos + shift, radius, color, material),
            Sphere::new(pos - shift, radius, color, material),
        ])
    }
}

impl Object for Union {
    fn color(&self, pos: Point) -> Color {
        self.objects[0].color(pos)
    }

    fn normal(&self, pos: Point) -> Vector {
        self.sdf_normal(pos)
    }

    fn material(&self) -> Material {
        self.objects[0].material()
    }
}

impl MarchingObject for Union {
    fn sdf(&self, pos: Point) -> f64 {
        self.objects
            .iter()
            .map(|obj| obj.sdf(pos))
            .fold(f64::INFINITY, f64::min)
    }
}
