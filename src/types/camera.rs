use super::{Matrix, Point, Vector};

const _UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub struct Camera {
    pub pos: Point,
    operator: Matrix,
}
impl Camera {
    pub fn rotate_ray(&self, ray: Vector) -> Vector {
        self.operator * ray
    }

    fn _cos_sin(length: f64, angle: f64) -> (f64, f64) {
        let angle = angle.to_radians();
        (angle.cos() * length, angle.sin() * length)
    }

    /// This function is not working as it should and I don't know why.
    /// TODO: fix
    pub fn _from_dir(pos: Point, dir: Vector) -> Self {
        let view_vec = dir.normalize();
        let side_vec = (view_vec ^ _UP).normalize();
        let up_vec = side_vec ^ view_vec;
        Camera {
            pos,
            operator: Matrix::new(side_vec, up_vec, -view_vec),
        }
    }

    pub fn _from_view_point(pos: Point, view_point: Point) -> Self {
        Self::_from_dir(pos, pos >> view_point)
    }

    pub fn _from_angles(pos: Point, angle_w: f64, angle_h: f64) -> Self {
        let (zx, y) = Self::_cos_sin(1.0, -angle_h);
        let (z, x) = Self::_cos_sin(zx, angle_w);
        Self::_from_dir(pos, -Vector::new(x, y, z))
    }

    pub fn from_angles(pos: Point, angle_w: f64, angle_h: f64) -> Self {
        Self {
            pos,
            operator: Matrix::new_y_rotation(angle_w) * Matrix::new_x_rotation(angle_h),
        }
    }
}
