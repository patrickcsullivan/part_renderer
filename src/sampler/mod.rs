use cgmath::Point2;

use crate::camera::CameraSample;

/// Defines the ability to generate multi-dimensional sample vectors for pixels.
pub trait Sampler {
    /// The number of samples that will be generated for each pixel in the final
    /// image.
    fn samples_per_pixel(&self) -> usize;

    /// Start sampling work on a given pixel. All subseqent requests to the
    /// sampler will generate samples for the given pixel, up until
    /// `start_pixel` is called again with a different pixel.
    fn start_pixel(pixel: Point2<usize>);

    /// Get a 1D sample for the next dimension of the sample vector. This method
    /// mutates the sampler by incrementing the current sample dimension by one.
    fn get_1d(&mut self) -> f32;

    /// Get a 2D sample for the next two dimensions of the current sample
    /// vector. This method mutates the sampler by incrementing the current
    /// sample dimension by two.
    fn get_2d(&mut self) -> Point2<f32>;

    /// Create a camera sample for the given pixel.
    fn get_camera_sample(&mut self, raster_point: Point2<usize>) -> CameraSample {
        // TODO: Why doesn't p_film need +0.5? Does `get_2D()` account for that? Does the conversion to Point2<f32> do that?
        let film_sample = self.get_2d();
        let film_point = Point2::new(
            raster_point.x as f32 + film_sample.x,
            raster_point.y as f32 + film_sample.y,
        );
        let time = self.get_1d();
        let lens_point = self.get_2d();
        CameraSample {
            film_point,
            time,
            lens_point,
        }
    }

    /// Inform the sampler that we will want to request a vector of 1D samples
    /// that has a length of `sample_count`. This tells the sampler to do the
    /// preprocessing necessary for it to return the vector of samples in an
    /// efficient manner.
    ///
    /// This must be called before rendering begins and before the samples are
    /// actually requested using `get_1d_array`.
    fn prepare_1d_array(&mut self, sample_count: usize);

    /// Inform the sampler that we will want to request a vector of 2D samples
    /// that has a length of `sample_count`. This tells the sampler to do the
    /// preprocessing necessary for it to return the vector of samples in an
    /// efficient manner.
    ///
    /// This must be called before rendering begins and before the samples are
    /// actually requested using `get_2d_array`.
    fn prepare_2d_array(&mut self, sample_count: usize);

    /// When generating arrays of samples, a sampler might be more efficient at
    /// generating sample arrays of certain lengths. This method takes an ideal
    /// sample count and returns a sample count that is close and that the
    /// sampler can generate efficiently.
    fn round_count(&self, sample_count: usize) -> usize {
        sample_count
    }

    /// Get a vector 1D samples for the next dimension of the sample vector.
    /// This method mutates the sampler by incrementing the current sample
    /// dimension by one.
    ///
    /// This must be called after `prepare_1d_array`. The returned vector will
    /// contain the number of samples that is specified in the call to
    /// `prepare_1d_array`.
    fn get_1d_array(&mut self);

    /// Get a vector 2D samples for the next dimension of the sample vector.
    /// This method mutates the sampler by incrementing the current sample
    /// dimension by two.
    ///
    /// This must be called after `prepare_2d_array`. The returned vector will
    /// contain the number of samples that is specified in the call to
    /// `prepare_2d_array`.
    fn get_2d_array(&mut self);

    /// Tell the sampler to start working on the next sample for the current
    /// pixel. This method mutates the sampler by updating the current sample
    /// index and by reseting the current dimension to the first dimension.
    ///
    /// This method returns `true` if the number of generated samples is less
    /// than `samples_per_pixel`, indicating that the next sample can be
    /// generated. It returns `false` otherwise.
    fn start_next_sample(&mut self) -> bool;

    /// Tell the sampler to start working on the sample with the given index for
    /// the current pixel. This method mutates the sampler by updating the
    /// current sample index and by reseting the current dimension to the first
    /// dimension.
    ///
    /// This method returns `true` if the number of `sample_index` is less than
    /// `samples_per_pixel`, indicating that the next sample can be generated.
    /// It returns `false` otherwise.
    fn start_nth_sample(&mut self, sample_index: usize) -> bool;
}
