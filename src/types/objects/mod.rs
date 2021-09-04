use super::*;

mod object_types;
pub use object_types::*;

mod lamp;
pub use lamp::Lamp;

pub mod polygons;
pub use polygons::ObjectPolygon;

mod cuboid;
mod dummy_object;
mod room;
mod sphere;

pub use {cuboid::Cuboid, dummy_object::DummyObject, room::Room, sphere::Sphere};
