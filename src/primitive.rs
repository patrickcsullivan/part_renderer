use crate::{material::Material, object::Object};

#[derive(Debug)]
pub struct Primitive<'msh, 'shp, 'mtrx, 'mtrl> {
    pub object: &'shp Object<'msh, 'mtrx>,
    pub material: &'mtrl Material,
}

impl<'msh, 'shp, 'mtrx, 'mtrl> Primitive<'msh, 'shp, 'mtrx, 'mtrl> {
    pub fn new(object: &'shp Object<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
        Self { object, material }
    }
}
