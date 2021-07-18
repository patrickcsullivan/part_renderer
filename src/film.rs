use cgmath::{Point2, Vector2};

use crate::{
    filter::{BoxFilter, Filter},
    geometry::bounds::Bounds2,
};

/// A 2D plane of pixels onto which a final image is rendered.
pub struct Film {
    /// The images resoltion in pixels.
    pub full_resolution: Vector2<f32>,

    /// Specifies a subset of the window to render in normal device coordinates.
    pub crop_window: Bounds2<f32>,

    /// Length of the diagonal of the film's physical area in meters.
    pub diagonal_m: f32,

    pub filter: Box<dyn Filter>,

    cropped_pixel_bounds: Bounds2<usize>,

    pixels: Vec<Pixel>,
}

impl Film {
    pub fn tmp_new(x: usize, y: usize) -> Self {
        Self::new(
            Vector2::new(x as f32, y as f32),
            Bounds2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)),
            0.0,
            Box::new(BoxFilter::new(1.0, 1.0)),
        )
    }

    pub fn new(
        full_resolution: Vector2<f32>,
        crop_window: Bounds2<f32>,
        diagonal_mm: f32,
        filter: Box<dyn Filter>,
    ) -> Self {
        let cropped_pixel_bounds = Bounds2::new(
            Point2::new(
                (full_resolution.x * crop_window.min.x).ceil() as usize,
                (full_resolution.y * crop_window.min.y).ceil() as usize,
            ),
            Point2::new(
                (full_resolution.x * crop_window.max.x).ceil() as usize,
                (full_resolution.y * crop_window.max.y).ceil() as usize,
            ),
        );
        let pixels = vec![Pixel::default(); cropped_pixel_bounds.area()];
        Self {
            full_resolution,
            crop_window,
            diagonal_m: diagonal_mm / 1000.0,
            filter,
            cropped_pixel_bounds,
            pixels,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Pixel {
    /// Running weight sum of spectral pixel contributions, in XYZ color
    /// coordinates.
    pub xyz: [f32; 3],

    // Sum of filter weight values for sample contributions to the pixel.
    pub filter_weight_sum: f32,

    /// Unweighted sum of sample splats.
    pub splat_xyz: [f32; 3],

    // Ensures the struct is 32 bytes wide instead of 28, meaning that it won't
    // straddle a cache line.
    _pad: f32,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            xyz: [0.0, 0.0, 0.0],
            filter_weight_sum: 0.0,
            splat_xyz: [0.0, 0.0, 0.0],
            _pad: 0.0,
        }
    }
}
