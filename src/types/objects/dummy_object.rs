use std::sync::Arc;

use super::*;

#[derive(Debug)]
pub struct DummyObject;

impl DummyObject {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl Object for DummyObject {
    fn color(&self, _pos: Point) -> Color {
        Color::ERR_COLOR
    }
    fn normal(&self, _pos: Point) -> Vector {
        ORIGIN
    }
    fn material(&self) -> Material {
        Material::ERR_MATERIAL
    }
    fn is_schematic(&self) -> bool {
        true
    }
}