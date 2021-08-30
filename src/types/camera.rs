use super::{Matrix, Point, UP};

pub struct Camera {
    pub pos: Point,
    pub look_op: Matrix,
}
impl Camera {
    fn cos_sin(length: f64, angle: f64) -> (f64, f64) {
        let radians = angle.to_radians();
        (radians.cos() * length, radians.sin() * length)
    }

    pub fn from_dir(pos: Point, dir: Point) -> Self {
        let view_vec = dir.normalize();
        let side_vec = view_vec.cross(UP).normalize();
        let up_vec = side_vec.cross(view_vec);
        Camera {
            pos: pos,
            look_op: Matrix::from_vectors(side_vec, up_vec, -view_vec),
        }
    }

    #[allow(dead_code)]
    pub fn from_view_point(pos: Point, view_point: Point) -> Self {
        Self::from_dir(pos, pos >> view_point)
    }

    pub fn from_angles(pos: Point, angle_w: f64, angle_h: f64) -> Self {
        let (zx, y) = Self::cos_sin(1.0, angle_h);
        let (z, x) = Self::cos_sin(zx, angle_w);

        Self::from_dir(pos, Point { x: -x, y, z: -z })
    }
}
