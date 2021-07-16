use cgmath::{Matrix4, Point3, Transform, Vector3};

use crate::{geometry::bounds::Bounds2, ray::Ray};

use super::{CameraSample, Film, GenerateRay, Medium};

pub struct ProjectiveMatrices {
    camera_to_screen: Matrix4<f32>,
    raster_to_camera: Matrix4<f32>,
    screen_to_raster: Matrix4<f32>,
    raster_to_screen: Matrix4<f32>,
}

impl ProjectiveMatrices {
    pub fn new(
        camera_to_screen: Matrix4<f32>,
        screen_window: &Bounds2<f32>,
        film: &Film,
    ) -> Option<Self> {
        let screen_to_raster: Matrix4<f32> =
            Matrix4::from_nonuniform_scale(film.resolution.0 as f32, film.resolution.1 as f32, 1.0)
                * Matrix4::from_nonuniform_scale(
                    1.0 / (screen_window.max.x - screen_window.min.x),
                    1.0 / (screen_window.max.y - screen_window.min.y),
                    1.0,
                )
                * Matrix4::from_translation(Vector3::new(
                    -1.0 * screen_window.min.x,
                    -1.0 * screen_window.max.y,
                    0.0,
                ));
        let raster_to_screen = screen_to_raster.inverse_transform()?;
        let raster_to_camera = camera_to_screen.inverse_transform()? * raster_to_screen;
        Some(Self {
            camera_to_screen,
            raster_to_camera,
            screen_to_raster,
            raster_to_screen,
        })
    }
}

/// A camera that transforms a 3D scene onto a 2D image using an orthgraphic
/// projective tranformation matrix.
pub struct OrthographicCamera {
    pub camera_to_world: Matrix4<f32>,
    pub screen_window: Bounds2<f32>,
    // pub shutter_open: f32,
    // pub shutter_close: f32,
    // pub lens_radius: f32,
    // pub focal_distance: f32,
    pub film: Film,
    pub medium: Medium,

    projective_transforms: ProjectiveMatrices,

    /// Translation that a ray origin undergoes in camera space when the origin
    /// is shifted one unit (one pixel) in the positive x direction in raster
    /// space.
    dx_camera: Vector3<f32>,

    /// Translation that a ray origin undergoes in camera space when the origin
    /// is shifted one unit (one pixel) in the positive y direction in raster
    /// space.
    dy_camera: Vector3<f32>,
}

impl OrthographicCamera {
    fn orthographic(z_near: f32, z_far: f32) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(1.0, 1.0, 1.0 / (z_far - z_near))
            * Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.0 * z_near))
    }

    pub fn new(
        camera_to_world: Matrix4<f32>,
        camera_to_screen: Matrix4<f32>,
        screen_window: Bounds2<f32>,
        film: Film,
        medium: Medium,
    ) -> Option<Self> {
        // TODO: Initialize depth of field params.

        let projective_transforms =
            ProjectiveMatrices::new(camera_to_screen, &screen_window, &film)?;

        let dx_camera = projective_transforms
            .raster_to_camera
            .transform_vector(Vector3::new(1.0, 0.0, 0.0));
        let dy_camera = projective_transforms
            .raster_to_camera
            .transform_vector(Vector3::new(0.0, 1.0, 0.0));

        Some(Self {
            camera_to_world,
            projective_transforms,
            screen_window,
            film,
            medium,

            dx_camera,
            dy_camera,
        })
    }
}

impl GenerateRay for OrthographicCamera {
    fn generate_ray(&self, sample: CameraSample) -> (crate::ray::Ray, f32) {
        let p_raster = Point3::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self
            .projective_transforms
            .raster_to_camera
            .transform_point(p_raster);
        let ray = Ray::new(p_camera, Vector3::new(0.0, 0.0, 1.0), self.medium);

        // TODO: Modify ray for depth of field.

        todo!()
    }
}
