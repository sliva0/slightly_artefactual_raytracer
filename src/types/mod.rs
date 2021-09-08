mod basic;
pub use basic::*;

mod objects;
pub use objects::*;

mod camera;
pub use camera::Camera;

mod scene;
pub use scene::Scene;

mod renderer;
pub use renderer::Renderer;

pub type Coord = (usize, usize);

pub const LAMP_RADIUS: f64 = 2.0;

pub const EPSILON: f64 = 1000.0 * f64::EPSILON;

pub const PORTIONS_SIZE: usize = 800;
