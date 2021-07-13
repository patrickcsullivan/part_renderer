use std::fmt::Debug;

use crate::material::Material;

pub const EPSILON: f32 = 0.0001;

pub trait ApproxEq: Debug {
    fn approx_eq(&self, other: &Self) -> bool;

    fn assert_approx_eq(&self, other: &Self) {
        assert!(
            self.approx_eq(other),
            "Values were not approximately equal.\nLeft value: `{:#?}`\nRight value: `{:#?}`",
            self,
            other
        );
    }
}

impl<T> ApproxEq for &T
where
    T: ApproxEq,
{
    fn approx_eq(&self, other: &Self) -> bool {
        (*self).approx_eq(*other)
    }
}

impl<T> ApproxEq for Option<T>
where
    T: ApproxEq,
{
    fn approx_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(left), Some(right)) => left.approx_eq(right),
            _ => false,
        }
    }
}

impl ApproxEq for f32 {
    fn approx_eq(&self, other: &Self) -> bool {
        (self - other).abs() < EPSILON
    }
}

impl ApproxEq for crate::color::Rgb {
    fn approx_eq(&self, other: &Self) -> bool {
        self.r.approx_eq(&other.r) && self.g.approx_eq(&other.g) && self.b.approx_eq(&other.b)
    }
}

impl ApproxEq for cgmath::Point2<f32> {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y)
    }
}

impl ApproxEq for cgmath::Point3<f32> {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y) && self.z.approx_eq(&other.z)
    }
}

impl ApproxEq for cgmath::Vector3<f32> {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y) && self.z.approx_eq(&other.z)
    }
}

impl ApproxEq for cgmath::Vector4<f32> {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x)
            && self.y.approx_eq(&other.y)
            && self.z.approx_eq(&other.z)
            && self.w.approx_eq(&other.w)
    }
}
impl ApproxEq for cgmath::Matrix4<f32> {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x)
            && self.y.approx_eq(&other.y)
            && self.z.approx_eq(&other.z)
            && self.w.approx_eq(&other.w)
    }
}

impl ApproxEq for crate::ray::Ray {
    fn approx_eq(&self, other: &Self) -> bool {
        self.origin.approx_eq(&other.origin) && self.direction.approx_eq(&other.direction)
    }
}

impl<'msh, 'mtrx, 'mtrl> ApproxEq for crate::interaction::SurfaceInteraction {
    fn approx_eq(&self, _other: &Self) -> bool {
        // TODO: For now it doesn't seem possible to compare two trait object
        // references. It might be necessary to give shape's some sort of id so
        // that they can be compared.
        true

        // std::ptr::eq(self.shape, other.shape)
    }
}
