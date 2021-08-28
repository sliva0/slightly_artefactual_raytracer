use std::sync::{Arc, Weak};

use super::{Color, Point, Vector};

use super::object_types::{
    MarchingObject, MetaTracingObject, Object, TracingObject, TracingObjectType,
};

pub struct DummyObject();

impl Object for DummyObject {
    fn get_color(&self, _pos: Point) -> Color {
        Color::ERR_COLOR
    }
    fn get_normal(&self, _pos: Point, _eps: f64) -> Vector {
        Vector::new()
    }
}

pub struct MarchingRoom {
    pub size: f64,
    pub square_size: f64,
    pub colors: (Color, Color),
}

impl Object for MarchingRoom {
    fn get_color(&self, pos: Point) -> Color {
        let arr: [f64; 3] = pos.into();
        let sum: i32 = arr
            .iter()
            .map(|x| ((x + self.size) / self.square_size).floor() as i32)
            .sum();
        match sum % 2 {
            1 => self.colors.0,
            _ => self.colors.1,
        }
    }

    fn get_normal(&self, pos: Point, eps: f64) -> Vector {
        MarchingObject::get_normal(self, pos, eps)
    }
}

impl MarchingObject for MarchingRoom {
    fn check_sdf(&self, pos: Point) -> f64 {
        let arr: [f64; 3] = pos.into();
        self.size - arr.iter().fold(0f64, |a, b| a.max(b.abs()))
    }
}

type PointTuple = (Point, Point, Point);

fn pair_with<F: Fn(Point, Point) -> T, T>(p: PointTuple, f: F) -> (T, T, T) {
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

impl<'a, T: MetaTracingObject + 'a> ObjectPolygon<T> {
    fn collect_cuboid_face(
        obj: Weak<T>,
        shift: Vector,
        dir: Vector,
        sides: (Vector, Vector),
    ) -> Vec<Arc<dyn TracingObject + 'a>> {
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
    fn get_normal(&self, _pos: Point, _eps: f64) -> Vector {
        self.p.get_normal()
    }
}
impl<T: MetaTracingObject> TracingObject for ObjectPolygon<T> {
    fn find_intersection(&self, start: Point, dir: Vector) -> Option<f64> {
        self.p.find_intersection(start, dir)
    }
}

pub struct TracingRoom {
    pub size: f64,
    pub square_size: f64,
    pub colors: (Color, Color),
}

impl TracingRoom {}
impl MetaTracingObject for TracingRoom {
    fn get_color(&self, pos: Point) -> Color {
        let arr: [f64; 3] = pos.into();
        let sum: i32 = arr
            .iter()
            .map(|x| ((x + self.size) / self.square_size).floor() as i32)
            .sum();
        match sum % 2 {
            1 => self.colors.0,
            _ => self.colors.1,
        }
    }

    fn build_objects<'a>(self: Arc<Self>) -> Vec<TracingObjectType<'a>> {
        let mut objects = Vec::with_capacity(12);

        let p0 = Point::new();
        let pairs = pair_with(
            (
                Vector { x: 1.0, ..p0 },
                Vector { y: 1.0, ..p0 },
                Vector { z: 1.0, ..p0 },
            ),
            |p1, p2| (p1, p2),
        );
        for (i, j) in [pairs.0, pairs.1, pairs.2] {
            for (dir, side) in [(i, j), (-i, -j)] {
                let dir = dir * self.size;
                objects.extend(ObjectPolygon::collect_cuboid_face(
                    Arc::downgrade(&self),
                    p0,
                    dir,
                    (side * self.size, (dir ^ side)),
                ));
            }
        }
        objects
    }
}
