use std::sync::Arc;

use super::*;

#[derive(Debug)]
pub struct Sphere {
    pub pos: Point,
    pub radius: f64,
    pub color: Color,
    pub material: Material,
    pub schematic: bool,
}

impl Sphere {
    pub fn new(pos: Point, radius: f64, color: Color, material: Material) -> Arc<Self> {
        Arc::new(Self {
            pos,
            radius,
            color,
            material,
            schematic: false,
        })
    }
}

impl Object for Sphere {
    fn color(&self, _pos: Point) -> Color {
        self.color
    }

    fn normal(&self, pos: Point) -> Vector {
        (self.pos >> pos).normalize()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn is_schematic(&self) -> bool {
        self.schematic
    }
}

impl MarchingObject for Sphere {
    fn sdf(&self, pos: Point) -> f64 {
        self.pos.dist(pos) - self.radius
    }
}

impl TracingObject for Sphere {
    fn find_intersection(&self, ray: Ray) -> Option<f64> {
        let r = self.radius;
        let l = ray.start >> self.pos;
        let s = l * ray.dir;
        let delta = (r * r + s * s - l * l).sqrt();
        if delta.is_nan() {
            None
        } else if s - delta > 0.0 {
            Some(s - delta)
        } else {
            Some(s + delta)
        }
    }
}
