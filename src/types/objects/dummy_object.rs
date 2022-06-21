use std::sync::Arc;

use super::*;

pub struct DummyObject;

impl DummyObject {
    pub fn new() -> ObjectType {
        Arc::new(Self)
    }
}

impl Object for DummyObject {
    fn get_color(&self, _pos: Point) -> Color {
        Color::ERR_COLOR
    }
    fn get_normal(&self, _pos: Point) -> Vector {
        ORIGIN
    }
    fn get_material(&self, _pos: Point) -> Material {
        Material::ERR_MATERIAL
    }
    fn is_schematic(&self) -> bool {
        true
    }
}