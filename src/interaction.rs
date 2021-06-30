use crate::shape::Sphere;

#[derive(Debug)]
pub struct SurfaceInteraction<'shp, 'mat> {
    pub shape: &'shp Sphere<'mat>,
}

impl<'shp, 'mat> PartialEq for SurfaceInteraction<'shp, 'mat> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.shape, other.shape)
    }
}
