use super::{object_types::LightSource, Color, Point, Vector};

pub struct Lamp {
    pub pos: Point,
    pub color: Color,
    pub brightness: f64,
}

impl LightSource for Lamp {
    fn get_light_dir(&self, pos: Point) -> Option<Vector> {
        Some((self.pos >> pos).normalize())
    }
    fn get_brightness(&self, pos: Point) -> f64 {
        let dist = self.pos.dist(pos);
        self.brightness / (dist * dist)
    }
    fn get_color(&self, _pos: Point) -> Color {
        self.color
    }
}