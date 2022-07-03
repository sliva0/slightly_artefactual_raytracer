#![allow(dead_code)]

use std::sync::Arc;

use super::polygons::get_basis_pairs;
use super::*;

#[derive(Debug)]
pub struct Cuboid {
    pub pos: Point,
    pub size: Point,
    pub color: Color,
    pub material: Material,
}

impl Cuboid {
    pub fn new(pos: Point, size: Point, color: Color, material: Material) -> Self {
        Self {
            pos,
            size,
            color,
            material,
        }
    }
}

impl Object for Cuboid {
    fn get_color(&self, _pos: Point) -> Color {
        self.color
    }

    fn get_normal(&self, pos: Point) -> Vector {
        MarchingObject::get_sdf_normal(self, pos)
    }

    fn get_material(&self) -> Material {
        self.material
    }
}

impl MarchingObject for Cuboid {
    fn get_sdf(&self, pos: Point) -> f64 {
        let arr: [f64; 3] = (pos.pdiv(self.size)).into();
        arr.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
}

impl MetaTracingObject for Cuboid {
    fn get_color(&self, pos: Point) -> Color {
        Object::get_color(self, pos)
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn build_objects(self: Arc<Self>) -> Vec<TracingObjectType> {
        let mut objects = Vec::with_capacity(12);

        for (dir, side) in get_basis_pairs() {
            let dir = dir.pmul(self.size);
            let sides = (
                side.pmul(self.size),
                (dir ^ side).normalize().pmul(self.size),
            );

            objects.extend(ObjectPolygon::collect_cuboid_face(
                &self,
                self.pos,
                dir,
                sides,
            ));
        }

        objects
    }
}
