use std::sync::Arc;

use super::polygons::basis_pairs;
use super::*;

#[derive(Debug)]
pub struct Cuboid {
    pub pos: Point,
    pub size: Point,
    pub color: Color,
    pub material: Material,
}

impl Cuboid {
    pub fn new(pos: Point, size: Point, color: Color, material: Material) -> Arc<Self> {
        Arc::new(Self {
            pos,
            size,
            color,
            material,
        })
    }
}

impl Object for Cuboid {
    fn color(&self, _pos: Point) -> Color {
        self.color
    }

    fn normal(&self, pos: Point) -> Vector {
        MarchingObject::sdf_normal(self, pos)
    }

    fn material(&self) -> Material {
        self.material
    }
}

impl MarchingObject for Cuboid {
    fn sdf(&self, pos: Point) -> f64 {
        (pos - self.pos)
            .iter()
            .zip(self.size)
            .map(|(x, s)| x.abs() - s)
            .fold(f64::INFINITY, f64::min)
    }
}

impl MetaTracingObject for Cuboid {
    fn build_objects(self: Arc<Self>) -> Vec<TracingObjectType> {
        let mut objects = Vec::with_capacity(12);

        for (dir, side) in basis_pairs() {
            let dir = dir.pmul(self.size);
            let sides = (
                side.pmul(self.size),
                (dir ^ side).normalize().pmul(self.size),
            );

            objects.extend(ObjectPolygon::collect_cuboid_face(
                &self, self.pos, dir, sides,
            ));
        }

        objects
    }
}
