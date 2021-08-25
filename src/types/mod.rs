pub use std::sync::Arc;

pub use image::Rgb;

mod point;
pub use point::Point;

mod matrix;
pub use matrix::Matrix;

pub mod objects;

pub type Color = Rgb<u8>;
pub type Coord = (usize, usize);

pub type MarchingObjectType = Arc<dyn objects::MarchingObject>;
pub type TracingObjectType = Arc<dyn objects::TracingObject>;
pub type ObjectType = Arc<dyn objects::Object>;

pub enum CheckRes {
    Miss(f64),
    Hit(ObjectType),
}


pub const EPSILON: f64 = 0.00000001;