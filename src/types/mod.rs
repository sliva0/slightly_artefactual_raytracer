pub use std::sync::Arc;

mod point;
pub use point::Point;

mod color;
pub use color::{Color, RawColor};

mod matrix;
pub use matrix::Matrix;

pub mod objects;
pub type Coord = (usize, usize);

pub type MarchingObjectType = Arc<dyn objects::MarchingObject>;
pub type TracingObjectType = Arc<dyn objects::TracingObject>;
pub type ObjectType = Arc<dyn objects::Object>;

pub const EPSILON: f64 = 0.00000001;
