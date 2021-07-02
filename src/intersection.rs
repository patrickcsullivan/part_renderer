use crate::interaction::SurfaceInteraction;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Intersection<'shp, 'mtrx, 'mtrl> {
    pub t: f32,
    pub interaction: SurfaceInteraction<'shp, 'mtrx, 'mtrl>,
}

pub struct Intersections<'shp, 'mtrx, 'mtrl> {
    pub values: Vec<Intersection<'shp, 'mtrx, 'mtrl>>,
}

impl<'shp, 'mtrx, 'mtrl> Intersections<'shp, 'mtrx, 'mtrl> {
    pub fn empty() -> Self {
        Self { values: vec![] }
    }

    pub fn new(values: Vec<Intersection<'shp, 'mtrx, 'mtrl>>) -> Self {
        Self { values }
    }

    pub fn hit(&self) -> Option<&Intersection<'shp, 'mtrx, 'mtrl>> {
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
    use crate::color::Rgb;
    use crate::interaction::SurfaceInteraction;
    use crate::intersection::{Intersection, Intersections};
    use crate::material::Material;
    use crate::matrix::identity4;
    use crate::shape::Sphere;
    use crate::test::ApproxEq;

    fn test_material() -> Material {
        Material {
            color: Rgb::new(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
        }
    }

    #[test]
    fn when_all_positive_t() {
        let identity = identity4();
        let material = test_material();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = Intersections::new(vec![
            Intersection {
                t: 1.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
            Intersection {
                t: 2.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
        ]);
        assert!(intersections
            .hit()
            .approx_eq(&Some(&intersections.values[0])));
    }

    #[test]
    fn when_some_negative_t() {
        let identity = identity4();
        let material = test_material();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = Intersections::new(vec![
            Intersection {
                t: -1.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
            Intersection {
                t: 1.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
        ]);
        assert!(intersections
            .hit()
            .approx_eq(&Some(&intersections.values[1])));
    }

    #[test]
    fn when_all_negative_t() {
        let identity = identity4();
        let material = test_material();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = Intersections::new(vec![
            Intersection {
                t: -2.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
            Intersection {
                t: -1.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
        ]);
        assert!(intersections.hit().approx_eq(&None));
    }

    #[test]
    fn always_lowest_positive() {
        let identity = identity4();
        let material = test_material();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = Intersections::new(vec![
            Intersection {
                t: 5.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
            Intersection {
                t: 7.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
            Intersection {
                t: -3.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
            Intersection {
                t: 2.0,
                interaction: SurfaceInteraction {
                    shape: Box::new(&sphere),
                },
            },
        ]);
        assert!(intersections
            .hit()
            .approx_eq(&Some(&intersections.values[3])));
    }
}
