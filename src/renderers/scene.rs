use iter_fixed::IntoIteratorFixed;

use crate::*;

pub type Coord = [usize; 2];

fn into_f64_array(coord: Coord) -> [f64; 2] {
    coord.into_iter_fixed().map(|x| x as f64).collect()
}

pub struct Scene {
    pub objs: SceneObjects,
    pub cam: Camera,
    pub fov: f64,
    pub resolution: Coord,
}

impl Scene {
    pub fn ray(&self, pixel: Coord) -> Ray {
        self.ray_with_resolution(pixel, self.resolution)
    }

    pub fn ray_with_resolution(&self, pixel: Coord, resolution: Coord) -> Ray {
        let [x, y] = into_f64_array(pixel);
        let [width, height] = into_f64_array(resolution);

        let [x, y] = [x - width / 2.0, height / 2.0 - y];
        let z = -height / (self.fov.to_radians() / 2.0).tan();

        let dir = self.cam.rotate_ray(Vector::new(x, y, z)).normalize();
        Ray::new(self.cam.pos, dir)
    }
}
