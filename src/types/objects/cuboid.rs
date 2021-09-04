use std::sync::Arc;

use super::polygons::pair_with;
use super::*;

pub struct Cuboid {
    pub pos: Point,
    pub size: f64,
    pub color: Color,
    pub material: Material,
}

impl Object for Cuboid {
    fn get_color(&self, _pos: Point) -> Color {
        self.color
    }

    fn get_normal(&self, pos: Point) -> Vector {
        MarchingObject::get_normal(self, pos)
    }

    fn get_material(&self, _pos: Point) -> Material {
        self.material
    }
}

impl MarchingObject for Cuboid {
    fn check_sdf(&self, pos: Point) -> f64 {
        let arr: [f64; 3] = pos.into();
        self.size - arr.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
}

impl MetaTracingObject for Cuboid {
    fn get_color(&self, pos: Point) -> Color {
        Object::get_color(self, pos)
    }

    fn get_material(&self, _pos: Point) -> Material {
        self.material
    }

    fn build_objects<'a>(self: Arc<Self>) -> Vec<TracingObjectType<'a>> {
        let mut objects = Vec::with_capacity(12);

        let pairs = pair_with(
            [
                Vector { x: 1.0, ..Vector::P0 },
                Vector { y: 1.0, ..Vector::P0 },
                Vector { z: 1.0, ..Vector::P0 },
            ],
            |p1, p2| (p1, p2),
        );
        for (i, j) in pairs {
            for (dir, side) in [(i, j), (-i, -j)] {
                let dir = dir * self.size;
                objects.extend(ObjectPolygon::collect_cuboid_face(
                    Arc::downgrade(&self),
                    self.pos,
                    dir,
                    (side * self.size, (dir ^ side)),
                ));
            }
        }
        objects
    }
}
