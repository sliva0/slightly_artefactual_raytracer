use std::{fmt::Debug, sync::Arc};

use iter_fixed::IntoIteratorFixed;

use super::*;

fn map_pairs<T: Copy, O, F: Fn(T, T) -> O>(p: [T; 3], f: F) -> [O; 3] {
    p.into_iter_fixed()
        .zip([p[1], p[2], p[0]])
        .map(|(a, b)| f(a, b))
        .collect()
}

pub fn basis_pairs() -> Vec<(Vector, Vector)> {
    [
        map_pairs(BASIS, |a, b| (a, b)),
        map_pairs(BASIS, |a, b| (-a, -b)),
    ]
    .concat()
}

#[derive(Debug)]
pub struct Plane {
    ///plane normal
    normal: Vector,
    ///d from plane equation (ax + by + cz + d = 0), plane shift
    shift: f64,
}
impl Plane {
    fn new(v: [Point; 3]) -> Self {
        let normal = ((v[0] >> v[1]) ^ (v[0] >> v[2])).normalize();
        Self {
            normal,
            shift: -normal * v[0],
        }
    }

    fn find_intersection(&self, ray: Ray) -> Option<f64> {
        let m = ray.dir * self.normal;
        if m.is_subnormal() {
            return None;
        }
        let dist = -(ray.start * self.normal + self.shift) / m;

        if dist > 0.0 {
            Some(dist)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Polygon {
    vertices: [Point; 3],
    ///edge vectors
    edges: [Vector; 3],
    plane: Plane,
}
impl Polygon {
    fn new(vertices: [Point; 3]) -> Self {
        Self {
            vertices,
            edges: map_pairs(vertices, |v1, v2| v1 >> v2),
            plane: Plane::new(vertices),
        }
    }

    fn find_intersection(&self, ray: Ray) -> Option<f64> {
        let dist = self.plane.find_intersection(ray)?;
        let pos = ray.point(dist);

        let dot_muls = map_pairs(
            self.edges
                .into_iter_fixed()
                .zip(self.vertices)
                .map(|(ei, vi)| ei ^ (vi >> pos))
                .collect(),
            |v1, v2| v1 * v2,
        );

        if dot_muls.into_iter().all(|x| x >= 0.0) {
            Some(dist)
        } else {
            None
        }
    }

    fn normal(&self) -> Vector {
        self.plane.normal
    }
}

#[derive(Debug)]
pub struct ObjectPolygon<T: ReferenceObject> {
    p: Polygon,
    obj: Arc<T>,
}

impl<T: ReferenceObject> ObjectPolygon<T> {
    fn new(vertices: [Point; 3], obj: &Arc<T>) -> Arc<Self> {
        Arc::new(Self {
            p: Polygon::new(vertices),
            obj: obj.clone(),
        })
    }
}

impl<T: ReferenceObject + 'static> ObjectPolygon<T> {
    pub fn collect_cuboid_face(
        obj: &Arc<T>,
        shift: Vector,
        dir: Vector,
        sides: (Vector, Vector),
    ) -> Vec<TracingObjectType> {
        let center = shift + dir;
        let eps1: f64 = 1.0 + EPSILON; // fill spaces between sides
        let sides = (sides.0 * eps1, sides.1 * eps1);
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

impl<T: ReferenceObject> Object for ObjectPolygon<T> {
    fn color(&self, pos: Point) -> Color {
        self.obj.color(pos)
    }

    fn normal(&self, _pos: Point) -> Vector {
        self.p.normal()
    }

    fn material(&self) -> Material {
        self.obj.material()
    }
}

impl<T: ReferenceObject> TracingObject for ObjectPolygon<T> {
    fn find_intersection(&self, ray: Ray) -> Option<f64> {
        self.p.find_intersection(ray)
    }
}
