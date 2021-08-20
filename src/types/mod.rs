pub use image::Rgb;

mod point;
pub use point::Point;

mod matrix;
pub use matrix::Matrix;

pub mod objects;

pub type Color = Rgb<u8>;
pub type Coord = (i32, i32);
pub type SceneObjectType = dyn objects::SceneObject;
