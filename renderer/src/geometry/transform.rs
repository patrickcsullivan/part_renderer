/// A trait representing an affine transformation that can be applied to data
/// structures containing points or vectors.
pub trait Transform<T> {
    fn transform(&self, t: &T) -> T;
}
