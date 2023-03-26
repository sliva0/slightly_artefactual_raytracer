use super::{Point, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub start: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn new(start: Point, dir: Vector) -> Self {
        Self { start, dir }
    }

    pub fn point(&self, dist: f64) -> Point {
        self.start + self.dir * dist
    }

    pub fn reflect(&self, pos: Point, normal: Vector) -> Self {
        Self::new(pos, self.dir.reflect(normal))
    }

    pub fn compute_reflectance_and_refract(
        &self,
        normal: Vector,
        n1: f64,
        n2: f64,
        crossed_point: Point,
    ) -> Option<(f64, Self)> {
        self.dir
            .compute_reflectance_and_refract(normal, n1, n2)
            .map(|(refl, dir)| (refl, Self::new(crossed_point, dir)))
    }
}
