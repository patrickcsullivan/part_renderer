use crate::shape::Sphere;

#[derive(Debug)]
pub struct SurfaceInteraction<'shp, 'mat> {
    // TODO: See polymorphic-shape branch for how to use Shape trait obj.
    /// The shape that the point lies on.
    pub shape: &'shp Sphere<'mat>,
}

impl<'shp, 'mat> PartialEq for SurfaceInteraction<'shp, 'mat> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.shape, other.shape)
    }
}
