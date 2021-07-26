mod constant;
mod pixel;
mod state;

pub use {constant::ConstantSampler, pixel::PixelSampler};

use crate::camera::CameraSample;
use cgmath::Point2;
use core::f32;

/// A sampler is responsible for generating sequences of n-dimensional sample
/// vectors, where each element in a sample vector is in the range [0, 1).
///
/// The sampler will generate up to a fixed number of sample vectors per pixel.
///
/// In general the first five dimensions of a sample vector will be used by a
/// camera to create a ray. They are used to set, respectively, the (x, y)
/// position on the film, the time, and the (u, v) position on the lens.
pub trait Sampler {
    /// Create a new sampler with the given seed.
    ///
    /// The returned sampler should have the same state that the source sampler
    /// had when it was initialized, with the exception that the clone will use
    /// the given seed to initialize its pseudo-random number generator if it
    /// has one.
    ///
    /// * seed - Samplers that use a pseudo-random number generator will use
    ///   this seed to initialize the generator. Other samplers will ignore it.
    fn clone_with_seed(&self, seed: usize) -> Self;

    /// Return the number of n-dimensional sample vectors that will be generated
    /// for each pixel in the image.
    fn samples_per_pixel(&self) -> usize;

    /// Start sampling work on a given pixel. All subseqent requests to the
    /// sampler will generate samples for the given pixel, up until
    /// `start_pixel` is called again with a different pixel.
    ///
    /// * pixel - A point identifying the pixel. We can think of this point
    ///   either as the x and y indices of the pixel or as the raster space
    ///   coordinates of the top-left corner of the pixel. Both representations
    ///   are equivalent.
    fn start_pixel(&mut self, pixel: Point2<i32>);

    /// Get a 1D value for the next dimension of the current sample vector.
    ///
    /// This method mutates the sampler by incrementing the current sample
    /// dimension by one.
    fn get_1d(&mut self) -> f32;

    /// Get a 2D value for the next two dimensions of the current sample vector.
    ///
    /// This method mutates the sampler by incrementing the current sample
    /// dimension by two.
    fn get_2d(&mut self) -> Point2<f32>;

    /// Create a camera sample for the given pixel.
    ///
    /// * pixel - A point identifying the pixel. We can think of this point
    ///   either as the x and y indices of the pixel or as the raster space
    ///   coordinates of the top-left corner of the pixel. Both representations
    ///   are equivalent.
    fn get_camera_sample(&mut self, pixel: Point2<i32>) -> CameraSample {
        // Recall that each component of `film_sample` will be in [0, 1). Since
        // `pixel` refers to the pixels' top-left corner at coordinates (x,y),
        // the x and y components of `film_point` will be in [x, x+1) and [y,
        // y+1), respectively.
        let film_sample = self.get_2d();
        let film_point = Point2::new(
            pixel.x as f32 + film_sample.x,
            pixel.y as f32 + film_sample.y,
        );
        let time = self.get_1d();
        let lens_point = self.get_2d();
        CameraSample {
            film_point,
            time,
            lens_point,
        }
    }

    /// Tell the sampler to start working on the next sample for the current
    /// pixel. This method mutates the sampler by updating the current sample
    /// index and by reseting the current dimension to the first dimension.
    ///
    /// This method returns `true` if the number of generated samples is less
    /// than `samples_per_pixel`, indicating that the next sample can be
    /// generated. It returns `false` otherwise.
    fn start_next_sample(&mut self) -> bool;

    // /// Tell the sampler to start working on the sample with the given index for
    // /// the current pixel. This method mutates the sampler by updating the
    // /// current sample index and by reseting the current dimension to the first
    // /// dimension.
    // ///
    // /// This method returns `true` if the number of `sample_index` is less than
    // /// `samples_per_pixel`, indicating that the next sample can be generated.
    // /// It returns `false` otherwise.
    // fn start_nth_sample(&mut self, sample_index: usize) -> bool;

    // /// Inform the sampler that we will want to request a vector of 1D values of
    // /// length `count` for the current pixel. Each element in the vector is a 1D
    // /// value from a separate sample.
    // ///
    // /// This tells the sampler to do the preprocessing necessary for it to
    // /// return the vector of values in an efficient manner. This method must be
    // /// called before rendering begins for the current pixel and before the
    // /// samples are actually requested using `get_1d_array`.
    // fn prepare_1d_array(&mut self, count: usize);

    // /// Inform the sampler that we will want to request a vector of 2D values of
    // /// length `count` for the current pixel. Each element in the vector is a 2D
    // /// value from a separate sample.
    // ///
    // /// This tells the sampler to do the preprocessing necessary for it to
    // /// return the vector of values in an efficient manner. This method must be
    // /// called before rendering begins for the current pixel and before the
    // /// samples are actually requested using `get_2d_array`.
    // fn prepare_2d_array(&mut self, count: usize);

    // /// When generating a vector of values for a sample vector, a sampler might
    // /// be more efficient at generating value vectors of certain lengths. This
    // /// method takes an ideal value count and returns a value count that is
    // /// close and that the sampler can generate efficiently.
    // fn round_count(&self, ideal_count: usize) -> usize {
    //     ideal_count
    // }

    // /// Get a vector of 1D values for the next dimension. Each element in the
    // /// vector is a 1D value from a separate sample.
    // ///
    // /// This method mutates the sampler by incrementing the current sample
    // /// dimension by one.
    // ///
    // /// This must be called after `prepare_1d_array`. The returned vector will
    // /// contain the number of values that is specified in the call to
    // /// `prepare_1d_array`.
    // fn get_1d_vec(&mut self) -> Option<Vec<f32>>;

    // /// Get a vector of 2D values for the next two dimensions. Each element in
    // /// the vector is a 2D value from a separate sample.
    // ///
    // /// This method mutates the sampler by incrementing the current sample
    // /// dimension by two.
    // ///
    // /// This must be called after `prepare_2d_array`. The returned vector will
    // /// contain the number of values that is specified in the call to
    // /// `prepare_2d_array`.
    // fn get_2d_vec(&mut self) -> Option<Vec<Point2<f32>>>;
}
