mod point;
pub use point::{Point, Vector, BASIS, ORIGIN};

mod color;
pub use color::{Color, RawColor};

mod matrix;
pub use matrix::Matrix;

mod material;
pub use material::Material;
pub use material::MaterialType;

mod ray;
pub use ray::Ray;

mod ray_context;
pub use ray_context::RayContext;

pub const EPSILON: f64 = 3_000.0 * f64::EPSILON;
