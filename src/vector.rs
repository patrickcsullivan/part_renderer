use cgmath::{InnerSpace, Vector3};

pub fn reflect(incoming: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    incoming - normal * 2.0 * incoming.dot(normal)
}

#[cfg(test)]
mod tests {
    use crate::test::ApproxEq;
    use crate::vector;
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
