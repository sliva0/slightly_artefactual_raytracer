use std::sync::{Arc, Weak};

use super::*;

type PointTuple = (Point, Point, Point);

pub fn pair_with<F: Fn(Point, Point) -> T, T>(p: PointTuple, f: F) -> (T, T, T) {
    (f(p.0, p.1), f(p.1, p.2), f(p.2, p.0))
}

pub struct Plane {
    ///plane normal
    n: Vector,
    ///d from plane equation (ax + by + cz + d = 0), plane shift
    d: f64,
}
impl Plane {
    fn new(v: PointTuple) -> Self {
        let n = ((v.0 >> v.1) ^ (v.0 >> v.2)).normalize();
        Self { n, d: -n * v.0 }
    }

    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        #[allow(illegal_floating_point_literal_pattern)]
        let dist = match dir * self.n {
            0.0 => return None,
            d => -(start * self.n + self.d) / d,
        };

        if dist <= 0.0 {
            None
        } else {
            Some(dist)
        }
    }
}

struct Polygon {
    ///vertices tuple
    v: PointTuple,
    ///edge vectors tuple
    e: PointTuple,
    plane: Plane,
}
impl Polygon {
    fn new(v: PointTuple) -> Self {
        Self {
            v,
            e: pair_with(v, |v1, v2| v1 >> v2),
            plane: Plane::new(v),
        }
    }

    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        let dist = self.plane.find_intersection(start, dir)?;
        let pos = start + dir * dist;

        let m = pair_with(
            (
                self.e.0 ^ (self.v.0 >> pos),
                self.e.1 ^ (self.v.1 >> pos),
                self.e.2 ^ (self.v.2 >> pos),
            ),
            |v1, v2| v1 * v2,
        );
        if m.0.min(m.1).min(m.2) >= 0.0 {
            Some(dist)
        } else {
            None
        }
    }

    fn get_normal(&self) -> Vector {
        self.plane.n
    }
}

pub struct ObjectPolygon<T: MetaTracingObject> {
    p: Polygon,
    obj: Weak<T>,
}
impl<'a, T: MetaTracingObject + 'a + Sync + Send> ObjectPolygon<T> {
    pub fn collect_cuboid_face(
        obj: Weak<T>,
        shift: Vector,
        dir: Vector,
        sides: (Vector, Vector),
    ) -> Vec<TracingObjectType<'a>> {
        let center = shift + dir;
        let c = (
            center + sides.0 + sides.1,
            center + sides.0 - sides.1,
            center - sides.0 - sides.1,
            center - sides.0 + sides.1,
        );
        vec![
            Arc::new(Self {
                p: Polygon::new((c.0, c.1, c.2)),
                obj: obj.clone(),
            }),
            Arc::new(Self {
                p: Polygon::new((c.2, c.3, c.0)),
                obj: obj.clone(),
            }),
        ]
    }
}
impl<T: MetaTracingObject> Object for ObjectPolygon<T> {
    fn get_color(&self, pos: Point) -> Color {
        match self.obj.upgrade() {
            Some(metaobj) => metaobj.get_color(pos),
            None => Color::ERR_COLOR,
        }
    }

    fn get_normal(&self, _pos: Point) -> Vector {
        self.p.get_normal()
    }

    fn get_material(&self, pos: Point) -> Material {
        match self.obj.upgrade() {
            Some(metaobj) => metaobj.get_material(pos),
            None => Material::ERR_MATERIAL,
        }
    }

    fn is_shematic(&self) -> bool {
        self.obj.strong_count() == 0
    }
}
impl<T: MetaTracingObject> TracingObject for ObjectPolygon<T> {
    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        self.p.find_intersection(start, dir)
    }
}