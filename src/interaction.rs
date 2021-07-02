use crate::shape::Shape;

#[derive(Debug)]
pub struct SurfaceInteraction<'shp, 'mtrx, 'mtrl> {
    /// The shape that the point lies on.
    pub shape: Box<dyn Shape<'shp, 'mtrx, 'mtrl> + 'shp>,
}
