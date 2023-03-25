mod basic;
pub use basic::*;

mod objects;
pub use objects::*;

mod camera;
pub use camera::Camera;

mod scene;
pub use scene::Scene;

mod progress;

mod renderer;
pub use renderer::Renderer;

mod subsampling_renderer;
pub use subsampling_renderer::{subsampling_func, SubsamplingRenderer};

pub type Coord = [usize; 2];

pub const LAMP_RADIUS: f64 = 2.0;

pub const EPSILON: f64 = 3_000.0 * f64::EPSILON;
