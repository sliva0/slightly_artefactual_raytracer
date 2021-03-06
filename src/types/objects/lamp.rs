use std::sync::Arc;

use super::*;

pub struct Lamp {
    pub pos: Point,
    pub color: Color,
    pub brightness: f64,
}

impl Lamp {
    pub fn new(pos: Point, color: Color, brightness: f64) -> Arc<Self> {
        Arc::new(Self {
            pos,
            color,
            brightness,
        })
    }
}

impl LightSource for Lamp {
    fn _light_dir(&self, pos: Point) -> Vector {
        (self.pos >> pos).normalize()
    }

    fn dist(&self, pos: Point) -> f64 {
        self.pos.dist(pos)
    }

    fn _brightness(&self, _pos: Point) -> f64 {
        self.brightness
    }

    fn color(&self, _pos: Point) -> Color {
        self.color
    }

    fn build_schematic_objects(self: Arc<Self>) -> Vec<TracingObjectType> {
        vec![Arc::new(Sphere {
            pos: self.pos,
            radius: LAMP_RADIUS,
            color: self.color,
            material: Material::ERR_MATERIAL,
            schematic: true,
        })]
    }
}
