use std::{
    fmt::Debug,
    sync::Arc,
};

use super::*;

fn get_vec_pairs<T: Copy, O, F: Fn(T, T) -> O>(p: Vec<T>, f: F) -> Vec<O> {
    p.iter()
        .enumerate()
        .map(|(i, &v)| f(v, p[(i + 1) % p.len()]))
        .collect()
}

pub fn get_pairs<T, const N: usize, O, F>(p: [T; N], f: F) -> [O; N]
where
    T: Copy,
    O: Debug,
    F: Fn(T, T) -> O,
{
    get_vec_pairs(p.to_vec(), f).try_into().unwrap()
}

pub fn get_basis_pairs() -> Vec<(Vector, Vector)> {
    let mut v = get_vec_pairs(BASIS.to_vec(), |x, y| (x, y));
    v.extend(get_vec_pairs(BASIS.to_vec(), |x, y| (-x, -y)));
    v
}

#[derive(Debug)]
pub struct Plane {
    ///plane normal
    n: Vector,
    ///d from plane equation (ax + by + cz + d = 0), plane shift
    d: f64,
}
impl Plane {
    fn new(v: [Point; 3]) -> Self {
        let n = ((v[0] >> v[1]) ^ (v[0] >> v[2])).normalize();
        Self { n, d: -n * v[0] }
    }

    fn find_intersection(&self, ray: Ray) -> Option<f64> {
        let m = ray.dir * self.n;
        if m.is_subnormal() {
            return None
        }
        let dist =  -(ray.start * self.n + self.d) / m;

        if dist > 0.0 {
            Some(dist)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Polygon {
    ///vertices
    v: [Point; 3],
    ///edge vectors
    e: [Point; 3],
    plane: Plane,
}
impl Polygon {
    fn new(v: [Point; 3]) -> Self {
        Self {
            v,
            e: get_pairs(v, |v1, v2| v1 >> v2),
            plane: Plane::new(v),
        }
    }

    fn find_intersection(&self, ray: Ray) -> Option<f64> {
        let dist = self.plane.find_intersection(ray)?;
        let pos = ray.get_point(dist);

        let m = get_vec_pairs(
            self.e
                .into_iter()
                .zip(self.v)
                .map(|(ei, vi)| ei ^ (vi >> pos))
                .collect(),
            |v1, v2| v1 * v2,
        );
        if m.into_iter().all(|x| x >= 0.0) {
            Some(dist)
        } else {
            None
        }
    }

    fn get_normal(&self) -> Vector {
        self.plane.n
    }
}

#[derive(Debug)]
pub struct ObjectPolygon<T: MetaTracingObject> {
    p: Polygon,
    obj: Arc<T>,
}
impl<T: MetaTracingObject> ObjectPolygon<T> {
    fn new(v: [Point; 3], obj: &Arc<T>) -> Arc<Self> {
        Arc::new(Self {
            p: Polygon::new(v),
            obj: obj.clone(),
        })
    }
}
impl<T: MetaTracingObject + 'static> ObjectPolygon<T> {
    pub fn collect_cuboid_face(
        obj: &Arc<T>,
        shift: Vector,
        dir: Vector,
        sides: (Vector, Vector),
    ) -> Vec<TracingObjectType> {
        let center = shift + dir;
        // I do not remember why these lines are needed, if I remember, I will return it
        // let eps1: f64 = 1.0 + EPSILON;
        // let sides = (sides.0 * eps1, sides.1 * eps1);
        let c = (
            center + sides.0 + sides.1,
            center - sides.0 + sides.1,
            center - sides.0 - sides.1,
            center + sides.0 - sides.1,
        );
        vec![
            Self::new([c.0, c.1, c.2], obj),
            Self::new([c.2, c.3, c.0], obj),
        ]
    }
}

impl<T: MetaTracingObject> Object for ObjectPolygon<T> {
    fn get_color(&self, pos: Point) -> Color {
        self.obj.get_color(pos)
    }

    fn get_normal(&self, _pos: Point) -> Vector {
        self.p.get_normal()
    }

    fn get_material(&self) -> Material {
        self.obj.get_material()
    }
}

impl<T: MetaTracingObject> TracingObject for ObjectPolygon<T> {
    fn find_intersection(&self, ray: Ray) -> Option<f64> {
        self.p.find_intersection(ray)
    }
}
