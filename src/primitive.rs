use crate::{material::Material, shape::Object};

#[derive(Debug)]
pub struct Primitive<'shp, 'mtrx, 'mtrl> {
    pub object: &'shp Object<'mtrx>,
    pub material: &'mtrl Material,
}

impl<'shp, 'mtrx, 'mtrl> Primitive<'shp, 'mtrx, 'mtrl> {
    pub fn new(object: &'shp Object<'mtrx>, material: &'mtrl Material) -> Self {
        Self { object, material }
    }
}
