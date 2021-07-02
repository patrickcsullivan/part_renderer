use crate::shape::Shape;

#[derive(Debug)]
pub struct SurfaceInteraction<'shp, 'mat> {
    /// The shape that the point lies on.
    pub shape: Box<dyn Shape<'shp, 'mat> + 'shp>,
}
