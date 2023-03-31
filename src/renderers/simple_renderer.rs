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
        let [width, height] = self.scene.resolution;
        let mut result = vec![vec![Color::ERR_COLOR; width]; height];

        result
            .par_iter_mut()
            .enumerate()
            .flat_map(|(yi, line)| {
                line.par_iter_mut()
                    .enumerate()
                    .map(move |(xi, pixel)| ([xi, yi], pixel))
            })
            .progress_with(progress_bar(width * height, "Rendering"))
            .for_each(|(coord, pixel)| {
                *pixel = self.scene.objs.trace_ray(self.scene.ray(coord));
            });

        result
    }

    pub fn render(&self) -> ImageBuffer<RawColor, Vec<u8>> {
        let image = self.render_raw();
        let [width, height] = self.scene.resolution;

        ImageBuffer::from_fn(width as u32, height as u32, |xi, yi| {
            image[yi as usize][xi as usize].into()
        })
    }
}
