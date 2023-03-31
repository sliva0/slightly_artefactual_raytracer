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
        let [xs, ys] = self.resolution();
        x == 0 || y == 0 || x == xs - 1 || y == ys - 1
    }

    #[allow(clippy::needless_range_loop)]
    fn collect_neighbors(x: usize, y: usize, image: &Image) -> Vec<Color> {
        let mut colors = vec![];
        for xi in (x - 1)..=(x + 1) {
            for yi in (y - 1)..=(y + 1) {
                if let Pixel::Rendered(color) = image[xi][yi] {
                    colors.push(color);
                }
            }
        }
        colors
    }

    fn interpolate_pixel(&self, x: usize, y: usize, image: &mut Image) -> Pixel {
        let colors = Self::collect_neighbors(x, y, image);

        if Color::colors_diff(&colors) > self.subsampling_limit {
            Pixel::ToRender
        } else {
            Pixel::Interpolated(Color::colors_avg(colors))
        }
    }

    fn interpolate_image(&self, image: &mut Image, progress_bar: ProgressBar) {
        let [ys, xs] = self.resolution();
        for y in 1..ys - 1 {
            for x in 1..xs - 1 {
                if let Pixel::ToInterpolate = image[x][y] {
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
                if let Pixel::ToRender = pixel {
                    let ray = self.scene.ray_with_resolution(coord, self.resolution());
                    *pixel = Pixel::Rendered(self.scene.objs.trace_ray(ray));
                }
            });
    }

    fn create_image_template(&self, subsampling_func: SubsamplingFunc) -> Image {
        let [columns, lines] = self.resolution();

        (0..lines)
            .map(|line_num| {
                (0..columns)
                    .map(|column_num| {
                        let pixel = [column_num, line_num];
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
        let [image_width, image_height] = self.resolution();

        let mpb = MultiProgress::new();

        let pb1 = mpb.add(progress_bar(image_width * image_height, "First pass"));

        let pbi = mpb.add(progress_bar(
            image_width.saturating_sub(2) * image_height.saturating_sub(2),
            "Interpolating",
        ));

        let pb2 = mpb.add(progress_bar(image_width * image_height, "Second pass"));

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
        let [x, y] = self.scene.resolution;

        let image = self.render_raw(func);
        let mp = self.supersampling_multiplier as u32;

        ImageBuffer::from_fn(x as u32, y as u32, |x, y| {
            let mut colors = Vec::with_capacity((mp * mp) as usize);
            for xi in (x * mp)..((x + 1) * mp) {
                for yi in (y * mp)..((y + 1) * mp) {
                    colors.push(Self::pixel_color(&image, xi, yi));
                }
            }
            Color::colors_avg(colors).into_raw()
        })
    }
}
