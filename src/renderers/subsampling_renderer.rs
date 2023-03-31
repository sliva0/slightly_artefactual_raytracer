use image::ImageBuffer;
use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar};
use iter_fixed::IntoIteratorFixed;
use rayon::prelude::*;

use super::{progress_bar, Coord, Scene};
use crate::*;

type SubsamplingFunc = Box<dyn Fn(Coord) -> bool>;
type Image = Vec<Vec<Pixel>>;

pub fn subsampling_func(subsample_number: i32) -> SubsamplingFunc {
    Box::new(match subsample_number {
        1 => |_| true,
        2 => |[x, y]| (x + y) % 2 == 0,
        3 => |[x, y]| (x + y) % 3 == 0,
        4 => |[x, y]| (x + y * 2) % 4 == 0,
        5 => |[x, y]| (x + y * 2) % 5 == 0,
        x => panic!("Unknown subsampling function for {x}"),
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
        match self {
            Pixel::Rendered(color) | Pixel::Interpolated(color) => Some(*color),
            _ => None,
        }
    }
}

pub struct SubsamplingRenderer {
    pub scene: Scene,
    pub subsampling_limit: f64,
    pub supersampling_multiplier: usize,
}

impl SubsamplingRenderer {
    fn resolution(&self) -> [usize; 2] {
        self.scene
            .resolution
            .into_iter_fixed()
            .map(|x| x * self.supersampling_multiplier)
            .collect()
    }

    fn is_edge(&self, pixel: Coord) -> bool {
        let [x, y] = pixel;
        let [width, height] = self.resolution();
        x == 0 || y == 0 || x == width - 1 || y == height - 1
    }

    #[allow(clippy::needless_range_loop)]
    fn collect_neighbors(image: &Image, x: usize, y: usize) -> Vec<Color> {
        let mut colors = vec![];
        for xi in (x - 1)..=(x + 1) {
            for yi in (y - 1)..=(y + 1) {
                if let Pixel::Rendered(color) = image[yi][xi] {
                    colors.push(color);
                }
            }
        }
        colors
    }

    fn interpolate_pixel(&self, image: &mut Image, x: usize, y: usize) -> Pixel {
        let colors = Self::collect_neighbors(image, x, y);

        if Color::colors_diff(&colors) > self.subsampling_limit {
            Pixel::ToRender
        } else {
            Pixel::Interpolated(Color::colors_avg(colors))
        }
    }

    fn interpolate_image(&self, image: &mut Image, progress_bar: ProgressBar) {
        let [width, height] = self.resolution();

        for xi in 0..width {
            for yi in 0..height {
                if let Pixel::ToInterpolate = image[yi][xi] {
                    image[yi][xi] = self.interpolate_pixel(image, xi, yi);
                }
                progress_bar.inc(1);
            }
        }
    }

    fn render_pixels_to_render(&self, image: &mut Image, progress_bar: ProgressBar) {
        image
            .par_iter_mut()
            .enumerate()
            .flat_map(|(yi, line)| {
                line.par_iter_mut()
                    .enumerate()
                    .map(move |(xi, pixel)| ([xi, yi], pixel))
            })
            .progress_with(progress_bar)
            .for_each(|(coord, pixel)| {
                if let Pixel::ToRender = pixel {
                    let ray = self.scene.ray_with_resolution(coord, self.resolution());
                    *pixel = Pixel::Rendered(self.scene.objs.trace_ray(ray));
                }
            });
    }

    fn create_image_template(&self, subsampling_func: SubsamplingFunc) -> Image {
        let [width, height] = self.resolution();

        (0..height)
            .map(|yi| {
                (0..width)
                    .map(|xi| {
                        let pixel = [xi, yi];
                        if self.is_edge(pixel) || subsampling_func(pixel) {
                            Pixel::ToRender
                        } else {
                            Pixel::ToInterpolate
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn render_raw(&self, func: SubsamplingFunc) -> Image {
        let mut image = self.create_image_template(func);
        let [width, height] = self.resolution();
        let pixel_count = width * height;

        let mpb = MultiProgress::new();

        let pb1 = mpb.add(progress_bar(pixel_count, "First pass"));
        let pbi = mpb.add(progress_bar(pixel_count, "Interpolating"));
        let pb2 = mpb.add(progress_bar(pixel_count, "Second pass"));

        self.render_pixels_to_render(&mut image, pb1);
        self.interpolate_image(&mut image, pbi);
        self.render_pixels_to_render(&mut image, pb2);

        image
    }

    fn pixel_color(image: &Image, x: u32, y: u32) -> Color {
        image[y as usize][x as usize]
            .color()
            .expect("Some pixels somehow didn't render")
    }

    pub fn render(&self, func: SubsamplingFunc) -> ImageBuffer<RawColor, Vec<u8>> {
        let [width, height] = self.scene.resolution;

        let image = self.render_raw(func);
        let mp = self.supersampling_multiplier as u32;

        ImageBuffer::from_fn(width as u32, height as u32, |xi, yi| {
            let mut colors = Vec::with_capacity((mp * mp) as usize);
            for xi in (xi * mp)..((xi + 1) * mp) {
                for yi in (yi * mp)..((yi + 1) * mp) {
                    colors.push(Self::pixel_color(&image, xi, yi));
                }
            }
            Color::colors_avg(colors).into_raw()
        })
    }
}
