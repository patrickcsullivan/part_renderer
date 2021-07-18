use cgmath::{BaseNum, Point2};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Bounds2<S> {
    pub min: Point2<S>,
    pub max: Point2<S>,
}

impl<S> Bounds2<S> {
    pub fn new(min: Point2<S>, max: Point2<S>) -> Self {
        Self { min, max }
    }
}

impl<S: BaseNum> Bounds2<S> {
    pub fn area(&self) -> S {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }
}
