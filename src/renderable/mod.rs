//! Contains data structures that can be used to construct a renderable 3D
//! world.
use crate::{
    interaction::SurfaceInteraction,
    material::Material,
    mesh::{Mesh, Triangle},
    ray::Ray,
    shape::Shape,
};
use bvh::bvh::BVH;
use bvh::{aabb::Bounded, bounding_hierarchy::BHShape};

// A data structure representing a scene that can be rendered by casting rays
// into it.
pub enum Renderable<'msh, 'mtrx, 'mtrl> {
    Primitive(Primitive<'msh, 'mtrx, 'mtrl>),
    Vector(Vec<Renderable<'msh, 'mtrx, 'mtrl>>),
    BVH(Vec<Primitive<'msh, 'mtrx, 'mtrl>>, BVH),
}

impl<'msh, 'mtrx, 'mtrl> Renderable<'msh, 'mtrx, 'mtrl> {
    pub fn primitive(shape: Shape<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
        Self::Primitive(Primitive::new(shape, material))
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
                .map(|(t, interaction)| (t, *p, interaction)),
            Renderable::Vector(ps) => ps
                .iter()
                .filter_map(|r| {
                    r.ray_intersection(ray)
                        .map(|(t, p, interaction)| (t, p, interaction))
                })
                .min_by(|(t1, _, _), (t2, _, _)| cmp_ignore_nan(t1, t2)),
            Renderable::BVH(ps, bvh) => {
                let hit_primitives = bvh.traverse(&ray.into(), ps);
                hit_primitives
                    .iter()
                    .filter_map(|&&p| {
                        p.shape
                            .ray_intersection(ray)
                            .map(|(t, interaction)| (t, p, interaction))
                    })
                    .min_by(|(t1, _, _), (t2, _, _)| cmp_ignore_nan(t1, t2))
            }
        }
    }

    pub fn from_mesh(mesh: &'msh Mesh<'mtrx>, material: &'mtrl Material) -> Self {
        let mut primitives: Vec<Primitive> = mesh
            .triangles()
            .into_iter()
            .map(|t| Primitive::new(Shape::Triangle(t), material))
            .collect();
        let bvh = BVH::build(&mut primitives);
        Self::BVH(primitives, bvh)
    }

    // pub fn from_triangle(triangle: Triangle<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
    //     Self::Primitive(Primitive {
    //         shape: Shape::Triangle(triangle),
    //         material,
    //     })
    // }
}

/// Combines a reference to a shape and a reference to a material. This is the
/// basic primitive used in the construction of any renderable.
#[derive(Debug, Clone, Copy)]
pub struct Primitive<'msh, 'mtrx, 'mtrl> {
    pub shape: Shape<'msh, 'mtrx>,
    pub material: &'mtrl Material,
    bvh_node_index: usize,
}

impl<'msh, 'mtrx, 'mtrl> Primitive<'msh, 'mtrx, 'mtrl> {
    pub fn new(shape: Shape<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
        Self {
            shape,
            material,
            bvh_node_index: 0,
        }
    }
}

impl<'msh, 'mtrx, 'mtrl> Bounded for Primitive<'msh, 'mtrx, 'mtrl> {
    fn aabb(&self) -> bvh::aabb::AABB {
        self.shape.aabb()
    }
}

impl<'msh, 'mtrx, 'mtrl> BHShape for Primitive<'msh, 'mtrx, 'mtrl> {
    fn set_bh_node_index(&mut self, index: usize) {
        self.bvh_node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_node_index
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
        color::Rgb, geometry::matrix::identity4, light::PointLight, material::Material,
        medium::Medium, ray::Ray, renderable::Renderable, shape::Shape, test::ApproxEq,
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
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vector3::new(0.0, 0.0, 1.0),
            Medium::new(),
        );
        if let Some((t, _, _)) = renderable.ray_intersection(&ray) {
            assert!(t.approx_eq(&4.0));
            // Other intersections would be at 4.5, 5.5, 6.0.
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }
}
