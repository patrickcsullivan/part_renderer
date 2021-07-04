use cgmath::{
    Angle, InnerSpace, Matrix4, PerspectiveFov, Point3, Rad, Transform, Vector3, Vector4,
};

use crate::ray::Ray;

/// A camera that is used to view a scene.
///
/// The camera sits at the origin of camera space and renders images onto a
/// canvas one unit away.
pub struct Camera {
    /// Canvas's width in pixels.
    pub width: u32,

    /// Canvas's height in pixels.
    pub height: u32,

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
    fn new(width: u32, height: u32, fov: Rad<f32>, world_to_camera: Matrix4<f32>) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = width as f32 / height as f32;
        let (half_width, half_height) = if aspect >= 1.0 {
            // horizontal
            (half_view, half_view / aspect)
        } else {
            // vertical
            (half_view * aspect, half_view)
        };
        let pixel_size = half_width * 2.0 / width as f32;

        Self {
            width,
            height,
            fov,
            world_to_camera,
            half_width,
            half_height,
            pixel_size,
        }
    }

    /// Returns a ray that starts at the camera and passes through the specified
    /// pixel on the canvas.
    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        // Offset from canvase edge to center of pixel.
        let x_offset = (px as f32 + 0.5) * self.pixel_size;
        let y_offset = (py as f32 + 0.5) * self.pixel_size;

        // Compute the pixel'ss position in camera space. By default camera looks
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

        Ray::new(ray_origin_ws, ray_direction_ws)
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
    use crate::{camera::view_transform, matrix::identity4, test::ApproxEq};
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
    use crate::{camera::Camera, matrix::identity4, test::ApproxEq};
    use cgmath::{Angle, Deg, Rad};
    use std::f32::consts::PI;

    #[test]
    fn for_horizontal_canvas() {
        let camera = Camera::new(200, 125, Rad(PI / 2.0), identity4());
        assert!(camera.pixel_size.approx_eq(&0.01));
    }

    #[test]
    fn for_vertical_canvas() {
        let camera = Camera::new(125, 200, Rad(PI / 2.0), identity4());
        assert!(camera.pixel_size.approx_eq(&0.01));
    }
}

#[cfg(test)]
mod ray_for_pixel_tests {
    use crate::{camera::Camera, matrix::identity4, ray::Ray, test::ApproxEq};
    use cgmath::{Matrix4, Point3, Rad, Vector3};
    use std::f32::consts::{PI, SQRT_2};

    #[test]
    fn through_center_of_canvas() {
        let camera = Camera::new(201, 101, Rad(PI / 2.0), identity4());
        let ray = camera.ray_for_pixel(100, 50);
        assert!(ray.approx_eq(&Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0)
        )));
    }

    #[test]
    fn through_corner_of_canvas() {
        let camera = Camera::new(201, 101, Rad(PI / 2.0), identity4());
        let ray = camera.ray_for_pixel(0, 0);
        assert!(ray.approx_eq(&Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.66519, 0.33259, -0.66851),
        )));
    }

    #[test]
    fn for_transformed_camera() {
        let transform = Matrix4::from_angle_y(Rad(PI / 4.0))
            * Matrix4::from_translation(Vector3::new(0.0, -2.0, 5.0));
        let camera = Camera::new(201, 101, Rad(PI / 2.0), transform);
        let ray = camera.ray_for_pixel(100, 50);
        assert!(ray.approx_eq(&Ray::new(
            Point3::new(0.0, 2.0, -5.0),
            Vector3::new(SQRT_2 / 2.0, 0.0, SQRT_2 / -2.0),
        )));
    }
}
