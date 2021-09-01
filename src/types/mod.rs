pub use std::sync::Arc;

mod point;
pub use point::Point;

mod color;
pub use color::{Color, RawColor};

mod matrix;
pub use matrix::Matrix;

mod object_types;
pub use object_types::{MarchingObjectType, MetaTracingObjectType, ObjectType, TracingObjectType};

pub mod objects;

mod camera;
pub use camera::Camera;

mod scene;
pub use scene::Scene;

mod renderer;
pub use renderer::Renderer;

mod lamp;
pub use lamp::Lamp;

pub type Vector = Point;
pub type Coord = (usize, usize);

pub const EPSILON: f64 = 500.0 * f64::EPSILON;
pub const UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
