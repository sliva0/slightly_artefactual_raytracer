use crossbeam_utils::thread;
use image::ImageBuffer;

use super::{Camera, Color, Coord, Point, Scene};

pub struct Renderer<'a> {
    pub scene: Scene<'a>,
    pub cam: Camera,
    pub fov: f64,
    pub resolution: (usize, usize),
}

impl<'a> Renderer<'a> {
    fn i32_res(&self) -> (i32, i32) {
        (self.resolution.0 as i32, self.resolution.1 as i32)
    }

    fn get_ray_dir(&self, pixel: Coord) -> Point {
        let (x, y) = (pixel.0 as i32, pixel.1 as i32);
        let (xs, ys) = self.i32_res();

        let (x, y) = ((x - xs / 2) as f64, -(y - ys / 2) as f64);
        let z = -(ys as f64) / (self.fov.to_radians() / 2.0).tan();

        (self.cam.look_op * Point { x, y, z }).normalize()
    }

    fn render_line(&self, line_num: usize, line: &mut Vec<Color>) {
        let (columns, lines) = self.resolution;

        for i in 0..columns {
            let ray_dir = self.get_ray_dir((i, line_num));
            line.push(self.scene.compute_ray_reflections(self.cam.pos, ray_dir));
        }
        println!("line: {} / {}", line_num + 1, lines);
    }

    fn render(&self) -> Vec<Vec<Color>> {
        let lines = self.resolution.1;
        let mut image = Vec::with_capacity(lines);
        image.resize_with(lines, Vec::new);

        thread::scope(|scope| {
            for (line_num, line) in image.iter_mut().enumerate() {
                scope.spawn(move |_| self.render_line(line_num, line));
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
