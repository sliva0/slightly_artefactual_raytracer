#![allow(dead_code)]
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use crossbeam_utils::thread;
use image::ImageBuffer;

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

    fn get_ray(&self, pixel: Coord) -> Ray {
        let (x, y) = (pixel.0 as f64, pixel.1 as f64);
        let (xs, ys) = self.f64_resolution();

        let (x, y) = ((x - xs / 2.0), -(y - ys / 2.0));
        let z = -(ys as f64) / (self.fov.to_radians() / 2.0).tan();

        let dir = self.cam.rotate_ray(Vector::new(x, y, z)).normalize();
        Ray::new(self.cam.pos, dir)
    }

    fn render_line(&self, line_num: usize, line: &mut Vec<Color>, tx: SyncSender<()>) {
        let columns = self.resolution.0;
        let mut pixel_cnt = 0;

        for i in 0..columns {
            let ray = self.get_ray((i, line_num));
            line.push(self.scene.trace_ray(ray));

            pixel_cnt += 1;
            if pixel_cnt % PORTIONS_SIZE == 0 {
                tx.send(()).unwrap()
            }
        }
    }

    fn progress_bar(&self, rx: Receiver<()>) {
        let (columns, lines) = self.resolution;
        let portion_amount = columns / PORTIONS_SIZE * lines;
        for i in 1..=portion_amount {
            rx.recv().unwrap();
            println!("{} / {}", i, portion_amount);
        }
        println!("done");
    }

    fn render(&self) -> Vec<Vec<Color>> {
        let lines = self.resolution.1;
        let mut image = Vec::with_capacity(lines);
        image.resize_with(lines, Vec::new);

        thread::scope(|scope| {
            let (tx, rx) = sync_channel(8);
            scope.spawn(move |_| self.progress_bar(rx));

            for (line_num, line) in image.iter_mut().enumerate() {
                let txc = tx.clone();
                scope.spawn(move |_| self.render_line(line_num, line, txc));
            }
        })
        .unwrap();

        image
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
