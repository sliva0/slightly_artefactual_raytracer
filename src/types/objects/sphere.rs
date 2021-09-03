use super::*;
pub struct Sphere {
    pub pos: Point,
    pub radius: f64,
    pub color: Color,
    pub material: Material,
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
        #[allow(illegal_floating_point_literal_pattern)]
        match (r * r + s * s - l * l).sqrt() {
            f64::NAN => None,
            delta => Some(if s - delta > 0.0 {
                s - delta
            } else {
                s + delta
            }),
        }
    }
}
