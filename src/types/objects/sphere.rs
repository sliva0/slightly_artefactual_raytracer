#![allow(dead_code)]

use super::*;
pub struct Sphere {
    pub pos: Point,
    pub radius: f64,
    pub color: Color,
    pub material: Material,
    pub schematic: bool,
}

impl Sphere {
    pub fn new(pos: Point, radius: f64, color: Color, material: Material) -> Self {
        Self {
            pos,
            radius,
            color,
            material,
            schematic: false,
        }
    }
}

impl Object for Sphere {
    fn get_color(&self, _pos: Point) -> Color {
        self.color
    }

    fn get_normal(&self, pos: Point) -> Vector {
        (self.pos >> pos).normalize()
    }

    fn get_material(&self, _pos: Point) -> Material {
        self.material
    }

    fn is_shematic(&self) -> bool {
        self.schematic
    }
}

impl MarchingObject for Sphere {
    fn check_sdf(&self, pos: Point) -> f64 {
        self.pos.dist(pos) - self.radius
    }
}

impl TracingObject for Sphere {
    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        let r = self.radius;
        let l = start >> self.pos;
        let s = l * dir;
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
