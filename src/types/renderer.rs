#![allow(dead_code)]

use image::ImageBuffer;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use super::*;

pub struct Renderer {
    pub scene: Scene,
    pub cam: Camera,
    pub fov: f64,
    pub resolution: (usize, usize),
}

impl Renderer {
    fn f64_resolution(&self) -> (f64, f64) {
        (self.resolution.0 as f64, self.resolution.1 as f64)
    }

    fn ray(&self, pixel: Coord) -> Ray {
        let [x, y] = pixel;
        let [x, y] = [x as f64, y as f64];
        let (xs, ys) = self.f64_resolution();

        let (x, y) = ((x - xs / 2.0), -(y - ys / 2.0));
        let z = -ys / (self.fov.to_radians() / 2.0).tan();

        let dir = self.cam.rotate_ray(Vector::new(x, y, z)).normalize();
        Ray::new(self.cam.pos, dir)
    }

    fn render(&self) -> Vec<Vec<Color>> {
        let mut result = vec![vec![Color::ERR_COLOR; self.resolution.0]; self.resolution.1];

        result
            .par_iter_mut()
            .enumerate()
            .flat_map(|(line_num, line)| {
                line.par_iter_mut()
                    .enumerate()
                    .map(move |(column_num, pixel)| ([column_num, line_num], pixel))
            })
            .progress_count((self.resolution.0 * self.resolution.1) as u64)
            .for_each(|(coord, pixel)| {
                *pixel = self.scene.trace_ray(self.ray(coord));
            });

        result
    }

    pub fn render_and_save(&self, path: &str) {
        let image = self.render();
        let (x, y) = self.resolution;

        let img = ImageBuffer::from_fn(x as u32, y as u32, |x, y| {
            image[y as usize][x as usize].raw()
        });
        img.save(path).unwrap();
    }
}
