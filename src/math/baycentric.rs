use cgmath::{Point2, Point3};

/// Return the point described by the triangle vertices and baycentric
/// coordinates.
pub fn into_point2(
    vertices: (Point2<f32>, Point2<f32>, Point2<f32>),
    baycentric: (f32, f32, f32),
) -> Point2<f32> {
    let b0v0 = baycentric.0 * vertices.0;
    let b1v1 = baycentric.1 * vertices.1;
    let b2v2 = baycentric.2 * vertices.2;
    let x = b0v0.x + b1v1.x + b2v2.x;
    let y = b0v0.y + b1v1.y + b2v2.y;
    Point2::new(x, y)
}

/// Return the point described by the triangle vertices and baycentric
/// coordinates.
pub fn into_point3(
    vertices: (Point3<f32>, Point3<f32>, Point3<f32>),
    baycentric: (f32, f32, f32),
) -> Point3<f32> {
    let b0v0 = baycentric.0 * vertices.0;
    let b1v1 = baycentric.1 * vertices.1;
    let b2v2 = baycentric.2 * vertices.2;
    let x = b0v0.x + b1v1.x + b2v2.x;
    let y = b0v0.y + b1v1.y + b2v2.y;
    let z = b0v0.z + b1v1.z + b2v2.z;
    Point3::new(x, y, z)
}

#[cfg(test)]
mod into_point2 {
    use super::into_point2;
    use crate::test::ApproxEq;
    use cgmath::Point2;

    const TRIANGLE: (Point2<f32>, Point2<f32>, Point2<f32>) = (
        Point2::new(-1.0, 0.0),
        Point2::new(0.0, 0.0),
        Point2::new(0.0, -1.0),
    );

    #[test]
    fn corners() {
        let top_left = into_point2(TRIANGLE, (1.0, 0.0, 0.0));
        top_left.assert_approx_eq(&Point2::new(-1.0, 0.0));

        let top_right = into_point2(TRIANGLE, (0.0, 1.0, 0.0));
        top_right.assert_approx_eq(&Point2::new(0.0, 0.0));

        let bottom_right = into_point2(TRIANGLE, (0.0, 0.0, 1.0));
        bottom_right.assert_approx_eq(&Point2::new(0.0, -1.0));
    }

    #[test]
    fn edges() {
        let top = into_point2(TRIANGLE, (0.5, 0.5, 0.0));
        top.assert_approx_eq(&Point2::new(-0.5, 0.0));

        let right = into_point2(TRIANGLE, (0.0, 0.5, 0.5));
        right.assert_approx_eq(&Point2::new(0.0, -0.5));

        let left = into_point2(TRIANGLE, (0.5, 0.0, 0.5));
        left.assert_approx_eq(&Point2::new(-0.5, -0.5));
    }
}

#[cfg(test)]
mod into_point3 {
    use super::into_point3;
    use crate::test::ApproxEq;
    use cgmath::Point3;

    const TRIANGLE: (Point3<f32>, Point3<f32>, Point3<f32>) = (
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
    );

    #[test]
    fn corners() {
        let x_corner = into_point3(TRIANGLE, (1.0, 0.0, 0.0));
        x_corner.assert_approx_eq(&Point3::new(1.0, 0.0, 0.0));

        let y_corner = into_point3(TRIANGLE, (0.0, 1.0, 0.0));
        y_corner.assert_approx_eq(&Point3::new(0.0, 1.0, 0.0));

        let z_corner = into_point3(TRIANGLE, (0.0, 0.0, 1.0));
        z_corner.assert_approx_eq(&Point3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn edges() {
        let top = into_point3(TRIANGLE, (0.5, 0.5, 0.0));
        top.assert_approx_eq(&Point3::new(0.5, 0.5, 0.0));

        let right = into_point3(TRIANGLE, (0.0, 0.5, 0.5));
        right.assert_approx_eq(&Point3::new(0.0, 0.5, 0.5));

        let left = into_point3(TRIANGLE, (0.5, 0.0, 0.5));
        left.assert_approx_eq(&Point3::new(0.5, 0.0, 0.5));
    }
}
