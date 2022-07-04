use std::sync::Arc;

use super::polygons::get_basis_pairs;
use super::*;

#[derive(Debug)]
pub struct Room {
    pub size: f64,
    pub square_size: f64,
    pub colors: (Color, Color),
    pub material: Material,
}

impl Room {
    pub fn new(
        size: f64,
        square_size: f64,
        colors: (Color, Color),
        material: Material,
    ) -> Arc<Self> {
        Arc::new(Self {
            size,
            square_size,
            colors,
            material,
        })
    }
}

impl Object for Room {
    fn get_color(&self, pos: Point) -> Color {
        let sum: i32 = pos
            .iter()
            .map(|x| ((x + self.size) / self.square_size).floor() as i32)
            .sum();
        match sum % 2 {
            1 => self.colors.1,
            _ => self.colors.0,
        }
    }

    fn get_normal(&self, pos: Point) -> Vector {
        MarchingObject::get_sdf_normal(self, pos)
    }

    fn get_material(&self) -> Material {
        self.material
    }
}

impl MarchingObject for Room {
    fn get_sdf(&self, pos: Point) -> f64 {
        self.size - pos.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
}

impl MetaTracingObject for Room {
    fn get_color(&self, pos: Point) -> Color {
        Object::get_color(self, pos)
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn build_objects(self: Arc<Self>) -> Vec<TracingObjectType> {
        let mut objects = Vec::with_capacity(12);

        let size = -self.size;
        for (dir, side) in get_basis_pairs() {
            let dir = dir * size;
            objects.extend(ObjectPolygon::collect_cuboid_face(
                &self,
                ORIGIN,
                dir,
                (side * size, (dir ^ side)),
            ));
        }
        objects
    }
}
