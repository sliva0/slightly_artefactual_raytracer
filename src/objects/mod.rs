use super::basic::*;

mod object_types;
pub use object_types::*;

mod polygons;
use polygons::ObjectPolygon;

mod cuboid;
mod dummy_object;
mod lamp;
mod marching_helpers;
mod room;
mod sphere;

pub use {
    cuboid::Cuboid, dummy_object::DummyObject, lamp::Lamp, marching_helpers::Union, room::Room,
    sphere::Sphere,
};

pub const LAMP_RADIUS: f64 = 2.0;
