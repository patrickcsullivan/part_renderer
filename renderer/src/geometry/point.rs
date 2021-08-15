use crate::geometry::axis::Axis3;
use cgmath::{BaseNum, InnerSpace, Point2, Point3};

/// Returns the point's component on the given axis.
pub fn component(p: Point3<f32>, axis: Axis3) -> f32 {
    match axis {
        Axis3::X => p.x,
        Axis3::Y => p.y,
        Axis3::Z => p.z,
    }
}

/// Returns a new vector whose components are taken from the components of the
/// given vector.
pub fn permute(p: Point3<f32>, new_x: Axis3, new_y: Axis3, new_z: Axis3) -> Point3<f32> {
    Point3::new(
        component(p, new_x),
        component(p, new_y),
        component(p, new_z),
    )
}

pub fn add_point2(points: Vec<Point2<f32>>) -> Point2<f32> {
    points.iter().fold(Point2::new(0.0, 0.0), |result, next| {
        Point2::new(result.x + next.x, result.y + next.y)
    })
}

pub fn add_point3(points: Vec<Point3<f32>>) -> Point3<f32> {
    points
        .iter()
        .fold(Point3::new(0.0, 0.0, 0.0), |result, next| {
            Point3::new(result.x + next.x, result.y + next.y, result.z + next.z)
        })
}

// pub fn add_point3(p1: Point3<f32>, p2: Point3<f32>) -> Point3<f32> {
//     Point3::new(p1.x + p2.x, p1.y + p2.y, p1.z + p2.z)
// }
