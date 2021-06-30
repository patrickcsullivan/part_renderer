use crate::interaction::SurfaceInteraction;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Intersection<'shp> {
    pub t: f32,
    pub interaction: SurfaceInteraction<'shp>,
}

pub struct Intersections<'shp> {
    pub values: Vec<Intersection<'shp>>,
}

impl<'shp> Intersections<'shp> {
    pub fn empty() -> Self {
        Self { values: vec![] }
    }

    pub fn new(values: Vec<Intersection<'shp>>) -> Self {
        Self { values }
    }

    pub fn hit(&self) -> Option<&Intersection<'shp>> {
        self.values.iter().filter(|v| v.t > 0.0).min_by(|x, y| {
            if x.t < y.t {
                Ordering::Less
            } else if (x.t - y.t).abs() < f32::EPSILON {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        })
    }
}

#[cfg(test)]
mod hit_tests {
    use crate::interaction::SurfaceInteraction;
    use crate::intersection::{Intersection, Intersections};
    use crate::shape::Sphere;

    #[test]
    fn when_all_positive_t() {
        let sphere = Sphere {};
        let intersections = Intersections::new(vec![
            Intersection {
                t: 1.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
            Intersection {
                t: 2.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
        ]);
        assert_eq!(intersections.hit(), Some(&intersections.values[0]));
    }

    #[test]
    fn when_some_negative_t() {
        let sphere = Sphere {};
        let intersections = Intersections::new(vec![
            Intersection {
                t: -1.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
            Intersection {
                t: 1.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
        ]);
        assert_eq!(intersections.hit(), Some(&intersections.values[1]));
    }

    #[test]
    fn when_all_negative_t() {
        let sphere = Sphere {};
        let intersections = Intersections::new(vec![
            Intersection {
                t: -2.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
            Intersection {
                t: -1.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
        ]);
        assert_eq!(intersections.hit(), None);
    }

    #[test]
    fn always_lowest_positive() {
        let sphere = Sphere {};
        let intersections = Intersections::new(vec![
            Intersection {
                t: 5.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
            Intersection {
                t: 7.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
            Intersection {
                t: -3.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
            Intersection {
                t: 2.0,
                interaction: SurfaceInteraction { shape: &sphere },
            },
        ]);
        assert_eq!(intersections.hit(), Some(&intersections.values[3]));
    }
}
