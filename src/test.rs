pub const EPSILON: f32 = 0.0001;

pub trait ApproxEq {
    fn approx_eq(&self, other: &Self) -> bool;
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

impl<'shp, 'mat> ApproxEq for crate::interaction::SurfaceInteraction<'shp, 'mat> {
    fn approx_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.shape, other.shape)
    }
}

impl<'shp, 'mat> ApproxEq for crate::intersection::Intersection<'shp, 'mat> {
    fn approx_eq(&self, other: &Self) -> bool {
        self.t.approx_eq(&other.t) && self.interaction.approx_eq(&other.interaction)
    }
}
