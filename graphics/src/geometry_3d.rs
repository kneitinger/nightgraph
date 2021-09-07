use crate::{MultiLine, Point};
use nalgebra::Point3 as nPoint3;
use nalgebra::{Matrix4, Vector3};
use num_traits::ToPrimitive;

pub struct ThreeSpace;

pub type Point3 = nPoint3<f64>;

/// Convenience function to allow making `Point`s quickly
/// from any compatible number type
#[inline(always)]
pub fn point3<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V) -> Point3 {
    Point3::new(
        x.to_f64().unwrap(),
        y.to_f64().unwrap(),
        z.to_f64().unwrap(),
    )
}

fn np2_to_point(p: Point3) -> Point {
    let xy = p.xy();
    let coords = xy.coords.as_slice();
    Point::new(coords[0], coords[1])
}

#[derive(Debug)]
pub struct Path3 {
    verticies: Vec<Point3>,
}

impl Path3 {
    pub fn new(verticies: Vec<Point3>) -> Path3 {
        Self { verticies }
    }

    pub fn new_closed(verticies: &[Point3]) -> Path3 {
        let mut closed_verts = Vec::new();
        closed_verts.extend_from_slice(verticies);
        closed_verts.push(verticies[0]);
        Self {
            verticies: closed_verts,
        }
    }

    pub fn projected(&self, observer: &Point3, target: &Point3) -> Path3 {
        let viewer = Matrix4::face_towards(observer, target, &Vector3::new(0., 1., 0.));
        Self {
            verticies: self
                .verticies
                .iter()
                .map(|p| viewer.transform_point(p))
                .collect(),
        }
    }

    pub fn flatten(&self) -> MultiLine {
        MultiLine::new(self.verticies.iter().map(|p| np2_to_point(*p)).collect())
    }
}
