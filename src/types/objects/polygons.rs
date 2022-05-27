use std::fmt::Debug;

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

    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        let m = dir * self.n;
        if m.is_subnormal() {
            return None;
        }
        let dist = -(start * self.n + self.d) / m;

        if dist > 0.0 {
            Some(dist)
        } else {
            None
        }
    }
}

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

    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        let dist = self.plane.find_intersection(start, dir)?;
        let pos = start + dir * dist;

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

pub struct ObjectPolygon<'a, T: MetaTracingObject> {
    p: Polygon,
    obj: &'a T,
}
impl<'a, T: MetaTracingObject> ObjectPolygon<'a, T> {
    fn new(v: [Point; 3], obj: &'a T) -> Box<Self> {
        Box::new(Self {
            p: Polygon::new(v),
            obj,
        })
    }
}
impl<'a, T: MetaTracingObject + 'a> ObjectPolygon<'a, T> {
    pub fn collect_cuboid_face(
        obj: &'a T,
        shift: Vector,
        dir: Vector,
        sides: (Vector, Vector),
    ) -> Vec<TracingObjectType<'a>> {
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
impl<T: MetaTracingObject> Object for ObjectPolygon<'_, T> {
    fn get_color(&self, pos: Point) -> Color {
        self.obj.get_color(pos)
    }

    fn get_normal(&self, _pos: Point) -> Vector {
        self.p.get_normal()
    }

    fn get_material(&self, pos: Point) -> Material {
        self.obj.get_material(pos)
    }

    fn is_shematic(&self) -> bool {
        false
    }
}
impl<T: MetaTracingObject> TracingObject for ObjectPolygon<'_, T> {
    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        self.p.find_intersection(start, dir)
    }
}
