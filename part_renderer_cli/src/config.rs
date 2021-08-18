use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub output_path: String,
    pub width: usize,
    pub height: usize,
    pub crop: bool,
    pub sampler: Sampler,
    pub part: Part,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

/// A position in spherical coordinates.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Spherical {
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Deserialize)]
pub enum Sampler {
    StratifiedSampler {
        x_strata_count: usize,
        y_strata_count: usize,
        jitter: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Handedness {
    LeftHanded,
    RightHanded,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub stl_path: String,
    pub material: Material,

    /// Indicates whether the vertex positions in the mesh assume a right hand
    /// coordinate system or a left hand coordinate system.
    pub handedness: Handedness,
}

#[derive(Debug, Deserialize)]
pub struct Material {
    pub color: Rgb,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

#[derive(Debug, Deserialize)]
pub enum Light {
    /// A point light source that emits the same amount of light in all directions.
    PointLight {
        position: Spherical,

        /// The amount of power emitted per unit solid angle.
        intensity: Rgb,
    },
}

#[derive(Debug, Deserialize)]
pub enum Camera {
    OrthographicCamera {
        position: Spherical,

        /// Distance between the near clipping plane and the camera.
        z_near: f32,

        /// Distance between the far clipping plane and the camera.
        z_far: f32,
    },
    PerspectiveCamera {
        position: Spherical,

        /// Vertical field of view in degrees.
        fov_y: f32,
    },
}
