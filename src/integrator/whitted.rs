use crate::{
    camera::Camera, color::RgbSpectrum, filter::Filter, geometry::bounds::Bounds2,
    interaction::SurfaceInteraction, ray::Ray, sampler::Sampler, scene::Scene,
};
use cgmath::{point2, InnerSpace, Point2};
use typed_arena::Arena;

/// An ray tracer based on Whitted's ray tracing algorithm. This can accurately
/// compute reflected and transmitted light from specular surfaces like glass,
/// mirrors, and water. It does not account for indirect lighting effects.
pub struct WhittedIntegrator<S> {
    max_depth: usize,

    /// Controls how the scene is viewed and contains the `Film` onto which the
    /// scene is rendered.
    camera: Box<dyn Camera>,

    filter: Box<dyn Filter>,

    /// A sampler that is responsible for (1) choosing points on the image from
    /// which rays are traced and (2) supplying sample positions used by the ray
    /// tracer to estimate the value of the light transport integral.
    // TODO: Maybe just get rid of phantom data and use a concrete Sampler once
    // one has been implemented.
    _marker: std::marker::PhantomData<S>,
}

impl<'msh, 'mtrx, 'mtrl, S: Sampler> WhittedIntegrator<S> {
    pub fn new(camera: Box<dyn Camera>, filter: Box<dyn Filter>) -> Self {
        Self {
            max_depth: 5,
            camera,
            filter,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn render(&mut self, scene: &Scene<'msh, 'mtrx, 'mtrl>) {
        let image_sample_bounds = self
            .camera
            .film()
            .sample_bounds(self.filter.half_width(), self.filter.half_height());
        let (tile_count_x, tile_count_y) = Self::tile_count(&image_sample_bounds);
        for ty in 0..tile_count_y {
            for tx in 0..tile_count_x {
                Self::render_tile(
                    &self.camera,
                    scene,
                    &image_sample_bounds,
                    tx,
                    ty,
                    tile_count_x,
                    self.max_depth,
                );
            }
        }
        // TODO: Merge film tiles returned by loop.
    }

    fn render_tile(
        camera: &Box<dyn Camera>,
        scene: &Scene<'msh, 'mtrx, 'mtrl>,
        image_sample_bounds: &Bounds2<usize>,
        tile_x_index: usize,
        tile_y_index: usize,
        tile_count_x: usize,
        max_depth: usize,
    ) {
        // Generate a unique seed for each tile. If the sampler generates random
        // numbers, we don't want samplers in different tiles generating
        // duplicate sequences of random numbers.
        let seed = tile_y_index * tile_count_x + tile_x_index;
        let mut sampler = S::new(seed);

        let tile_sample_bounds =
            Self::tile_sample_bounds(image_sample_bounds, tile_x_index, tile_y_index);

        // TODO: Create a "film tile".

        for y in tile_sample_bounds.min.y..tile_sample_bounds.max.y {
            for x in tile_sample_bounds.min.x..tile_sample_bounds.max.x {
                let pixel_min_corner = point2(x, y);
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
                        // TODO: Confirm that this is more efficient.
                        let mut spectrum_arena = Arena::new();
                        Self::incoming_radiance(
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

                    // TODO: Add camera ray's contribution to image (film tile).
                    if !sampler.start_next_sample() {
                        break;
                    }
                }
            }
        }

        // TODO: Return film tile.
        todo!()
    }

    /// Return the number of 16-by-16 tiles into which the image's sample bounds
    /// can be divided for parallelized rendering.
    ///
    /// If a dimension of the sample bounds cannot be evenly divided by 16, then
    /// the number of tiles in that dimension is rounded up so that the entire
    /// sample bounds can be contained in the tiles.
    fn tile_count(image_sample_bounds: &Bounds2<usize>) -> (usize, usize) {
        let sample_extent = image_sample_bounds.diagonal();
        const TILE_SIZE: usize = 16;
        (
            (sample_extent.x + TILE_SIZE - 1) / TILE_SIZE,
            (sample_extent.y + TILE_SIZE - 1) / TILE_SIZE,
        )
    }

    fn tile_sample_bounds(
        image_sample_bounds: &Bounds2<usize>,
        tile_x_index: usize,
        tile_y_index: usize,
    ) -> Bounds2<usize> {
        const TILE_SIZE: usize = 16;
        let min = Point2::new(
            image_sample_bounds.min.x + tile_x_index * TILE_SIZE,
            image_sample_bounds.min.y + tile_y_index * TILE_SIZE,
        );
        let max = Point2::new(
            // Tiles on the bottom and right edges might extend beyond the image
            // sample bounds, so be sure to limit the tile sample bounds to the
            // image sample bounds.
            (min.x + TILE_SIZE).min(image_sample_bounds.max.x),
            (min.y + TILE_SIZE).min(image_sample_bounds.max.y),
        );
        Bounds2::new(min, max)
    }

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
        // TODO: Change to ray differential.
        ray: &Ray,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
        max_depth: usize,
    ) -> RgbSpectrum {
        if let Some((_t, _prim, interaction)) = scene.ray_intersection(ray) {
            // We will calculate the outgoing radiance along the ray at the
            // surface. Since we ignore all particpating media (like smoke or
            // fog), the outgoing radiance at the intersected surface will equal
            // the incoming radiance at the ray origin.
            let mut outgoing_radiance = RgbSpectrum::constant(0.0);

            // Initialize the normal and outgoing direction of light at the
            // surface.
            let normal = interaction.shading_geometry.normal;
            let point_to_ray_origin_direction = interaction.neg_ray_direction;

            // Compute scattering functions for surface interaction.
            interaction.compute_scattering_functions(ray, spectrum_arena);

            // Compute emitted light if ray hit an area light source.
            outgoing_radiance += interaction.emitted_radiance(&point_to_ray_origin_direction);

            // Add the contribution of each light source.
            for light in &scene.lights {
                let sample = sampler.get_2d();
                let (radiance_from_light, point_to_light_direction, pdf, visibility) =
                    light.sample_incoming_radiance_at_surface(&interaction, sample);
                if radiance_from_light.is_black() || pdf == 0.0 {
                    continue;
                }
                let f = interaction.bsdf(&point_to_light_direction, &point_to_ray_origin_direction);
                if !f.is_black() && visibility.unocculuded(scene) {
                    outgoing_radiance += f
                        * radiance_from_light
                        * (point_to_light_direction.dot(normal).abs() / pdf);
                }
            }

            if depth + 1 < max_depth {
                // Trace rays for specular reflection and refraction.
            }

            outgoing_radiance
        } else {
            let mut ray_origin_incoming_radiance = RgbSpectrum::constant(0.0);
            for light in &scene.lights {
                ray_origin_incoming_radiance += light.outgoing_radiance_onto_ray(ray);
            }
            ray_origin_incoming_radiance
        }
    }

    fn specular_reflect(
        &self,
        ray: &Ray,
        interaction: &SurfaceInteraction,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        todo!()
    }

    fn specular_transmit(
        &self,
        ray: &Ray,
        interaction: &SurfaceInteraction,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        todo!()
    }
}

/// A tile in an image's sample bounds that can be rendered in parallel with
/// other tiles.
struct Tile {
    sample_bounds: Bounds2<usize>,

    /// The index of the tile in a vector represeting a row-major grid of tiles.
    ///
    /// This index is mostly useful as a unique ID for each tile. We will be
    /// able to use this as a unique pseudo-random number generator seed for
    /// each tile.
    row_major_index: usize,
}

impl Tile {
    fn from_image_sample_bounds(image_sample_bounds: &Bounds2<usize>) -> Vec<Tile> {
        const TILE_SIZE: usize = 16;
        let image_sample_extent = image_sample_bounds.diagonal();
        let tile_count_x = (image_sample_extent.x + TILE_SIZE - 1) / TILE_SIZE;
        let tile_count_y = (image_sample_extent.y + TILE_SIZE - 1) / TILE_SIZE;

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
        image_sample_bounds: &Bounds2<usize>,
        tile_x_index: usize,
        tile_y_index: usize,
    ) -> Bounds2<usize> {
        const TILE_SIZE: usize = 16;
        let min = Point2::new(
            image_sample_bounds.min.x + tile_x_index * TILE_SIZE,
            image_sample_bounds.min.y + tile_y_index * TILE_SIZE,
        );
        let max = Point2::new(
            // Tiles on the bottom and right edges might extend beyond the image
            // sample bounds, so be sure to limit the tile sample bounds to the
            // image sample bounds.
            (min.x + TILE_SIZE).min(image_sample_bounds.max.x),
            (min.y + TILE_SIZE).min(image_sample_bounds.max.y),
        );
        Bounds2::new(min, max)
    }
}
