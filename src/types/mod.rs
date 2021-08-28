pub use std::sync::Arc;

mod point;
pub use point::Point;

mod color;
pub use color::{Color, RawColor};

mod matrix;
pub use matrix::Matrix;

pub mod object_types;
pub use object_types::{MarchingObjectType, MetaTracingObjectType, ObjectType, TracingObjectType};

pub mod objects;

pub type Vector = Point;
pub type Coord = (usize, usize);
pub const EPSILON: f64 = f64::EPSILON;