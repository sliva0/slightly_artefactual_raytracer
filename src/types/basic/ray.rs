use super::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub start: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn new(start: Point, dir: Vector) -> Self {
        Self { start, dir }
    }

    pub fn get_point(&self, dist: f64) -> Point {
        self.start + self.dir * dist
    }

    pub fn reflect(&self, pos: Point, normal: Vector) -> Self {
        Self::new(pos, self.dir.reflect(normal))
    }
}
