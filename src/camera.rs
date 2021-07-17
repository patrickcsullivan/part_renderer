use cgmath::{
    Angle, InnerSpace, Matrix4, PerspectiveFov, Point2, Point3, Rad, Transform, Vector2, Vector3,
    Vector4,
};

use crate::ray::Ray;

/// A 2D plane of pixels onto which a final image is rendered.
pub struct Film {
    // The images
    pub resolution: Vector2<f32>,
}

impl Film {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            resolution: Vector2::new(x as f32, y as f32),
        }
    }
}

/// Container for all the information needed to generate a ray from a cameraa.
#[derive(Debug, Clone, Copy)]
pub struct CameraSample {
    /// The point on the film in raster space to which a generated ray will
    /// carry radiance.
    pub film_point: Point2<f32>,
}

impl CameraSample {
    pub fn new(film_point: Point2<f32>) -> Self {
        Self { film_point }
    }

    pub fn at_pixel_center(pixel: Point2<usize>) -> Self {
        Self {
            film_point: Point2::new(pixel.x as f32 + 0.5, pixel.y as f32 + 0.5),
        }
    }
}

/// A camera that is used to view a scene.
///
/// The camera sits at the origin of camera space and renders images onto a
/// canvas one unit away.
pub struct Camera {
    pub film: Film,

    /// The field of view for the largest dimension.
    pub fov: Rad<f32>,

    /// Transformation matrix from world space to camera space.
    pub world_to_camera: Matrix4<f32>,

    /// Half the width of the camera in world space.
    half_width: f32,

    /// Half the height of the camera in world space.
    half_height: f32,

    /// Width of a square pixel in world space.
    pixel_size: f32,
}

impl Camera {
    pub fn new(film: Film, fov: Rad<f32>, world_to_camera: Matrix4<f32>) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = film.resolution.x / film.resolution.y;
        let (half_width, half_height) = if aspect >= 1.0 {
            // horizontal
            (half_view, half_view / aspect)
        } else {
            // vertical
            (half_view * aspect, half_view)
        };
        let pixel_size = half_width * 2.0 / film.resolution.x as f32;

        Self {
            film,
            fov,
            world_to_camera,
            half_width,
            half_height,
            pixel_size,
        }
    }

    /// Generate a ray for the given sample.
    ///
    /// This method also returns a radiance contribution value that indicates
    /// how much the radiance carried by the ray contributes to the final image
    /// on the film plane. This value can be 1.0 for simple camera models, or it
    /// may vary for cameras that simulate physical lenses.
    pub fn generate_ray(&self, sample: &CameraSample) -> (Ray, f32) {
        // Offset, measured in camera space, from canvase edge to center of
        // pixel.
        let x_offset = sample.film_point.x * self.pixel_size;
        let y_offset = sample.film_point.y * self.pixel_size;

        // Compute the pixel's position in camera space. By default camera looks
        // towards -z in LH coordinate system, so +x points left and +y points
        // up. The canvas is at z = -1.
        let pixel_cs = Point3::new(
            self.half_width - x_offset,
            self.half_height - y_offset,
            -1.0,
        );

        let camera_to_world = self.world_to_camera.inverse_transform().unwrap();
        let pixel_ws = camera_to_world.transform_point(pixel_cs);
        let ray_origin_ws = camera_to_world.transform_point(Point3::new(0.0, 0.0, 0.0));
        let ray_direction_ws = (pixel_ws - ray_origin_ws).normalize();

        let ray = Ray::new(ray_origin_ws, ray_direction_ws);
        (ray, 1.0)
    }
}

pub fn view_transform(from: Point3<f32>, to: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);
    let orientation = Matrix4::from_cols(
        Vector4::new(left.x, true_up.x, -1.0 * forward.x, 0.0),
        Vector4::new(left.y, true_up.y, -1.0 * forward.y, 0.0),
        Vector4::new(left.z, true_up.z, -1.0 * forward.z, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    );
    orientation * Matrix4::from_translation(Point3::new(0.0, 0.0, 0.0) - from)
}

#[cfg(test)]
mod view_transform_tests {
    use crate::{camera::view_transform, math::matrix::identity4, test::ApproxEq};
    use cgmath::{Matrix4, Point3, Vector3, Vector4};

    #[test]
    fn default_orientation_looking_in_negative_z() {
        let from = Point3::new(0.0, 0.0, 0.0);
        let to = Point3::new(0.0, 0.0, -1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert!(t.approx_eq(&identity4()));
    }

    #[test]
    fn looking_in_positive_z() {
        let from = Point3::new(0.0, 0.0, 0.0);
        let to = Point3::new(0.0, 0.0, 1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert!(t.approx_eq(&Matrix4::from_nonuniform_scale(-1.0, 1.0, -1.0)));
    }

    #[test]
    fn view_moves_world() {
        let from = Point3::new(0.0, 0.0, 8.0);
        let to = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert!(t.approx_eq(&Matrix4::from_translation(Vector3::new(0.0, 0.0, -8.0))));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point3::new(1.0, 3.0, 2.0);
        let to = Point3::new(4.0, -2.0, 8.0);
        let up = Vector3::new(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        let expected = Matrix4::from_cols(
            Vector4::new(-0.50709, 0.76772, -0.35857, 0.0),
            Vector4::new(0.50709, 0.60609, 0.59761, 0.0),
            Vector4::new(0.67612, 0.12122, -0.71714, 0.0),
            Vector4::new(-2.36643, -2.82843, 0.0, 1.0),
        );
        assert!(t.approx_eq(&expected));
    }
}

#[cfg(test)]
mod pixel_size_tests {
    use crate::{
        camera::{Camera, Film},
        math::matrix::identity4,
        test::ApproxEq,
    };
    use cgmath::{Angle, Deg, Rad};
    use std::f32::consts::PI;

    #[test]
    fn for_horizontal_canvas() {
        let film = Film::new(200, 125);
        let camera = Camera::new(film, Rad(PI / 2.0), identity4());
        assert!(camera.pixel_size.approx_eq(&0.01));
    }

    #[test]
    fn for_vertical_canvas() {
        let film = Film::new(125, 200);
        let camera = Camera::new(film, Rad(PI / 2.0), identity4());
        assert!(camera.pixel_size.approx_eq(&0.01));
    }
}

#[cfg(test)]
mod ray_for_pixel_tests {
    use crate::{
        camera::{Camera, CameraSample, Film},
        math::matrix::identity4,
        ray::Ray,
        test::ApproxEq,
    };
    use cgmath::{Matrix4, Point2, Point3, Rad, Vector3};
    use std::f32::consts::{PI, SQRT_2};

    #[test]
    fn through_center_of_canvas() {
        let film = Film::new(201, 101);
        let camera = Camera::new(film, Rad(PI / 2.0), identity4());
        let (ray, _) = camera.generate_ray(&CameraSample::at_pixel_center(Point2::new(100, 50)));
        assert!(ray.approx_eq(&Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0)
        )));
    }

    #[test]
    fn through_corner_of_canvas() {
        let film = Film::new(201, 101);
        let camera = Camera::new(film, Rad(PI / 2.0), identity4());
        let (ray, _) = camera.generate_ray(&CameraSample::at_pixel_center(Point2::new(0, 0)));
        assert!(ray.approx_eq(&Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.66519, 0.33259, -0.66851),
        )));
    }

    #[test]
    fn for_transformed_camera() {
        let transform = Matrix4::from_angle_y(Rad(PI / 4.0))
            * Matrix4::from_translation(Vector3::new(0.0, -2.0, 5.0));
        let film = Film::new(201, 101);
        let camera = Camera::new(film, Rad(PI / 2.0), transform);
        let (ray, _) = camera.generate_ray(&CameraSample::at_pixel_center(Point2::new(100, 50)));
        assert!(ray.approx_eq(&Ray::new(
            Point3::new(0.0, 2.0, -5.0),
            Vector3::new(SQRT_2 / 2.0, 0.0, SQRT_2 / -2.0),
        )));
    }
}
