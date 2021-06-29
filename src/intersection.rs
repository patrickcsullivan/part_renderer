use crate::interaction::SurfaceInteraction;

#[derive(Debug, PartialEq)]
pub struct Intersection<'shp> {
    pub t: f32,
    pub interaction: SurfaceInteraction<'shp>,
}
