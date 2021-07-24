use cgmath::{BaseNum, Point2, Vector2};

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

    /// Return a vector from `min` to `max`.
    pub fn diagonal(&self) -> Vector2<S> {
        self.max - self.min
    }
}

impl Bounds2<usize> {
    /// Return the intersection of the bounding boxes.
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let min = Point2::new(self.min.x.max(other.min.x), self.min.y.max(other.min.y));
        let max = Point2::new(self.max.x.min(other.max.x), self.max.y.min(other.max.y));
        if min.x <= max.x && min.y <= max.y {
            Some(Self { min, max })
        } else {
            None
        }
    }

    /// Return the range of points inside the bounds, where the lower bounds are
    /// inclusive and the upper bounds are exclusive.
    pub fn range(&self) -> Vec<Point2<usize>> {
        let xs = self.min.x..self.max.x;
        let ys = self.min.y..self.max.y;
        ys.flat_map(|y| xs.clone().map(move |x| (x, y)))
            .map(|(x, y)| Point2::new(x, y))
            .collect()
    }
}

impl Into<Bounds2<f32>> for Bounds2<usize> {
    fn into(self) -> Bounds2<f32> {
        Bounds2::new(
            Point2::new(self.min.x as f32, self.min.y as f32),
            Point2::new(self.max.x as f32, self.max.y as f32),
        )
    }
}

#[cfg(test)]
mod range_tests {
    use super::Bounds2;
    use cgmath::Point2;

    #[test]
    fn min_inclusive_max_exclusive() {
        let bounds: Bounds2<usize> = Bounds2::new(Point2::new(-1, -1), Point2::new(1, 1));
        let points = bounds.range();
        assert_eq!(
            points,
            vec![
                Point2::new(-1, -1),
                Point2::new(-1, 0),
                Point2::new(0, -1),
                Point2::new(0, 0)
            ]
        );
    }
}
