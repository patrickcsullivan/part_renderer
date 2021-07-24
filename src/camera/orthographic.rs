use super::{Camera, CameraSample, Film, GenerateRay, HasFilm};
use crate::{
    geometry::{axis::Axis2, bounds::Bounds2},
    ray::{Ray, RayDifferential},
};
use cgmath::{
    Angle, InnerSpace, Matrix4, PerspectiveFov, Point2, Point3, Rad, Transform, Vector2, Vector3,
    Vector4,
};

/// A camera that is used to view a scene.
///
/// The camera sits at the origin of camera space and renders images onto a
/// canvas one unit away.
pub struct OrthographicCamera {
    pub film: Film,

    /// The bounds of the screen in screen space. This should be centered at the
    /// origin (around the camera), and its width and height should be
    /// equivalent to the width and height of the screen (at the near clipping
    /// plane) in world or camera space.
    pub screen_bounds: Bounds2<f32>,

    pub camera_to_world: Matrix4<f32>,

    camera_to_screen: Matrix4<f32>,
    screen_to_raster: Matrix4<f32>,
    raster_to_screen: Matrix4<f32>,
    raster_to_camera: Matrix4<f32>,

    /// The amount that a differential ray origin shifts in camera space due to
    /// a single pixel shift in the x direction in raster space.
    ray_dx_camera: Vector3<f32>,

    /// The amount that a differential ray origin shifts in camera space due to
    /// a single pixel shift in the y direction in raster space.
    ray_dy_camera: Vector3<f32>,
}

impl OrthographicCamera {
    /// * `screen_size` - Width and height of the screen in world space. In
    ///   general, this should have the same aspect ratio as `resolution`.
    pub fn new(
        film: Film,
        camera_to_world: Matrix4<f32>,
        z_near: f32,
        z_far: f32,
        screen_size: Vector2<f32>,
    ) -> Self {
        let screen_bounds = Bounds2::new(
            Point2::new(0.0, 0.0) - 0.5 * screen_size,
            Point2::new(0.0, 0.0) + 0.5 * screen_size,
        );
        let camera_to_screen = Self::camera_to_screen(z_near, z_far);
        let screen_to_camera = camera_to_screen.inverse_transform().unwrap();
        let screen_to_raster = Self::screen_to_raster(screen_bounds, film.resolution);
        let raster_to_screen = screen_to_raster.inverse_transform().unwrap();
        let raster_to_camera = screen_to_camera * raster_to_screen;

        let ray_dx_camera = raster_to_camera.transform_vector(Vector3::new(1.0, 0.0, 0.0));
        let ray_dy_camera = raster_to_camera.transform_vector(Vector3::new(0.0, 1.0, 0.0));

        Self {
            film,
            screen_bounds,
            camera_to_world,
            camera_to_screen,
            screen_to_raster,
            raster_to_screen,
            raster_to_camera,
            ray_dx_camera,
            ray_dy_camera,
        }
    }

    /// Returns the projective orthographic matrix that transforms camera space
    /// to screen space.
    ///
    /// * `z_near` - z value of the near clipping plane in camera space.
    /// * `z_far` - z value of the far clipping plane in camera space.
    fn camera_to_screen(z_near: f32, z_far: f32) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(1.0, 1.0, 1.0 / (z_far - z_near))
            * Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.0 * z_near))
    }

    /// Returns the matrix that transforms camera screen space to raster space.
    fn screen_to_raster(screen_bounds: Bounds2<f32>, resolution: Vector2<usize>) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(resolution.x as f32, resolution.y as f32, 1.0)
            * Matrix4::from_nonuniform_scale(
                1.0 / (screen_bounds.max.x - screen_bounds.min.x),
                1.0 / (screen_bounds.min.y - screen_bounds.max.y),
                1.0,
            )
            * Matrix4::from_translation(Vector3::new(
                -1.0 * screen_bounds.min.x,
                -1.0 * screen_bounds.max.y,
                0.0,
            ))
    }

    fn generate_camera_space_ray(&self, sample: &CameraSample) -> Ray {
        let raster_point = Point3::new(sample.film_point.x, sample.film_point.y, 0.0);
        let camera_point = self.raster_to_camera.transform_point(raster_point);

        // TODO: Modify ray for depth of field.
        // TODO: Set ray time.
        // TODO: Set ray medium equal to camera medium.
        Ray::new(camera_point, Vector3::new(0.0, 0.0, 1.0))
    }
}

impl Camera for OrthographicCamera {}

impl GenerateRay for OrthographicCamera {
    fn generate_ray(&self, sample: &CameraSample) -> (Ray, f32) {
        let camera_ray = self.generate_camera_space_ray(sample);
        use crate::geometry::Transform;
        let world_ray = self.camera_to_world.transform(&camera_ray);
        (world_ray, 1.0)
    }

    fn generate_ray_differential(
        &self,
        sample: &CameraSample,
    ) -> (Ray, Option<RayDifferential>, f32) {
        let camera_primary_ray = self.generate_camera_space_ray(sample);
        let camera_ray_differitial = RayDifferential::new(
            camera_primary_ray.origin + self.ray_dx_camera,
            camera_primary_ray.direction,
            camera_primary_ray.origin + self.ray_dy_camera,
            camera_primary_ray.direction,
        );

        use crate::geometry::Transform;
        let world_primay_ray = self.camera_to_world.transform(&camera_primary_ray);
        let world_ray_differential = self.camera_to_world.transform(&camera_ray_differitial);

        (world_primay_ray, Some(world_ray_differential), 1.0)
    }
}

impl HasFilm for OrthographicCamera {
    fn film(&self) -> &Film {
        &self.film
    }
}

#[cfg(test)]
mod generate_ray_tests {
    use crate::{
        camera::{CameraSample, Film, GenerateRay, OrthographicCamera},
        geometry::matrix::identity4,
        ray::Ray,
        scene,
        test::ApproxEq,
    };
    use cgmath::{Matrix4, Point2, Point3, Rad, Transform, Vector2, Vector3};
    use std::f32::consts::{PI, SQRT_2};

    #[test]
    fn untransformed_camera() {
        let camera_to_world = identity4();
        let film = Film::new(400, 200);
        let camera =
            OrthographicCamera::new(film, camera_to_world, 0.0, 100.0, Vector2::new(4.0, 2.0));

        let sample = CameraSample::at_pixel_center(Point2::new(0, 0));
        let (ray, _) = camera.generate_ray(&sample);
        ray.assert_approx_eq(&Ray::new(
            Point3::new(-1.995, 0.995, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ));

        let sample = CameraSample::at_pixel_center(Point2::new(399, 199));
        let (ray, _) = camera.generate_ray(&sample);
        ray.assert_approx_eq(&Ray::new(
            Point3::new(1.995, -0.995, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ));
    }

    #[test]
    fn translated_camera() {
        let camera_to_world = Matrix4::from_translation(Vector3::new(3.0, 3.0, 3.0));
        let film = Film::new(400, 200);
        let camera =
            OrthographicCamera::new(film, camera_to_world, 0.0, 100.0, Vector2::new(4.0, 2.0));

        let sample = CameraSample::at_pixel_center(Point2::new(0, 0));
        let (ray, _) = camera.generate_ray(&sample);
        ray.assert_approx_eq(&Ray::new(
            Point3::new(1.005, 3.995, 3.0),
            Vector3::new(0.0, 0.0, 1.0),
        ));

        let sample = CameraSample::at_pixel_center(Point2::new(399, 199));
        let (ray, _) = camera.generate_ray(&sample);
        ray.assert_approx_eq(&Ray::new(
            Point3::new(4.995, 2.005, 3.0),
            Vector3::new(0.0, 0.0, 1.0),
        ));
    }

    #[test]
    fn rotated_camera() {
        let camera_to_world = Matrix4::from_angle_y(Rad(PI / 2.0));
        let film = Film::new(400, 200);
        let camera =
            OrthographicCamera::new(film, camera_to_world, 0.0, 100.0, Vector2::new(4.0, 2.0));

        let sample = CameraSample::at_pixel_center(Point2::new(0, 0));
        let (ray, _) = camera.generate_ray(&sample);
        ray.assert_approx_eq(&Ray::new(
            Point3::new(0.0, 0.995, 1.995),
            Vector3::new(1.0, 0.0, 0.0),
        ));

        let sample = CameraSample::at_pixel_center(Point2::new(399, 199));
        let (ray, _) = camera.generate_ray(&sample);
        ray.assert_approx_eq(&Ray::new(
            Point3::new(0.0, -0.995, -1.995),
            Vector3::new(1.0, 0.0, 0.0),
        ));
    }
}
