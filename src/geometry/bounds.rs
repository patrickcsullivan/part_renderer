#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Bounds2<S> {
    pub min: cgmath::Point2<S>,
    pub max: cgmath::Point2<S>,
}
