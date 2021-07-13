//! Contains data structures that can be used to construct a renderable 3D
//! world.
use crate::{
    interaction::SurfaceInteraction, material::Material, mesh::Mesh, ray::Ray, shape::Shape,
};

// A data structure representing a scene that can be rendered by casting rays
// into it.
pub enum Renderable<'msh, 'mtrx, 'mtrl> {
    Primitive(Primitive<'msh, 'mtrx, 'mtrl>),
    Vector(Vec<Renderable<'msh, 'mtrx, 'mtrl>>),
}

/// Combines a reference to a shape and a reference to a material. This is the
/// basic primitive used in the construction of any renderable.
#[derive(Debug, Clone, Copy)]
pub struct Primitive<'msh, 'mtrx, 'mtrl> {
    pub shape: Shape<'msh, 'mtrx>,
    pub material: &'mtrl Material,
}

impl<'msh, 'mtrx, 'mtrl> Renderable<'msh, 'mtrx, 'mtrl> {
    pub fn primitive(shape: Shape<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
        Self::Primitive(Primitive { shape, material })
    }

    // Find the first primitive the ray intersects. Return the parametric value
    // at the intersection, a reference to the primitive, and a description of
    // the primitive-ray interaction.
    pub fn ray_intersection(
        &self,
        ray: &Ray,
    ) -> Option<(f32, Primitive<'msh, 'mtrx, 'mtrl>, SurfaceInteraction)> {
        match self {
            Renderable::Primitive(p) => p
                .shape
                .ray_intersection(ray)
                .map(|(t, interaction)| (t, p.clone(), interaction)),
            Renderable::Vector(ps) => ps
                .iter()
                .filter_map(|r| {
                    r.ray_intersection(ray)
                        .map(|(t, p, interaction)| (t, p, interaction))
                })
                .min_by(|(t1, _, _), (t2, _, _)| cmp_ignore_nan(t1, t2)),
        }
    }
}

/// Returns an ordering between two numbers. This function assumes neither
/// number is NaN.
fn cmp_ignore_nan(x: &f32, y: &f32) -> std::cmp::Ordering {
    x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Less)
}

#[cfg(test)]
mod ray_intersections_tests {
    use crate::{
        color::Rgb, light::PointLight, material::Material, math::matrix::identity4, ray::Ray,
        renderable::Renderable, shape::Shape, test::ApproxEq,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn ray_intersects_spheres() -> Result<(), String> {
        let identity = identity4();
        let scale = Matrix4::from_scale(0.5);
        let inv_scale = scale.inverse_transform().unwrap();
        let material = Material::new(Rgb::new(0.8, 1.0, 0.6), 0.0, 0.7, 0.2, 0.0, 0.0);
        let sphere1 = Shape::sphere(&identity, &identity, false);
        let sphere2 = Shape::sphere(&scale, &inv_scale, false);
        let primitive1 = Renderable::primitive(sphere1, &material);
        let primitive2 = Renderable::primitive(sphere2, &material);
        let renderable = Renderable::Vector(vec![primitive1, primitive2]);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        if let Some((t, _, _)) = renderable.ray_intersection(&ray) {
            assert!(t.approx_eq(&4.0));
            // Other intersections would be at 4.5, 5.5, 6.0.
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }
}
