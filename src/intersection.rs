use crate::{interaction::SurfaceInteraction, primitive::Primitive};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Intersection<'msh, 'shp, 'mtrx, 'mtrl> {
    pub t: f32,
    pub interaction: SurfaceInteraction,
    pub primitive: Primitive<'msh, 'shp, 'mtrx, 'mtrl>,
}

pub struct Intersections<'msh, 'shp, 'mtrx, 'mtrl> {
    values: Vec<Intersection<'msh, 'shp, 'mtrx, 'mtrl>>,
}

impl<'msh, 'shp, 'mtrx, 'mtrl> Intersections<'msh, 'shp, 'mtrx, 'mtrl> {
    pub fn empty() -> Self {
        Self { values: vec![] }
    }

    pub fn new(values: Vec<Intersection<'msh, 'shp, 'mtrx, 'mtrl>>) -> Self {
        let mut inters = Self { values };
        inters.filter_and_sort_values();
        inters
    }

    pub fn values(&self) -> Vec<&Intersection<'msh, 'shp, 'mtrx, 'mtrl>> {
        self.values.iter().collect()
    }

    pub fn hit(&self) -> Option<&Intersection<'msh, 'shp, 'mtrx, 'mtrl>> {
        self.values.iter().find(|i| i.t > 0.0)
    }

    pub fn append(&mut self, other: &mut Self) {
        self.values.append(&mut other.values);
        self.filter_and_sort_values();
    }

    fn filter_and_sort_values(&mut self) {
        self.values.retain(|v| v.t.is_finite());
        self.values.sort_by(|x, y| {
            if x.t < y.t {
                Ordering::Less
            } else if (x.t - y.t).abs() < f32::EPSILON {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
    }
}

#[cfg(test)]
mod hit_tests {
    use cgmath::{Point3, Vector3};

    use crate::color::Rgb;
    use crate::interaction::SurfaceInteraction;
    use crate::intersection::{Intersection, Intersections};
    use crate::material::Material;
    use crate::math::matrix::identity4;
    use crate::object::Object;
    use crate::primitive::Primitive;
    use crate::test::ApproxEq;

    fn test_material() -> Material {
        Material {
            color: Rgb::new(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflective: 0.0,
        }
    }

    fn test_interaction() -> SurfaceInteraction {
        SurfaceInteraction {
            point: Point3::new(0.0, 0.0, 0.0),
            neg_ray_direction: Vector3::new(0.0, 0.0, 1.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    #[test]
    fn when_all_positive_t() {
        let identity = identity4();
        let material = test_material();
        let sphere = Object::sphere(&identity, &identity, false);
        let intersections = Intersections::new(vec![
            Intersection {
                t: 1.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
            Intersection {
                t: 2.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
        ]);
        assert!(intersections.hit().map(|h| h.t).approx_eq(&Some(1.0)));
    }

    #[test]
    fn when_some_negative_t() {
        let identity = identity4();
        let material = test_material();
        let sphere = Object::sphere(&identity, &identity, false);
        let intersections = Intersections::new(vec![
            Intersection {
                t: -1.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
            Intersection {
                t: 1.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
        ]);
        assert!(intersections.hit().map(|h| h.t).approx_eq(&Some(1.0)));
    }

    #[test]
    fn when_all_negative_t() {
        let identity = identity4();
        let material = test_material();
        let sphere = Object::sphere(&identity, &identity, false);
        let intersections = Intersections::new(vec![
            Intersection {
                t: -2.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
            Intersection {
                t: -1.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
        ]);
        assert!(intersections.hit().approx_eq(&None));
    }

    #[test]
    fn always_lowest_positive() {
        let identity = identity4();
        let material = test_material();
        let sphere = Object::sphere(&identity, &identity, false);
        let intersections = Intersections::new(vec![
            Intersection {
                t: 5.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
            Intersection {
                t: 7.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
            Intersection {
                t: -3.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
            Intersection {
                t: 2.0,
                interaction: test_interaction(),
                primitive: Primitive::new(&sphere, &material),
            },
        ]);
        assert!(intersections.hit().map(|h| h.t).approx_eq(&Some(2.0)));
    }
}
