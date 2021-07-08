use crate::interaction::SurfaceInteraction;
use crate::math::axis::Axis3;
use crate::math::baycentric;
use crate::math::vector;
use crate::mesh::TriangleMesh;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix4, Point2, Point3, Transform, Vector3, Vector4};

/// A reference to an individual triangle in a mesh.
pub struct Triangle<'tm, 'mtrx> {
    mesh: &'tm TriangleMesh<'mtrx>,
    index_in_mesh: usize,
}

impl<'tm, 'mtrx> Triangle<'tm, 'mtrx> {
    pub fn object_space_vertices(&self) -> (Point3<f32>, Point3<f32>, Point3<f32>) {
        let (i1, i2, i3) = self.mesh.triangle_vertex_indices[self.index_in_mesh];
        let p1 = self
            .mesh
            .world_to_object
            .transform_point(self.mesh.world_space_vertices[i1]);
        let p2 = self
            .mesh
            .world_to_object
            .transform_point(self.mesh.world_space_vertices[i2]);
        let p3 = self
            .mesh
            .world_to_object
            .transform_point(self.mesh.world_space_vertices[i3]);
        (p1, p2, p3)
    }

    pub fn world_space_vertices(&self) -> (Point3<f32>, Point3<f32>, Point3<f32>) {
        let (i1, i2, i3) = self.mesh.triangle_vertex_indices[self.index_in_mesh];
        let p1 = self.mesh.world_space_vertices[i1];
        let p2 = self.mesh.world_space_vertices[i2];
        let p3 = self.mesh.world_space_vertices[i3];
        (p1, p2, p3)
    }

    /// Returns the UV coordinates for each of the triangle's vertices. If the
    /// mesh does not contain UV coordinates then default coordinates are
    /// returned.
    pub fn uv_vertices(&self) -> (Point2<f32>, Point2<f32>, Point2<f32>) {
        if let Some(uvs) = &self.mesh.uvs {
            let (i1, i2, i3) = self.mesh.triangle_vertex_indices[self.index_in_mesh];
            (uvs[i1], uvs[i2], uvs[i3])
        } else {
            (
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 0.0),
                Point2::new(1.0, 1.0),
            )
        }
    }
}

impl<'tm, 'mtrx> TriangleMesh<'mtrx> {
    pub fn triangle_at(&'tm self, index: usize) -> Triangle<'tm, 'mtrx> {
        Triangle {
            mesh: self,
            index_in_mesh: index,
        }
    }
}

impl<'shape, 'tm, 'mtrx> Triangle<'tm, 'mtrx> {
    fn object_to_world(&self) -> &'mtrx Matrix4<f32> {
        self.mesh.object_to_world
    }

    fn world_to_object(&self) -> &'mtrx Matrix4<f32> {
        self.mesh.world_to_object
    }

    fn object_to_world_swaps_handedness(&self) -> bool {
        false // FIXME
    }

    fn reverse_orientation(&self) -> bool {
        self.mesh.reverse_orientation
    }

    fn ray_intersection(&'shape self, ray: &Ray) -> Option<(f32, SurfaceInteraction)> {
        let vertices = self.world_space_vertices();

        if let Some(IntersectionLocation { t, baycentric }) =
            find_intersection_location(vertices, ray)
        {
            let uv_vertices = self.uv_vertices();

            if let Some((_dpdu, _dpdv)) = triangle_partial_derivatives(vertices, uv_vertices) {
                let world_space_hit = baycentric::into_point3(vertices, baycentric);
                let _uv_hit = baycentric::into_point2(uv_vertices, baycentric);
                let normal = (vertices.0 - vertices.2)
                    .cross(vertices.1 - vertices.2)
                    .normalize();
                let interaction =
                    SurfaceInteraction::new(world_space_hit, -1.0 * ray.direction, normal);
                Some((t, interaction))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn does_ray_intersect(&self, ray: &Ray) -> bool {
        self.ray_intersection(ray).is_some()
    }

    fn surface_area(&self) -> f32 {
        todo!()
    }
}

struct IntersectionLocation {
    /// The ray parametric value at which the intersection occurs.
    t: f32,

    /// The baycentric coordinates of the intersection point in the triangle.
    baycentric: (f32, f32, f32),
}

/// Find the point where the ray intersects the triangle defined by the given
/// vertices in world space. Returns the ray's parametric value and the
/// triangles baycentric coordinates where the intersection occurs.
fn find_intersection_location(
    world_space_vertices: (Point3<f32>, Point3<f32>, Point3<f32>),
    ray: &Ray,
) -> Option<IntersectionLocation> {
    // Transform triangle vertices to ray coordinate space.
    let to_ray_coordinate_space = world_to_ray_coordinate_space(ray);
    let (p1, p2, p3) = (
        to_ray_coordinate_space.transform_point(world_space_vertices.0),
        to_ray_coordinate_space.transform_point(world_space_vertices.1),
        to_ray_coordinate_space.transform_point(world_space_vertices.2),
    );

    // Now that the triangle is in ray coordinate space, the ray starts at
    // the origin and has a length of 1 in the positive z direction.

    // Determine if the ray will pass to the left of, to the right of, or
    // through each triangle edge. Do this by projecting the triangle onto
    // the x-y plane and checking the position of the origin relative to
    // each edge.
    let side1 = origin_side(p1, p2);
    let side2 = origin_side(p2, p3);
    let side3 = origin_side(p3, p1);

    // If the origin is to the left of one edge and to the right of another,
    // then it cannot be in the triangle.
    if (side1 < 0.0 || side2 < 0.0 || side3 < 0.0) && (side1 > 0.0 || side2 > 0.0 || side3 > 0.0) {
        return None;
    }

    // If the origin is on all three edges, then the ray is parallel to and
    // "skims" the triangle. We treat this as a non-intersection.
    let side_sum = side1 + side2 + side3;
    if side_sum == 0.0 {
        return None;
    }

    // We know that t will equal `t_scaled / side_sum`, so we can check for
    // out of bounds t values before performing the division.
    let t_scaled = side1 * p1.z + side2 * p2.z + side3 * p3.z;
    if side_sum < 0.0 {
        if t_scaled >= 0.0 {
            // `t_scaled / side_sum` will be <= 0.
            return None;
        }
        if t_scaled < ray.t_max * side_sum {
            // `t_scaled / side_sum` will be > ray.t_max.
            return None;
        }
    } else if side_sum > 0.0 {
        if t_scaled <= 0.0 {
            // `t_scaled / side_sum` will be <= 0.
            return None;
        }
        if t_scaled > ray.t_max * side_sum {
            // `t_scaled / side_sum` will be > ray.t_max.
            return None;
        }
    }

    // At this point, we know there must be a valid intersection.
    let inv_side_sum = 1.0 / side_sum;
    let t = t_scaled * inv_side_sum;

    // Compute baycentric coordinates.
    let b1 = side1 * inv_side_sum;
    let b2 = side2 * inv_side_sum;
    let b3 = side3 * inv_side_sum;

    Some(IntersectionLocation {
        t,
        baycentric: (b1, b2, b3),
    })
}

/// Return a matrix that transforms points from world space to a special ray
/// coordinate space where the ray's origin is at the coordinate system
/// origin and where the ray's largest component is along the positive z
/// axis.
fn world_to_ray_coordinate_space(ray: &Ray) -> Matrix4<f32> {
    // Translate the ray so that its origin is at the coordinate system
    // origin.
    let translate = Matrix4::from_translation(Point3::new(0.0, 0.0, 0.0) - ray.origin);

    // Rotate the x, y, and z axes such that the ray's larget component is
    // along the z axis.
    let permute = match vector::max_dimension(ray.direction) {
        Axis3::X => Matrix4::new(
            0.0, 0.0, 1.0, 0.0, // column 0
            1.0, 0.0, 0.0, 0.0, // column 1
            0.0, 1.0, 0.0, 0.0, // column 2
            0.0, 0.0, 0.0, 1.0, // column 3
        ),
        Axis3::Y => Matrix4::new(
            0.0, 1.0, 0.0, 0.0, // column 0
            0.0, 0.0, 1.0, 0.0, // column 1
            1.0, 0.0, 0.0, 0.0, // column 2
            0.0, 0.0, 0.0, 1.0, // column 3
        ),
        Axis3::Z => Matrix4::from_scale(1.0), // identity
    };

    // Align the ray direction wit the positive z axis.
    let shear = Matrix4::from_cols(
        Vector4::unit_x(),
        Vector4::unit_y(),
        Vector4::new(
            -1.0 * ray.direction.x / ray.direction.z,
            -1.0 * ray.direction.y / ray.direction.z,
            1.0 / ray.direction.x,
            0.0,
        ),
        Vector4::unit_w(),
    );

    shear * permute * translate
}

// TODO: Write docs. This is the edge function.
fn origin_side(p1: Point3<f32>, p2: Point3<f32>) -> f32 {
    let side_f32 = p1.x * p2.y - p2.x * p1.y;
    if side_f32 == 0.0 {
        (p1.x as f64 * p2.y as f64 - p2.x as f64 * p1.y as f64) as f32
    } else {
        side_f32
    }
}

/// Calculates the partial derivatives of (x,y,z) positions on the triangle with
/// respect to the texture coordinates, u and v. Returns the vectors
/// (δx/δu,δy/δu,δz/δu) and (δx/δv,δy/δv,δz/δv) if the triangle is not
/// degenerate.
fn triangle_partial_derivatives(
    world_space_vertices: (Point3<f32>, Point3<f32>, Point3<f32>),
    uv_vertices: (Point2<f32>, Point2<f32>, Point2<f32>),
) -> Option<(Vector3<f32>, Vector3<f32>)> {
    let (p1, p2, p3) = world_space_vertices;
    let (uv1, uv2, uv3) = uv_vertices;

    let delta_uv1_uv3 = uv1 - uv3;
    let delta_uv2_uv3 = uv2 - uv3;
    let delta_p1_p3 = p1 - p3;
    let delta_p2_p3 = p2 - p3;

    // Caclculate the determinant of the uv deltas matrix.
    let determinant = delta_uv1_uv3[0] * delta_uv2_uv3[1] - delta_uv1_uv3[1] * delta_uv2_uv3[0];

    // We'll need to invert the uv deltas matrix, so we need to make sure it's
    // not singular.
    if determinant.abs() < 1e-8 {
        // If the uv deltas matrix is singular, the uv coordinates for the
        // triangle vertices must be degenerate.
        let perp = (p3 - p1).cross(p2 - p1);
        if perp.magnitude2() == 0.0 {
            // The triangle's (x,y,z) coordinates are also degenerate, so we
            // can't compute partial derivatives.
            return None;
        }

        // Return arbintary vectors that are parallel to the triangle and
        // perpendicular to each other.
        let (dpdu, dpdv) = vector::arbitrary_coordinate_system(perp);
        return Some((dpdu, dpdv));
    }

    let inv_determinant = 1.0 / determinant;
    let dpdu = (delta_uv2_uv3[1] * delta_p1_p3 - delta_uv1_uv3[1] * delta_p2_p3) * inv_determinant;
    let dpdv =
        (-1.0 * delta_uv2_uv3[0] * delta_p1_p3 - delta_uv1_uv3[0] * delta_p2_p3) * inv_determinant;
    Some((dpdu, dpdv))
}

#[cfg(test)]
mod ray_intersects_tests {
    use crate::ray::Ray;
    use crate::test::ApproxEq;
    use crate::{math::matrix::identity4, mesh::TiangleMeshBuilder};
    use cgmath::{Point3, Vector3};

    #[test]
    fn ray_parallel_to_triangle() {
        let identity = identity4();
        let object_space_vertices = vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let triangle_vertex_indices = vec![(0, 1, 2)];
        let mesh = TiangleMeshBuilder::new(
            &identity,
            &identity,
            false,
            object_space_vertices,
            triangle_vertex_indices,
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(0.0, -1.0, -2.0), Vector3::new(0.0, 1.0, 0.0));
        let intersection = triangle.ray_intersection(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let identity = identity4();
        let object_space_vertices = vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let triangle_vertex_indices = vec![(0, 1, 2)];
        let mesh = TiangleMeshBuilder::new(
            &identity,
            &identity,
            false,
            object_space_vertices,
            triangle_vertex_indices,
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let intersection = triangle.ray_intersection(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let identity = identity4();
        let object_space_vertices = vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let triangle_vertex_indices = vec![(0, 1, 2)];
        let mesh = TiangleMeshBuilder::new(
            &identity,
            &identity,
            false,
            object_space_vertices,
            triangle_vertex_indices,
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(-1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let intersection = triangle.ray_intersection(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let identity = identity4();
        let object_space_vertices = vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let triangle_vertex_indices = vec![(0, 1, 2)];
        let mesh = TiangleMeshBuilder::new(
            &identity,
            &identity,
            false,
            object_space_vertices,
            triangle_vertex_indices,
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(0.0, -1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let intersection = triangle.ray_intersection(&ray);
        assert!(intersection.is_none());
    }

    #[test]
    fn ray_strikes_triangle() -> Result<(), String> {
        let identity = identity4();
        let object_space_vertices = vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let triangle_vertex_indices = vec![(0, 1, 2)];
        let mesh = TiangleMeshBuilder::new(
            &identity,
            &identity,
            false,
            object_space_vertices,
            triangle_vertex_indices,
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(0.0, 0.5, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let intersection = triangle.ray_intersection(&ray);

        if let Some((t, _interaction)) = intersection {
            assert!(t.approx_eq(&2.0));
            Ok(())
        } else {
            Err("Expected intersection. Found none.".to_string())
        }
    }
}
