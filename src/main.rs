use std::{process::Command, thread};

use image::ImageBuffer;

mod types;
use types::*;

const UP: Point = Point {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

struct Camera {
    pos: Point,
    look_op: Matrix,
}
impl Camera {
    fn cos_sin(length: f64, angle: f64) -> (f64, f64) {
        let radians = angle.to_radians();
        (radians.cos() * length, radians.sin() * length)
    }

    fn from_dir(pos: Point, dir: Point) -> Self {
        let view_vec = dir.normalize();
        let side_vec = view_vec.cross(UP).normalize();
        let up_vec = side_vec.cross(view_vec);
        Camera {
            pos: pos,
            look_op: Matrix::from_vectors(side_vec, up_vec, -view_vec),
        }
    }

    #[allow(dead_code)]
    fn from_view_point(pos: Point, view_point: Point) -> Self {
        Self::from_dir(pos, pos >> view_point)
    }

    fn from_angles(pos: Point, angle_w: f64, angle_h: f64) -> Self {
        let (zx, y) = Self::cos_sin(1.0, angle_h);
        let (z, x) = Self::cos_sin(zx, angle_w);

        Self::from_dir(pos, Point { x: -x, y, z: -z })
    }
}

pub enum SdfCheckRes<'a> {
    Miss(f64),
    Hit(ObjectType<'a>),
}

struct Scene<'a> {
    marching_objs: Vec<MarchingObjectType<'a>>,
    tracing_objs: Vec<TracingObjectType<'a>>,
    meta_objs: Vec<MetaTracingObjectType<'a>>,
}
impl<'a> Scene<'a> {
    fn build_meta_objects(&mut self) {
        for object in self.meta_objs.iter().map(Arc::clone) {
            self.tracing_objs.extend(object.build_objects());
        }
    }

    fn new(
        marching_objs: Vec<MarchingObjectType<'a>>,
        tracing_objs: Vec<TracingObjectType<'a>>,
        meta_objs: Vec<MetaTracingObjectType<'a>>,
    ) -> Self {
        let mut new_self = Self {
            marching_objs,
            tracing_objs,
            meta_objs,
        };
        new_self.build_meta_objects();
        new_self
    }

    fn check_sdf(&self, pos: Point) -> SdfCheckRes {
        let mut sdf = f64::INFINITY;

        for object in self.marching_objs.iter() {
            sdf = sdf.min(object.check_sdf(pos));
            if sdf < EPSILON {
                return SdfCheckRes::Hit(object.clone().upcast());
            }
        }
        SdfCheckRes::Miss(sdf)
    }

    fn march_ray(&self, start: Point, dir: Point, max_depth: f64) -> Option<(ObjectType, f64)> {
        let mut depth = 0.0;

        loop {
            let pos = start + (dir * depth);
            match self.check_sdf(pos) {
                SdfCheckRes::Hit(obj) => return Some((obj, depth)),
                SdfCheckRes::Miss(sdf) => depth += sdf,
            }
            if depth > max_depth || depth == f64::INFINITY {
                return None;
            }
        }
    }

    fn trace_ray(&self, start: Point, dir: Point) -> (ObjectType, Point) {
        let mut distance = f64::INFINITY;
        let mut object: ObjectType = Arc::new(objects::DummyObject());

        for obj in self.tracing_objs.iter() {
            match obj.find_intersection(start, dir) {
                Some(dist) if dist < distance => {
                    object = obj.clone().upcast();
                    distance = dist;
                }
                _ => (),
            };
        }
        match self.march_ray(start, dir, distance) {
            Some((obj, dist)) => {
                object = obj;
                distance = dist;
            }
            None => (),
        }

        (object, start + dir * distance)
    }

    fn compute_ray(&self, start: Point, dir: Point) -> Color {
        let (object, pos) = self.trace_ray(start, dir);
        object.get_color(pos)
    }
}

struct Renderer<'a> {
    scene: Scene<'a>,
    cam: Camera,
    fov: f64,
    resolution: (usize, usize),
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
        let (lines, columns) = self.resolution;

        for i in 0..columns {
            let ray_dir = self.get_ray_dir((i, line_num));
            line.push(self.scene.compute_ray(self.cam.pos, ray_dir));
        }
        println!("line: {} / {}", line_num + 1, lines);
    }

    fn render(self: Arc<Self>) -> Vec<Vec<Color>> {
        let lines = self.resolution.1;
        let mut image = Vec::with_capacity(lines);
        image.resize_with(lines, Vec::new);

        for (i, line) in image.iter_mut().enumerate() {
            let sref = self.clone();
            thread::spawn(move || sref.render_line(i, line))
                .join()
                .unwrap();
        }
        image
    }

    fn render_and_save(self: Arc<Self>, path: &str) {
        let image = self.render();
        let (x, y) = self.resolution;

        let img = ImageBuffer::from_fn(x as u32, y as u32, |x, y| {
            image[y as usize][x as usize].raw()
        });
        img.save(path).unwrap();
    }
}

fn open_image(path: &str) {
    match {
        if cfg!(windows) {
            Some("C:/Windows/explorer.exe")
        } else if cfg!(unix) {
            Some("xdg-open")
        } else {
            None
        }
    } {
        Some(opener) => {
            Command::new(opener).arg(path).spawn().unwrap();
        }
        None => (),
    }
}

fn main() {
    let renderer = Arc::new(Renderer {
        scene: Scene::new(
            vec![],
            vec![],
            vec![Arc::new(objects::TracingRoom {
                size: 100.0,
                square_size: 20.0,
                colors: (Color::new(0, 0, 255), Color::new(255, 0, 0)),
            })],
        ),
        cam: Camera::from_angles(
            Point {
                x: 0.0,
                y: 70.0,
                z: 0.0,
            },
            -30.0,
            0.0,
        ),
        fov: 60.0,
        resolution: (640, 360),
    });

    let path = "image.png";
    renderer.render_and_save(path);
    open_image(path);
}
