use crate::math::axis::Axis3;
use cgmath::{BaseNum, InnerSpace, Vector3};

/// Returns the outgoing vector when the incoming vector is reflected across the
/// surface normal.
pub fn reflect(incoming: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    incoming - normal * 2.0 * incoming.dot(normal)
}

/// If the angle between `v1` and `v2` is less than 90 degrees then return `v1`.
/// Otherwise flip and return `v1` so that it is in the same hemisphere as `v2`.
pub fn face_forward<S: BaseNum>(v1: Vector3<S>, v2: Vector3<S>) -> Vector3<S> {
    if v1.dot(v2) < S::zero() {
        v1 * (S::zero() - S::one())
    } else {
        v1
    }
}

/// Returns the axis along which the vector has the greatest magnitude.
pub fn max_dimension(v: Vector3<f32>) -> Axis3 {
    let x = v.x.abs();
    let y = v.y.abs();
    let z = v.z.abs();
    if z > x && z > y {
        Axis3::Z
    } else if y > x {
        Axis3::Y
    } else {
        Axis3::X
    }
}

/// Returns the vector's component on the given axis.
pub fn component(v: Vector3<f32>, axis: Axis3) -> f32 {
    match axis {
        Axis3::X => v.x,
        Axis3::Y => v.y,
        Axis3::Z => v.z,
    }
}

/// Returns a new vector whose components are taken from the components of the
/// given vector.
pub fn permute(v: Vector3<f32>, new_x: Axis3, new_y: Axis3, new_z: Axis3) -> Vector3<f32> {
    Vector3::new(
        component(v, new_x),
        component(v, new_y),
        component(v, new_z),
    )
}

/// Returns two arbitrary vectors that are perpendicular to each other and the
/// given vector.
pub fn arbitrary_coordinate_system(v: Vector3<f32>) -> (Vector3<f32>, Vector3<f32>) {
    let v2 = if v.x.abs() > v.y.abs() {
        Vector3::new(-1.0 * v.z, 0.0, v.x).normalize()
    } else {
        Vector3::new(0.0, v.z, -1.0 * v.y).normalize()
    };
    let v3 = v.cross(v2);
    (v2, v3)
}

#[cfg(test)]
mod reflect_tests {
    use crate::math::vector;
    use crate::test::ApproxEq;
    use cgmath::Vector3;

    #[test]
    fn reflect_approaching_45_degrees() {
        let incoming = Vector3::new(1.0, -1.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);

        let reflection = vector::reflect(incoming, normal);
        let expected = Vector3::new(1.0, 1.0, 0.0);
        assert!(reflection.approx_eq(&expected));
    }

    #[test]
    fn reflect_approaching_slant() {
        let incoming = Vector3::new(0.0, -1.0, 0.0);
        let normal = Vector3::new(f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0);

        let reflection = vector::reflect(incoming, normal);
        let expected = Vector3::new(1.0, 0.0, 0.0);
        assert!(reflection.approx_eq(&expected));
    }
}
