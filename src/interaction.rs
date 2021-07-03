use crate::shape::Sphere;

#[derive(Debug)]
pub struct SurfaceInteraction<'shp, 'mtrx, 'mtrl> {
    /// The shape that the point lies on.
    pub shape: &'shp Sphere<'mtrx, 'mtrl>,
}
