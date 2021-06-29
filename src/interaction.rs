use crate::shape::Sphere;

#[derive(Debug)]
pub struct SurfaceInteraction<'shp> {
    pub shape: &'shp Sphere,
}

impl<'shp> PartialEq for SurfaceInteraction<'shp> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.shape, other.shape)
    }
}
