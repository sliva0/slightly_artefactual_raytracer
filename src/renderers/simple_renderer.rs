use image::ImageBuffer;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use super::{progress_bar, Scene};
use crate::*;

pub struct SimpleRenderer {
    scene: Scene,
}

impl SimpleRenderer {
    fn render_raw(&self) -> Vec<Vec<Color>> {
        let [x, y] = self.scene.resolution;
        let mut result = vec![vec![Color::ERR_COLOR; x]; y];

        result
            .par_iter_mut()
            .enumerate()
            .flat_map(|(line_num, line)| {
                line.par_iter_mut()
                    .enumerate()
                    .map(move |(column_num, pixel)| ([column_num, line_num], pixel))
            })
            .progress_with(progress_bar(x * y, "Rendering"))
            .for_each(|(coord, pixel)| {
                *pixel = self.scene.objs.trace_ray(self.scene.ray(coord));
            });

        result
    }

    pub fn render(&self) -> ImageBuffer<RawColor, Vec<u8>> {
        let image = self.render_raw();
        let [x, y] = self.scene.resolution;

        ImageBuffer::from_fn(x as u32, y as u32, |x, y| {
            image[y as usize][x as usize].into()
        })
    }
}
