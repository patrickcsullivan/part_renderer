mod whitted;

pub use whitted::WhittedRayTracer;

use crate::{
    camera::Camera,
    color::RgbaSpectrum,
    film::{Film, FilmTile},
    filter::Filter,
    geometry::bounds::Bounds2,
    ray::Ray,
    sampler::IncrementalSampler,
    // scene::Scene,
};
use cgmath::{point2, Point2, Zero};
use rayon::prelude::*;

pub trait RayTracer<Scene, Sampler: IncrementalSampler> {
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
        sampler: &mut Sampler,
        depth: usize,
        max_depth: usize,
    ) -> RgbaSpectrum;
}

/// * S - The type of sampler that is responsible for (1) choosing points on the image from
///   which rays are traced and (2) supplying sample positions used by the ray
///   tracer to estimate the value of the light transport integral.
/// * scene -
/// * camera - Controls how the scene is viewed and contains the `Film` onto
///   which the scene is rendered.
/// * filter -
pub fn render<Scene: Send + Sync, Sampler: IncrementalSampler + Send + Sync>(
    scene: &Scene,
    camera: &(dyn Camera + Send + Sync),
    film: &mut Film,
    filter: &(dyn Filter + Send + Sync),
    sampler: &Sampler,
    ray_tracer: &(dyn RayTracer<Scene, Sampler> + Send + Sync),
    max_depth: usize,
) {
    let image_sample_bounds = film.sample_bounds(filter.half_width(), filter.half_height());

    let film_tiles: Vec<FilmTile> = Tile::span_image_sample_bounds(&image_sample_bounds)
        .par_iter()
        .filter_map(|tile| {
            // If the sampler generates random numbers, we don't want samplers in
            // different tiles generating duplicate sequences of random numbers, so we
            // use the tile's row-major index as a unique seed.
            let mut sampler = sampler.clone_with_seed(tile.row_major_index as u64);
            render_tile::<Scene, Sampler>(
                camera,
                film,
                scene,
                tile,
                filter,
                &mut sampler,
                ray_tracer,
                max_depth,
            )
        })
        .collect();

    for ft in film_tiles {
        film.merge_tile(&ft);
    }
}

fn render_tile<Scene, Sampler: IncrementalSampler>(
    camera: &dyn Camera,
    film: &Film,
    scene: &Scene,
    tile: &Tile,
    filter: &dyn Filter,
    sampler: &mut Sampler,
    ray_tracer: &dyn RayTracer<Scene, Sampler>,
    max_depth: usize,
) -> Option<FilmTile> {
    let sample_bounds = tile.sample_bounds;

    if let Some(mut film_tile) =
        film.tile(&sample_bounds, filter.half_width(), filter.half_height())
    {
        for pixel_min_corner in sample_bounds.range() {
            let mut sample_count = 0;
            sampler.start_pixel(pixel_min_corner);
            loop {
                let sample = sampler.get_camera_sample(pixel_min_corner);
                let (ray, _differential, weight) = camera.generate_ray_differential(&sample);
                // TODO: Scale differential.

                let radiance = if weight > 0.0 {
                    ray_tracer.incoming_radiance(&ray, scene, sampler, 0, max_depth)
                } else {
                    RgbaSpectrum::transparent()
                };

                // println!(
                //     "At ({}, {})\tsample {}\tradiance {}",
                //     pixel_min_corner.x,
                //     pixel_min_corner.y,
                //     sample_count,
                //     radiance.a()
                // );

                // TODO: Check for NaN or Inf values in spectrum.

                film_tile.add_sample(&sample.film_point, &radiance, weight, filter);

                sample_count += 1;
                if !sampler.start_next_sample() {
                    break;
                }
            }
        }
        Some(film_tile)
    } else {
        None
    }
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
    /// Return a vector of 16-by-16 tiles that span the given image sample
    /// bounds.
    pub fn span_image_sample_bounds(image_sample_bounds: &Bounds2<i32>) -> Vec<Tile> {
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
