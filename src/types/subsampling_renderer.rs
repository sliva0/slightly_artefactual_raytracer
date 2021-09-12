use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use crossbeam_utils::thread;
use image::ImageBuffer;

use super::*;

type SubsamplingFunc = Box<dyn Fn(Coord) -> bool>;
type Image = Vec<Vec<Pixel>>;

pub fn subsampling_func(subsample_number: i32) -> SubsamplingFunc {
    Box::new(match subsample_number {
        1 => |_| true,
        2 => |(x, y)| (x + y) % 2 == 0,
        3 => |(x, y)| (x + y) % 3 == 0,
        4 => |(x, y)| (x + y * 2) % 4 == 0,
        5 => |(x, y)| (x + y * 2) % 5 == 0,
        _ => panic!("incorrect subsample number"),
    })
}

enum Pixel {
    PixelToRender,
    PixelToInterpolate,
    RenderedPixel(Color),
    InterpolatedPixel(Color),
}

impl Pixel {
    fn get_color(&self) -> Option<Color> {
        if let RenderedPixel(col) | InterpolatedPixel(col) = self {
            Some(*col)
        } else {
            None
        }
    }
}

use Pixel::*;

pub struct SubsamplingRenderer<'a> {
    pub scene: Scene<'a>,
    pub cam: Camera,
    pub fov: f64,
    pub resolution: (usize, usize),
    pub subsampling_limit: f64,
    pub supersampling_multiplier: usize,
}

impl<'a> SubsamplingRenderer<'a> {
    fn get_resolution(&self) -> (usize, usize) {
        (
            (self.resolution.0 * self.supersampling_multiplier),
            (self.resolution.1 * self.supersampling_multiplier),
        )
    }

    fn f64_resolution(&self) -> (f64, f64) {
        let (x, y) = self.get_resolution();
        (x as f64, y as f64)
    }

    fn is_edge(&self, pixel: Coord) -> bool {
        let (x, y) = pixel;
        let (xs, ys) = self.get_resolution();
        x == 0 || y == 0 || x == xs - 1 || y == ys - 1
    }

    fn get_ray_dir(&self, pixel: Coord) -> Vector {
        let (x, y) = (pixel.0 as f64, pixel.1 as f64);
        let (xs, ys) = self.f64_resolution();

        let (x, y) = ((x - xs / 2.0), -(y - ys / 2.0));
        let z = -(ys as f64) / (self.fov.to_radians() / 2.0).tan();

        self.cam.rotate_ray(Vector { x, y, z }).normalize()
    }

    fn render_line(&self, line_num: usize, line: &mut Vec<Pixel>, tx: SyncSender<()>) {
        let columns = self.get_resolution().0;
        let mut pixel_cnt = 0;

        for i in 0..columns {
            if let PixelToRender = line[i] {
                let ray_dir = self.get_ray_dir((i, line_num));
                line[i] = RenderedPixel(self.scene.compute_ray(self.cam.pos, ray_dir));
            }

            pixel_cnt += 1;
            if pixel_cnt % PORTIONS_SIZE == 0 {
                tx.send(()).unwrap()
            }
        }
    }

    fn collect_neighbors(x: usize, y: usize, image: &Image) -> Vec<Color> {
        let mut colors = vec![];
        for yi in (y - 1)..=(y + 1) {
            for xi in (x - 1)..=(x + 1) {
                if let RenderedPixel(color) = image[xi][yi] {
                    colors.push(color);
                }
            }
        }
        colors
    }

    fn interpolate_pixel(&self, x: usize, y: usize, image: &mut Image) -> Pixel {
        let colors = Self::collect_neighbors(x, y, image);

        if Color::colors_diff(colors.clone()) > self.subsampling_limit {
            PixelToRender
        } else {
            InterpolatedPixel(Color::colors_avg(colors))
        }
    }

    fn interpolate_image(&self, image: &mut Image) {
        let (ys, xs) = self.get_resolution();
        for y in 1..ys - 1 {
            for x in 1..xs - 1 {
                if let PixelToInterpolate = image[x][y] {
                    image[x][y] = self.interpolate_pixel(x, y, image)
                }
            }
        }
    }

    fn progress_bar(&self, rx: Receiver<()>) {
        println!("starting render");
        let (columns, lines) = self.get_resolution();
        let portion_amount = columns / PORTIONS_SIZE * lines;
        for i in 1..=portion_amount {
            rx.recv().unwrap();
            println!("{} / {}", i, portion_amount);
        }
        println!("\ndone");
    }

    fn render_pixels_to_render(&self, image: &mut Image) {
        thread::scope(|scope| {
            let (tx, rx) = sync_channel(8);
            scope.spawn(move |_| self.progress_bar(rx));

            for (line_num, line) in image.iter_mut().enumerate() {
                let txc = tx.clone();
                scope.spawn(move |_| self.render_line(line_num, line, txc));
            }
        })
        .unwrap();
    }

    fn create_image_template(&self, func: SubsamplingFunc) -> Image {
        let (columns, lines) = self.get_resolution();

        let mut image = Vec::with_capacity(lines);
        let mut line_num = 0;

        image.resize_with(lines, || {
            let mut line = Vec::with_capacity(columns);
            let mut column_num = 0;

            line.resize_with(columns, || {
                let pixel = (column_num, line_num);
                column_num += 1;
                if self.is_edge(pixel) || func(pixel) {
                    Pixel::PixelToRender
                } else {
                    Pixel::PixelToInterpolate
                }
            });
            line_num += 1;
            line
        });
        image
    }

    fn render(&self, func: SubsamplingFunc) -> Image {
        let mut image = self.create_image_template(func);
        self.render_pixels_to_render(&mut image);
        self.interpolate_image(&mut image);
        self.render_pixels_to_render(&mut image);

        image
    }

    fn get_pixel_color(x: u32, y: u32, image: &Image) -> Color {
        match image[y as usize][x as usize].get_color() {
            Some(color) => color,
            None => panic!("Some pixels somehow didn't render"),
        }
    }

    pub fn render_and_save(&self, path: &str, func: SubsamplingFunc) {
        let (x, y) = self.resolution;

        let image = self.render(func);
        let mp = self.supersampling_multiplier as u32;

        ImageBuffer::from_fn(x as u32, y as u32, |x, y| {
            let mut colors = Vec::with_capacity((mp * mp) as usize);
            for xi in (x * mp)..((x + 1) * mp) {
                for yi in (y * mp)..((y + 1) * mp) {
                    colors.push(Self::get_pixel_color(xi, yi, &image));
                }
            }
            Color::colors_avg(colors).raw()
        })
        .save(path)
        .unwrap();
    }
}
