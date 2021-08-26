use std::process::Command;

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
            look_op: Matrix::from_points(side_vec, up_vec, -view_vec),
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

pub enum SdfCheckRes {
    Miss(f64),
    Hit(ObjectType),
}

struct Scene {
    objects: Vec<MarchingObjectType>,
}
impl Scene {
    fn check_sdf(&self, pos: Point) -> SdfCheckRes {
        let mut sdf = f64::INFINITY;

        for object in self.objects.iter() {
            sdf = sdf.min(object.check_sdf(pos));
            if sdf < EPSILON {
                return SdfCheckRes::Hit(object.clone().upcast());
            }
        }
        SdfCheckRes::Miss(sdf)
    }

    fn trace_ray(&self, start: Point, dir: Point) -> Color {
        let mut depth = 0.0;

        loop {
            let pos = start + (dir * depth);
            match self.check_sdf(pos) {
                SdfCheckRes::Hit(obj) => return obj.get_color(pos),
                SdfCheckRes::Miss(sdf) => depth += sdf,
            }
        }
    }
}

struct Renderer {
    scene: Scene,
    cam: Camera,
    fov: f64,
    resolution: (usize, usize),
}
impl Renderer {
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

    fn render_line(&self, line_num: usize) -> Vec<Color> {
        let columns = self.resolution.0;
        let mut line = Vec::with_capacity(columns);

        for i in 0..columns {
            let ray_dir = self.get_ray_dir((i, line_num));
            line.push(self.scene.trace_ray(self.cam.pos, ray_dir));
        }
        line
    }

    fn render(&self) -> Vec<Vec<Color>> {
        let lines = self.resolution.1;
        let mut image = Vec::with_capacity(lines);

        for i in 0..lines {
            image.push(self.render_line(i));
            println!("line: {} / {}", i + 1, lines);
        }
        image
    }

    fn render_and_save(&self, path: &str) {
        let image = self.render();
        let (x, y) = self.resolution;

        let img = ImageBuffer::from_fn(x as u32, y as u32, |x, y| {
            image[y as usize][x as usize].raw()
        });
        img.save(path).unwrap();
    }
}

fn main() {
    let renderer = Renderer {
        scene: Scene {
            objects: vec![Arc::new(objects::Room {
                size: 100.0,
                square_size: 20.0,
                colors: (Color::new(0, 0, 255), Color::new(255, 0, 0)),
            })],
        },
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
        resolution: (480, 270),
    };

    let path = "image.png";
    renderer.render_and_save(path);

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
            Command::new(opener)
                .arg(path)
                .output()
                .unwrap();
        }
        None => {}
    }
}
