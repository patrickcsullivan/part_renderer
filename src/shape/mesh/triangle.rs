use super::Mesh;
use crate::geometry::{axis::Axis3, point, vector};
use crate::interaction::SurfaceInteraction;
use crate::number::efloat;
use crate::ray::Ray;
use bvh::aabb::{Bounded, AABB};
use cgmath::{InnerSpace, Matrix4, Point2, Point3, Transform, Vector3};

/// A reference to an individual triangle in a mesh.
#[derive(Debug, Clone, Copy)]
pub struct Triangle<'msh, 'mtrx> {
    mesh: &'msh Mesh<'mtrx>,
    index_in_mesh: usize,
}

impl<'msh, 'mtrx> Triangle<'msh, 'mtrx> {
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

impl<'msh, 'mtrx> Mesh<'mtrx> {
    pub fn triangle_at(&'msh self, index: usize) -> Triangle<'msh, 'mtrx> {
        Triangle {
            mesh: self,
            index_in_mesh: index,
        }
    }

    pub fn triangles(&'msh self) -> Vec<Triangle<'msh, 'mtrx>> {
        let triangle_count = self.triangle_vertex_indices.len();
        (0..triangle_count).map(|i| self.triangle_at(i)).collect()
    }
}

impl<'shape, 'msh, 'mtrx> Triangle<'msh, 'mtrx> {
    pub fn object_to_world(&self) -> &'mtrx Matrix4<f32> {
        self.mesh.object_to_world
    }

    pub fn world_to_object(&self) -> &'mtrx Matrix4<f32> {
        self.mesh.world_to_object
    }

    pub fn object_to_world_swaps_handedness(&self) -> bool {
        false // FIXME
    }

    pub fn reverse_orientation(&self) -> bool {
        self.mesh.reverse_orientation
    }

    pub fn ray_intersection(&'shape self, ray: &Ray) -> Option<(f32, SurfaceInteraction)> {
        let (p0, p1, p2) = self.world_space_vertices();
        let (uv0, uv1, uv2) = self.uv_vertices();

        // Transform triangle vertices to ray coordinate space.

        // Start by translating vertices such that the ray origin would be at
        // the coordinate system origin.
        let p0t = p0 + (Point3::new(0.0, 0.0, 0.0) - ray.origin);
        let p1t = p1 + (Point3::new(0.0, 0.0, 0.0) - ray.origin);
        let p2t = p2 + (Point3::new(0.0, 0.0, 0.0) - ray.origin);
        // Permute components of triangle vertices and ray direction. Swap axes
        // such that the ray direction's component with the greatest absolute
        // value is along the z axis.
        let new_z_axis = vector::max_dimension(ray.direction);
        let new_x_axis = match new_z_axis {
            Axis3::X => Axis3::Y,
            Axis3::Y => Axis3::Z,
            Axis3::Z => Axis3::X,
        };
        let new_y_axis = match new_x_axis {
            Axis3::X => Axis3::Y,
            Axis3::Y => Axis3::Z,
            Axis3::Z => Axis3::X,
        };
        let dir_t = vector::permute(ray.direction, new_x_axis, new_y_axis, new_z_axis);
        let p0t = point::permute(p0t, new_x_axis, new_y_axis, new_z_axis);
        let p1t = point::permute(p1t, new_x_axis, new_y_axis, new_z_axis);
        let p2t = point::permute(p2t, new_x_axis, new_y_axis, new_z_axis);
        // Apply shear transformation to translated vertex positions. (Only x
        // and y shears are applied at this time. Shearing on z is applied
        // later.)
        let sx = -1.0 * dir_t.x / dir_t.z;
        let sy = -1.0 * dir_t.y / dir_t.z;
        let sz = 1.0 / dir_t.z;
        let p0t = Point3::new(p0t.x + sx * p0t.z, p0t.y + sy * p0t.z, p0t.z);
        let p1t = Point3::new(p1t.x + sx * p1t.z, p1t.y + sy * p1t.z, p1t.z);
        let p2t = Point3::new(p2t.x + sx * p2t.z, p2t.y + sy * p2t.z, p2t.z);

        // Compute edge function coefficients. Each edge function coefficient
        // tells us if the z axis is left of, right of, or directly on a
        // particular edge of the transformed triangle.
        let e0 = p1t.x * p2t.y - p1t.y * p2t.x;
        let e1 = p2t.x * p0t.y - p2t.y * p0t.x;
        let e2 = p0t.x * p1t.y - p0t.y * p1t.x;
        // Fall back to double precision test at triangle edges
        let (e0, e1, e2) = if e0 == 0.0 || e1 == 0.0 || e2 == 0.0 {
            let p2txp1ty = p2t.x as f64 * p1t.y as f64;
            let p2typ1tx = p2t.y as f64 * p1t.x as f64;
            let e0 = (p2typ1tx - p2txp1ty) as f32;
            let p0txp2ty = p0t.x as f64 * p2t.y as f64;
            let p0typ2tx = p0t.y as f64 * p2t.x as f64;
            let e1 = (p0typ2tx - p0txp2ty) as f32;
            let p1txp0ty = p1t.x as f64 * p0t.y as f64;
            let p1typ0tx = p1t.y as f64 * p0t.x as f64;
            let e2 = (p1typ0tx - p1txp0ty) as f32;
            (e0, e1, e2)
        } else {
            (e0, e1, e2)
        };

        // If the z axis is to the left of one edge and to the right of another,
        // then it cannot be in the triangle.
        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            return None;
        }
        // If the z axis on all three edges, then the ray is parallel to and
        // "skims" the triangle. We treat this as a non-intersection.
        let det = e0 + e1 + e2;
        if det == 0.0 {
            return None;
        }

        // Now apply z shear. We didn't do this earlier because we didn't need
        // to at that time, and if there had been a ray intersection miss then
        // that would have been wasted work. Now we need the z shear so we can
        // find scaled hit distance.
        let p0t = Point3::new(p0t.x, p0t.y, p0t.z * sz);
        let p1t = Point3::new(p1t.x, p1t.y, p1t.z * sz);
        let p2t = Point3::new(p2t.x, p2t.y, p2t.z * sz);

        // Compute scaled hit distance to triangle and test against ray's t range.
        let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if det < 0.0 && (t_scaled >= 0.0 || t_scaled < ray.t_max * det) {
            return None;
        }
        if det > 0.0 && (t_scaled <= 0.0 || t_scaled > ray.t_max * det) {
            return None;
        }

        // Compute t value for triangle intersection
        let inv_det = 1.0 / det;
        let t = t_scaled * inv_det;

        // Ensure that computed t is conservatively greater than zero.

        // Compute delta_z term for triangle t error bounds
        let max_zt = p0.z.abs().max(p1.z.abs()).max(p2.z.abs());
        let delta_z = efloat::gamma(3) * max_zt;
        // Compute delta_x and delta_y terms for triangle t error bounds
        let max_xt = p0.x.abs().max(p1.x.abs()).max(p2.x.abs());
        let max_yt = p0.y.abs().max(p1.y.abs()).max(p2.y.abs());
        let delta_x = efloat::gamma(5) * max_xt;
        let delta_y = efloat::gamma(5) * max_yt;
        // Compute delta_e term for triangle t error bounds
        let delta_e =
            2.0 * (efloat::gamma(2) * max_xt * max_yt + delta_y * max_xt + delta_x * max_yt);
        // Compute delta_t term for triangle t error bounds and check _t_
        let max_e = e0.abs().max(e1.abs()).max(e2.abs());
        let delta_t = 3.0
            * (efloat::gamma(3) * max_e * max_xt + delta_e * max_zt + delta_z * max_e)
            * inv_det.abs();
        if t <= delta_t {
            return None;
        }

        // Compute partial derivatives.
        let (dpdu, dpdv) = triangle_partial_derivatives((p0, p1, p2), (uv0, uv1, uv2))?;

        // Compute baycentric coordinates.
        let b0 = e0 * inv_det;
        let b1 = e1 * inv_det;
        let b2 = e2 * inv_det;

        // Compute error bounds for triangle intersection
        let x_abs_sum = (b0 * p0.x).abs() + (b1 * p1.x).abs() + (b2 * p2.x).abs();
        let y_abs_sum = (b0 * p0.y).abs() + (b1 * p1.y).abs() + (b2 * p2.y).abs();
        let z_abs_sum = (b0 * p0.z).abs() + (b1 * p1.z).abs() + (b2 * p2.z).abs();
        let p_error = efloat::gamma(7) * Vector3::new(x_abs_sum, y_abs_sum, z_abs_sum);

        // Interpolate (u,v) coordinates and hit point
        let p_hit = point::add_point3(vec![b0 * p0, b1 * p1, b2 * p2]);
        let uv_hit = point::add_point2(vec![b0 * uv0, b1 * uv1, b2 * uv2]);

        // Test intersection against alpha texture went here...
        let dp02 = p0 - p2;
        let dp12 = p1 - p2;
        let normal = if self.reverse_orientation() || self.object_to_world_swaps_handedness() {
            -1.0 * dp02.cross(dp12).normalize()
        } else {
            dp02.cross(dp12).normalize()
        };

        // Fill in SurfaceInteraction for triangle hit
        let interaction = SurfaceInteraction::new(p_hit, -1.0 * ray.direction, normal, dpdu, dpdv);

        Some((t, interaction))
    }

    fn does_ray_intersect(&self, ray: &Ray) -> bool {
        self.ray_intersection(ray).is_some()
    }

    fn surface_area(&self) -> f32 {
        todo!()
    }
}

impl<'shape, 'msh, 'mtrx> Bounded for Triangle<'msh, 'mtrx> {
    fn aabb(&self) -> bvh::aabb::AABB {
        let (v0, v1, v2) = self.world_space_vertices();
        let min = bvh::Point3::new(
            v0.x.min(v1.x).min(v2.x),
            v0.y.min(v1.y).min(v2.y),
            v0.z.min(v1.z).min(v2.z),
        );
        let max = bvh::Point3::new(
            v0.x.max(v1.x).max(v2.x),
            v0.y.max(v1.y).max(v2.y),
            v0.z.max(v1.z).max(v2.z),
        );
        AABB::with_bounds(min, max)
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
    let (p0, p1, p2) = world_space_vertices;
    let (uv0, uv1, uv2) = uv_vertices;

    let delta_uv0_uv2 = uv0 - uv2;
    let delta_uv1_uv2 = uv1 - uv2;
    let delta_p0_p2 = p0 - p2;
    let delta_p1_p2 = p1 - p2;

    // Caclculate the determinant of the uv deltas matrix.
    let determinant = delta_uv0_uv2[0] * delta_uv1_uv2[1] - delta_uv0_uv2[1] * delta_uv1_uv2[0];

    // We'll need to invert the uv deltas matrix, so we need to make sure it's
    // not singular.
    if determinant.abs() < 1e-8 {
        // If the uv deltas matrix is singular, the uv coordinates for the
        // triangle vertices must be degenerate.
        let perp = (p2 - p0).cross(p1 - p0);
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
    let dpdu = (delta_uv1_uv2[1] * delta_p0_p2 - delta_uv0_uv2[1] * delta_p1_p2) * inv_determinant;
    let dpdv =
        (-1.0 * delta_uv1_uv2[0] * delta_p0_p2 - delta_uv0_uv2[0] * delta_p1_p2) * inv_determinant;
    Some((dpdu, dpdv))
}

#[cfg(test)]
mod ray_intersection_tests {
    use super::super::MeshBuilder;
    use crate::geometry::matrix::identity4;
    use crate::ray::Ray;
    use crate::test::ApproxEq;
    use cgmath::{Point3, Vector3};

    #[test]
    fn parallel_ray_misses() {
        let identity = identity4();
        let mesh = MeshBuilder::new(
            &identity,
            &identity,
            false,
            vec![
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![(0, 1, 2)],
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(0.0, -1.0, -2.0), Vector3::new(0.0, 1.0, 0.0));
        let result = triangle.ray_intersection(&ray);
        assert!(result.is_none(), "Expected to not find intersection.")
    }

    #[test]
    fn outside_p1_p3_misses() {
        let identity = identity4();
        let mesh = MeshBuilder::new(
            &identity,
            &identity,
            false,
            vec![
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![(0, 1, 2)],
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let result = triangle.ray_intersection(&ray);
        assert!(result.is_none(), "Expected to not find intersection.")
    }

    #[test]
    fn outside_p1_p2_misses() {
        let identity = identity4();
        let mesh = MeshBuilder::new(
            &identity,
            &identity,
            false,
            vec![
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![(0, 1, 2)],
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(-1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let result = triangle.ray_intersection(&ray);
        assert!(result.is_none(), "Expected to not find intersection.")
    }

    #[test]
    fn outside_p2_p3_misses() {
        let identity = identity4();
        let mesh = MeshBuilder::new(
            &identity,
            &identity,
            false,
            vec![
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![(0, 1, 2)],
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(0.0, -1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let result = triangle.ray_intersection(&ray);
        assert!(result.is_none(), "Expected to not find intersection.")
    }

    #[test]
    fn skimming_misses() {
        let identity = identity4();
        let mesh = MeshBuilder::new(
            &identity,
            &identity,
            false,
            vec![
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![(0, 1, 2)],
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let result = triangle.ray_intersection(&ray);
        assert!(result.is_none(), "Expected to not find intersection.")
    }

    #[test]
    fn ray_strikes_triangle() -> Result<(), String> {
        let identity = identity4();
        let mesh = MeshBuilder::new(
            &identity,
            &identity,
            false,
            vec![
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![(0, 1, 2)],
        )
        .build();
        let triangle = mesh.triangle_at(0);
        let ray = Ray::new(Point3::new(0.0, 0.5, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let result = triangle.ray_intersection(&ray);
        if let Some((t, interaction)) = result {
            assert!(t.approx_eq(&2.0));
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }
}

#[cfg(test)]
mod triangle_partial_derivatives_tests {
    use crate::test::ApproxEq;

    use super::triangle_partial_derivatives;
    use cgmath::{point2, point3, vec3, Point3, Vector3};

    #[test]
    fn calculates_partial_derivatives() -> Result<(), String> {
        let points = (
            point3(0.0, 0.0, 0.0),
            point3(1.0, 2.0, 3.0),
            point3(4.0, 5.0, 6.0),
        );
        let uvs = (point2(0.0, 0.0), point2(1.0, 0.0), point2(0.0, 1.0));

        if let Some((dpdu, dpdv)) = triangle_partial_derivatives(points, uvs) {
            dpdu.assert_approx_eq(&vec3(1.0, 2.0, 3.0));
            dpdv.assert_approx_eq(&vec3(4.0, 5.0, 6.0));
            Ok(())
        } else {
            Err("Expected result.".to_string())
        }
    }
}
