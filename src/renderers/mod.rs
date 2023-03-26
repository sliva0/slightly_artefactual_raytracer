mod progress_bar;
use progress_bar::progress_bar;

mod renderer;
pub use renderer::Renderer;

mod subsampling_renderer;
pub use subsampling_renderer::{subsampling_func, SubsamplingRenderer};

type Coord = [usize; 2];
