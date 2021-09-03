use super::*;

pub struct Lamp {
    pub pos: Point,
    pub color: Color,
    pub brightness: f64,
}

impl LightSource for Lamp {
    fn get_light_dir(&self, scene: &Scene, pos: Point) -> Option<Vector> {
        let dir = (self.pos >> pos).normalize();
        let dist = self.pos.dist(pos);
        if scene.compute_shadow_ray(pos, -dir, dist) {
            None
        } else {
            Some(dir)
        }
        
    }
    fn get_brightness(&self, pos: Point) -> f64 {
        let dist = self.pos.dist(pos);
        self.brightness / (dist * dist)
    }
    fn get_color(&self, _pos: Point) -> Color {
        self.color
    }
}
