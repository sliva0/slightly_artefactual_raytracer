use std::sync::Arc;

use super::polygons::basis_pairs;
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
    fn color(&self, pos: Point) -> Color {
        let sum: i32 = pos
            .iter()
            .map(|x| ((x + self.size) / self.square_size).floor() as i32)
            .sum();
        match sum % 2 {
            1 => self.colors.1,
            _ => self.colors.0,
        }
    }

    fn normal(&self, pos: Point) -> Vector {
        MarchingObject::sdf_normal(self, pos)
    }

    fn material(&self) -> Material {
        self.material
    }
}

impl MarchingObject for Room {
    fn sdf(&self, pos: Point) -> f64 {
        self.size - pos.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
}

impl MetaTracingObject for Room {
    fn build_objects(self: Arc<Self>) -> Vec<TracingObjectType> {
        let mut objects = Vec::with_capacity(12);

        let size = -self.size;
        for (dir, side) in basis_pairs() {
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
