use image::ImageBuffer;

mod types;
use types::*;

const UP: Point = Point {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
const EPSILON: f64 = 0.0001;

#[derive(Debug)]
struct Camera {
    pos: Point,
    fov: f64,
    resolution: (i32, i32),
    view_point: Point,
}
impl Camera {
    fn look_at(&self) -> Matrix {
        let view_vec = (self.view_point - self.pos).normalize();
        let side_vec = view_vec.cross(UP).normalize();
        let up_vec = side_vec.cross(view_vec);
        Matrix::from_points(side_vec, up_vec, -view_vec)
    }

    fn get_ray_dir(&self, pixel: Coord) -> Point {
        let (xs, ys) = self.resolution;

        let (x, y) = ((pixel.0 - xs / 2) as f64, (pixel.1 - ys / 2) as f64);
        let z = -ys as f64 / (self.fov.to_radians() / 2.0).tan();

        (self.look_at() * Point { x, y, z }).normalize()
    }
}

enum CheckRes<'a> {
    Miss(f64),
    Hit(&'a SceneObjectType),
}

struct Scene<'a> {
    cam: Camera,
    objects: Vec<&'a SceneObjectType>,
}
impl<'a> Scene<'a> {
    fn check_sdf(&self, pos: Point) -> CheckRes {
        let mut sdf = f64::INFINITY;

        for object in self.objects.iter() {
            sdf = sdf.min(object.check_sdf(pos));
            if sdf < EPSILON {
                return CheckRes::Hit(*object);
            }
        }
        CheckRes::Miss(sdf)
    }

    fn trace_ray(&self, pixel: Coord) -> Color {
        let mut depth = 0.0;
        let dir = self.cam.get_ray_dir(pixel);

        loop {
            let pos = self.cam.pos + (dir * depth);
            match self.check_sdf(pos) {
                CheckRes::Hit(obj) => return obj.get_color(pos),
                CheckRes::Miss(sdf) => depth += sdf,
            }
        }
    }

    fn render_line(&self, line_num: i32) -> Vec<Color> {
        let mut line = Vec::new();

        for column_num in 0..self.cam.resolution.0 {
            line.push(self.trace_ray((column_num, line_num)));
        }
        line
    }

    fn render_picture(&self) -> Vec<Vec<Color>> {
        let mut picture = Vec::new();

        for line_num in 0..self.cam.resolution.1 {
            println!("({}/{})", line_num, self.cam.resolution.1);
            picture.push(self.render_line(line_num));
        }
        picture
    }
}

fn main() {
    let scene = Scene {
        cam: Camera {
            pos: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            fov: 60.0,
            resolution: (480, 270),
            view_point: Point {
                x: 0.0,
                y: 0.0,
                z: 100.0,
            },
        },
        objects: vec![&objects::Room{
            size: 100.0,
            square_size: 20.0,
            colors: (Rgb([0, 0, 255]), Rgb([255, 0, 0])),
        }],
    };

    let pict = scene.render_picture();
    let (x, y) = scene.cam.resolution;

    let img = ImageBuffer::from_fn(x as u32, y as u32, |x, y| pict[y as usize][x as usize]);
    img.save("image.png").unwrap();
}
