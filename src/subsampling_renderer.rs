use image::ImageBuffer;
use indicatif::{
    MultiProgress, ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle,
};
use rayon::prelude::*;

use super::*;

type SubsamplingFunc = Box<dyn Fn(Coord) -> bool>;
type Image = Vec<Vec<Pixel>>;

pub fn subsampling_func(subsample_number: i32) -> SubsamplingFunc {
    Box::new(match subsample_number {
        1 => |_| true,
        2 => |[x, y]| (x + y) % 2 == 0,
        3 => |[x, y]| (x + y) % 3 == 0,
        4 => |[x, y]| (x + y * 2) % 4 == 0,
        5 => |[x, y]| (x + y * 2) % 5 == 0,
        _ => panic!("incorrect subsample number"),
    })
}

enum Pixel {
    ToRender,
    ToInterpolate,
    Rendered(Color),
    Interpolated(Color),
}

impl Pixel {
    fn color(&self) -> Option<Color> {
        if let Rendered(col) | Interpolated(col) = self {
            Some(*col)
        } else {
            None
        }
    }
}

use Pixel::*;

pub struct SubsamplingRenderer {
    pub scene: Scene,
    pub cam: Camera,
    pub fov: f64,
    pub resolution: (usize, usize),
    pub subsampling_limit: f64,
    pub supersampling_multiplier: usize,
}

impl SubsamplingRenderer {
    fn resolution(&self) -> (usize, usize) {
        (
            (self.resolution.0 * self.supersampling_multiplier),
            (self.resolution.1 * self.supersampling_multiplier),
        )
    }

    fn f64_resolution(&self) -> (f64, f64) {
        let (x, y) = self.resolution();
        (x as f64, y as f64)
    }

    fn is_edge(&self, pixel: Coord) -> bool {
        let [x, y] = pixel;
        let (xs, ys) = self.resolution();
        x == 0 || y == 0 || x == xs - 1 || y == ys - 1
    }

    fn create_ray(&self, pixel: Coord) -> Ray {
        let [x, y] = pixel;
        let [x, y] = [x as f64, y as f64];
        let (xs, ys) = self.f64_resolution();

        let (x, y) = ((x - xs / 2.0), -(y - ys / 2.0));
        let z = -ys / (self.fov.to_radians() / 2.0).tan();

        let dir = self.cam.rotate_ray(Vector::new(x, y, z)).normalize();
        Ray::new(self.cam.pos, dir)
    }

    #[allow(clippy::needless_range_loop)]
    fn collect_neighbors(x: usize, y: usize, image: &Image) -> Vec<Color> {
        let mut colors = vec![];
        for xi in (x - 1)..=(x + 1) {
            for yi in (y - 1)..=(y + 1) {
                if let Rendered(color) = image[xi][yi] {
                    colors.push(color);
                }
            }
        }
        colors
    }

    fn interpolate_pixel(&self, x: usize, y: usize, image: &mut Image) -> Pixel {
        let colors = Self::collect_neighbors(x, y, image);

        if Color::colors_diff(&colors) > self.subsampling_limit {
            ToRender
        } else {
            Interpolated(Color::colors_avg(colors))
        }
    }

    fn interpolate_image(&self, image: &mut Image, progress_bar: ProgressBar) {
        let (ys, xs) = self.resolution();
        for y in 1..ys - 1 {
            for x in 1..xs - 1 {
                if let ToInterpolate = image[x][y] {
                    image[x][y] = self.interpolate_pixel(x, y, image);
                    progress_bar.inc(1);
                }
            }
        }
    }

    fn render_pixels_to_render(&self, image: &mut Image, progress_bar: ProgressBar) {
        image
            .par_iter_mut()
            .enumerate()
            .flat_map(|(line_num, line)| {
                line.par_iter_mut()
                    .enumerate()
                    .map(move |(column_num, pixel)| ([column_num, line_num], pixel))
            })
            .progress_with(progress_bar)
            .for_each(|(coord, pixel)| {
                if let ToRender = pixel {
                    let ray = self.create_ray(coord);
                    *pixel = Rendered(self.scene.trace_ray(ray));
                }
            });
    }

    fn create_image_template(&self, func: SubsamplingFunc) -> Image {
        let (columns, lines) = self.resolution();

        let mut image = Vec::with_capacity(lines);
        let mut line_num = 0;

        image.resize_with(lines, || {
            let mut line = Vec::with_capacity(columns);
            let mut column_num = 0;

            line.resize_with(columns, || {
                let pixel = [column_num, line_num];
                column_num += 1;
                if self.is_edge(pixel) || func(pixel) {
                    Pixel::ToRender
                } else {
                    Pixel::ToInterpolate
                }
            });
            line_num += 1;
            line
        });
        image
    }

    fn render(&self, func: SubsamplingFunc) -> Image {
        let mut image = self.create_image_template(func);
        let (image_width, image_height) = self.resolution();
        let pixel_count = (image_width * image_height) as u64;
        let style = ProgressStyle::with_template(
            "{msg:14} {elapsed:>3} {wide_bar} {pos}/{len} ETA {eta:>3}",
        )
        .unwrap();

        let mpb = MultiProgress::new();

        let pb1 = mpb.add(
            ProgressBar::new(pixel_count)
                .with_style(style.clone())
                .with_finish(ProgressFinish::AndLeave)
                .with_message("First pass"),
        );

        let pbi = mpb.add(
            ProgressBar::new(((image_width - 2) * (image_height - 2)) as u64)
                .with_style(style.clone())
                .with_finish(ProgressFinish::AndLeave)
                .with_message("Interpolating"),
        );

        let pb2 = mpb.add(
            ProgressBar::new(pixel_count)
                .with_style(style)
                .with_finish(ProgressFinish::AndLeave)
                .with_message("Second pass"),
        );

        self.render_pixels_to_render(&mut image, pb1);
        self.interpolate_image(&mut image, pbi);
        self.render_pixels_to_render(&mut image, pb2);

        image
    }

    fn pixel_color(x: u32, y: u32, image: &Image) -> Color {
        image[y as usize][x as usize]
            .color()
            .expect("Some pixels somehow didn't render")
    }

    pub fn render_and_save(&self, path: &str, func: SubsamplingFunc) {
        let (x, y) = self.resolution;

        let image = self.render(func);
        let mp = self.supersampling_multiplier as u32;

        ImageBuffer::from_fn(x as u32, y as u32, |x, y| {
            let mut colors = Vec::with_capacity((mp * mp) as usize);
            for xi in (x * mp)..((x + 1) * mp) {
                for yi in (y * mp)..((y + 1) * mp) {
                    colors.push(Self::pixel_color(xi, yi, &image));
                }
            }
            Color::colors_avg(colors).raw()
        })
        .save(path)
        .unwrap();
    }
}
