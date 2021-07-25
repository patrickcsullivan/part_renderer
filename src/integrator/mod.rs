mod original;
mod whitted;

pub use {original::OriginalRayTracer, whitted::WhittedRayTracer};

use crate::{
    camera::Camera, color::RgbSpectrum, film::FilmTile, filter::Filter, geometry::bounds::Bounds2,
    ray::Ray, sampler::Sampler, scene::Scene,
};
use cgmath::{point2, Point2};
use typed_arena::Arena;

pub trait RayTracer<'msh, 'mtrx, 'mtrl, S: Sampler> {
    /// Determine the incoming radiance that arrives along the ray at the ray
    /// origin.
    ///
    /// * `ray` - The ray along which incoming radiance is caluclated.
    /// * `scene` - The scene being rendered.
    /// * `sampler` - The sampler that is used to solve the light transport
    ///   equation using Monte Carlo integration.
    /// * `spectrum_arena` - An arena that will be used for efficient memory
    ///   allocation of temporary spectrums used in the incoming radiance
    ///   calculation.
    /// * `depth` - The number of ray bounces from the camera that have occured
    ///   up until the current call to this method.
    fn incoming_radiance(
        &self,
        // TODO: Change to ray differential.
        ray: &Ray,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
        max_depth: usize,
    ) -> RgbSpectrum;
}

/// * S - The type of sampler that is responsible for (1) choosing points on the image from
///   which rays are traced and (2) supplying sample positions used by the ray
///   tracer to estimate the value of the light transport integral.
/// * scene -
/// * camera - Controls how the scene is viewed and contains the `Film` onto
///   which the scene is rendered.
/// * filter -
pub fn render<'msh, 'mtrx, 'mtrl, S: Sampler>(
    scene: &Scene<'msh, 'mtrx, 'mtrl>,
    camera: Box<dyn Camera>,
    filter: Box<dyn Filter>,
    ray_tracer: Box<dyn RayTracer<'msh, 'mtrx, 'mtrl, S>>,
    max_depth: usize,
) {
    let image_sample_bounds = camera
        .film()
        .sample_bounds(filter.half_width(), filter.half_height());
    let (tile_count_x, tile_count_y) = tile_count(&image_sample_bounds);
    for ty in 0..tile_count_y {
        for tx in 0..tile_count_x {
            render_tile::<S>(
                &camera,
                scene,
                &image_sample_bounds,
                tx,
                ty,
                tile_count_x,
                &filter,
                &ray_tracer,
                max_depth,
            );
        }
    }
    // TODO: Merge film tiles returned by loop.
}

fn render_tile<'msh, 'mtrx, 'mtrl, S: Sampler>(
    camera: &Box<dyn Camera>,
    scene: &Scene<'msh, 'mtrx, 'mtrl>,
    image_sample_bounds: &Bounds2<i32>,
    tile_x_index: usize,
    tile_y_index: usize,
    tile_count_x: usize,
    filter: &Box<dyn Filter>,
    ray_tracer: &Box<dyn RayTracer<'msh, 'mtrx, 'mtrl, S>>,
    max_depth: usize,
) -> Option<FilmTile> {
    // If the sampler generates random numbers, we don't want samplers in
    // different tiles generating duplicate sequences of random numbers, so we
    // use the tile's row-major index as a unique seed.
    let seed = tile_y_index * tile_count_x + tile_x_index;
    let mut sampler = S::new(seed);

    let sample_bounds = tile_sample_bounds(image_sample_bounds, tile_x_index, tile_y_index);

    if let Some(mut tile) =
        camera
            .film()
            .tile(&sample_bounds, filter.half_width(), filter.half_height())
    {
        for pixel_min_corner in sample_bounds.range() {
            sampler.start_pixel(pixel_min_corner);
            loop {
                let sample = sampler.get_camera_sample(pixel_min_corner);
                let (ray, _differential, weight) = camera.generate_ray_differential(&sample);
                // TODO: Scale differential.

                let radiance = if weight > 0.0 {
                    // Recursive calls to `incoming_radiance` may need to
                    // allocate space for many different radiance spectrums.
                    // Rather than repeatedly allocating memory as needed,
                    // it's more efficient to pre-allocate in an arena.
                    // TODO: Confirm that this is actually more efficient.
                    let mut spectrum_arena = Arena::new();
                    ray_tracer.incoming_radiance(
                        &ray,
                        scene,
                        &mut sampler,
                        &mut spectrum_arena,
                        0,
                        max_depth,
                    )
                } else {
                    RgbSpectrum::black()
                };
                // TODO: Check for NaN or Inf values in spectrum.

                tile.add_sample(&sample.film_point, &radiance, weight, &filter);

                if !sampler.start_next_sample() {
                    break;
                }
            }
        }
        Some(tile)
    } else {
        None
    }
}

/// Return the number of 16-by-16 tiles into which the image's sample bounds
/// can be divided for parallelized rendering.
///
/// If a dimension of the sample bounds cannot be evenly divided by 16, then
/// the number of tiles in that dimension is rounded up so that the entire
/// sample bounds can be contained in the tiles.
fn tile_count(image_sample_bounds: &Bounds2<i32>) -> (usize, usize) {
    let sample_extent = image_sample_bounds.diagonal();
    const TILE_SIZE: usize = 16;
    (
        (sample_extent.x.min(0) as usize + TILE_SIZE - 1) / TILE_SIZE,
        (sample_extent.y.min(0) as usize + TILE_SIZE - 1) / TILE_SIZE,
    )
}

fn tile_sample_bounds(
    image_sample_bounds: &Bounds2<i32>,
    tile_x_index: usize,
    tile_y_index: usize,
) -> Bounds2<i32> {
    const TILE_SIZE: usize = 16;
    let min = Point2::new(
        image_sample_bounds.min.x + (tile_x_index * TILE_SIZE) as i32,
        image_sample_bounds.min.y + (tile_y_index * TILE_SIZE) as i32,
    );
    let max = Point2::new(
        // Tiles on the bottom and right edges might extend beyond the image
        // sample bounds, so be sure to limit the tile sample bounds to the
        // image sample bounds.
        (min.x + TILE_SIZE as i32).min(image_sample_bounds.max.x),
        (min.y + TILE_SIZE as i32).min(image_sample_bounds.max.y),
    );
    Bounds2::new(min, max)
}

/// A tile in an image's sample bounds that can be rendered in parallel with
/// other tiles.
struct Tile {
    sample_bounds: Bounds2<i32>,

    /// The index of the tile in a vector represeting a row-major grid of tiles.
    ///
    /// This index is mostly useful as a unique ID for each tile. We will be
    /// able to use this as a unique pseudo-random number generator seed for
    /// each tile.
    row_major_index: usize,
}

impl Tile {
    fn from_image_sample_bounds(image_sample_bounds: &Bounds2<i32>) -> Vec<Tile> {
        const TILE_SIZE: usize = 16;
        let image_sample_extent = image_sample_bounds.diagonal();
        let tile_count_x = (image_sample_extent.x as usize + TILE_SIZE - 1) / TILE_SIZE;
        let tile_count_y = (image_sample_extent.y as usize + TILE_SIZE - 1) / TILE_SIZE;

        let xs = 0..tile_count_x;
        let ys = 0..tile_count_y;
        ys.flat_map(|y| xs.clone().map(move |x| (x, y)))
            .map(|(x, y)| Tile {
                sample_bounds: Self::tile_sample_bounds(image_sample_bounds, x, y),
                row_major_index: y * tile_count_x + x,
            })
            .collect()
    }

    fn tile_sample_bounds(
        image_sample_bounds: &Bounds2<i32>,
        tile_x_index: usize,
        tile_y_index: usize,
    ) -> Bounds2<i32> {
        const TILE_SIZE: usize = 16;
        let min = Point2::new(
            image_sample_bounds.min.x + (tile_x_index * TILE_SIZE) as i32,
            image_sample_bounds.min.y + (tile_y_index * TILE_SIZE) as i32,
        );
        let max = Point2::new(
            // Tiles on the bottom and right edges might extend beyond the image
            // sample bounds, so be sure to limit the tile sample bounds to the
            // image sample bounds.
            (min.x + TILE_SIZE as i32).min(image_sample_bounds.max.x),
            (min.y + TILE_SIZE as i32).min(image_sample_bounds.max.y),
        );
        Bounds2::new(min, max)
    }
}
