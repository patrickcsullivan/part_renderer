use crate::scene::Scene;

pub struct VisibilityTester {}

impl VisibilityTester {
    pub fn unocculuded(&self, scene: &Scene) -> bool {
        true // TODO
    }
}
