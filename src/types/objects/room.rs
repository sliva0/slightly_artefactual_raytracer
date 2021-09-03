use std::sync::Arc;

use super::polygons::pair_with;
use super::*;

pub struct Room {
    pub size: f64,
    pub square_size: f64,
    pub colors: (Color, Color),
    pub material: Material,
}

impl Object for Room {
    fn get_color(&self, pos: Point) -> Color {
        let arr: [f64; 3] = pos.into();
        let sum: i32 = arr
            .iter()
            .map(|x| ((x + self.size) / self.square_size).floor() as i32)
            .sum();
        match sum % 2 {
            1 => self.colors.1,
            _ => self.colors.0,
        }
    }

    fn get_normal(&self, pos: Point) -> Vector {
        MarchingObject::get_normal(self, pos)
    }

    fn get_material(&self, _pos: Point) -> Material {
        self.material
    }
}

impl MarchingObject for Room {
    fn check_sdf(&self, pos: Point) -> f64 {
        let arr: [f64; 3] = pos.into();
        self.size - arr.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
}

impl MetaTracingObject for Room {
    fn get_color(&self, pos: Point) -> Color {
        let arr: [f64; 3] = pos.into();
        let sum: i32 = arr
            .iter()
            .map(|x| ((x + self.size) / self.square_size).floor() as i32)
            .sum();
        match sum % 2 {
            1 => self.colors.0,
            _ => self.colors.1,
        }
    }

    fn get_material(&self, _pos: Point) -> Material {
        self.material
    }

    fn build_objects<'a>(self: Arc<Self>) -> Vec<TracingObjectType<'a>> {
        let mut objects = Vec::with_capacity(12);

        let p0 = Point::new();
        let pairs = pair_with(
            (
                Vector { x: 1.0, ..p0 },
                Vector { y: 1.0, ..p0 },
                Vector { z: 1.0, ..p0 },
            ),
            |p1, p2| (p1, p2),
        );
        for (i, j) in [pairs.0, pairs.1, pairs.2] {
            for (dir, side) in [(i, j), (-i, -j)] {
                let dir = dir * self.size;
                objects.extend(ObjectPolygon::collect_cuboid_face(
                    Arc::downgrade(&self),
                    p0,
                    dir,
                    (side * self.size, (dir ^ side)),
                ));
            }
        }
        objects
    }
}
