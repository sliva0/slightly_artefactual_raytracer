mod basic;
pub use basic::*;

mod objects;
pub use objects::*;

mod camera;
pub use camera::Camera;

mod scene;
pub use scene::Scene;

mod renderers;
pub use renderers::*;

pub type Coord = [usize; 2];

pub const LAMP_RADIUS: f64 = 2.0;

pub const EPSILON: f64 = 3_000.0 * f64::EPSILON;
