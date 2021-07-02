use crate::interaction::SurfaceInteraction;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Intersection<'shp, 'mat> {
    pub t: f32,
    pub interaction: SurfaceInteraction<'shp, 'mat>,
}

pub struct Intersections<'shp, 'mat> {
    pub values: Vec<Intersection<'shp, 'mat>>,
}

impl<'shp, 'mat> Intersections<'shp, 'mat> {
    pub fn empty() -> Self {
        Self { values: vec![] }
    }

    pub fn new(values: Vec<Intersection<'shp, 'mat>>) -> Self {
        Self { values }
    }

    pub fn hit(&self) -> Option<&Intersection<'shp, 'mat>> {
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
    use crate::matrix::identity4;
    use crate::shape::Sphere;
    use crate::test::ApproxEq;

    #[test]
    fn when_all_positive_t() {
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
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
        assert!(intersections
            .hit()
            .approx_eq(&Some(&intersections.values[0])));
    }

    #[test]
    fn when_some_negative_t() {
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
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
        assert!(intersections
            .hit()
            .approx_eq(&Some(&intersections.values[1])));
    }

    #[test]
    fn when_all_negative_t() {
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
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
        assert!(intersections.hit().approx_eq(&None));
    }

    #[test]
    fn always_lowest_positive() {
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
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
        assert!(intersections
            .hit()
            .approx_eq(&Some(&intersections.values[3])));
    }
}
