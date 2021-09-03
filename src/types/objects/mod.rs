use super::*;

mod object_types;
pub use object_types::*;

mod lamp;
pub use lamp::Lamp;

mod dummy_object;
mod room;
pub mod polygons;

pub use {dummy_object::DummyObject, room::Room, polygons::ObjectPolygon};