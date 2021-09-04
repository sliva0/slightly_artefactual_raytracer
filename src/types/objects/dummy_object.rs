use super::*;

pub struct DummyObject();

impl Object for DummyObject {
    fn get_color(&self, _pos: Point) -> Color {
        Color::ERR_COLOR
    }
    fn get_normal(&self, _pos: Point) -> Vector {
        Vector::P0
    }
    fn get_material(&self, _pos: Point) -> Material {
        Material::ERR_MATERIAL
    }
    fn is_shematic(&self) -> bool {
        true
    }
}