use cgmath::Point3;

use crate::color::Rgb;

pub struct PointLight {
    pub intensity: Rgb,
    pub positiong: Point3<f32>,
}

#[cfg(test)]
mod tests {
    use crate::test::ApproxEq;
    use crate::vector;
    use cgmath::Vector3;

    #[test]
    fn reflect_approaching_45_degrees() {}
}
