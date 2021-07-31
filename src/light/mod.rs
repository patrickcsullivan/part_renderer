mod point;
mod visibility;

pub use point::PointLight;
pub use visibility::VisibilityTester;

use bitflags::bitflags;
use cgmath::{Point2, Vector3};

use crate::{color::RgbSpectrum, interaction::SurfaceInteraction, scene::Scene};

pub trait Light {
    /// Given a surface interation containing a point and a time, return the
    /// radiance arriving at that point and time due to the light source,
    /// ignoring possible occlusion. In addition to incoming radiance, this
    /// method also returns the incident direction from the surface point to the
    /// light source, and a visibility tester.
    // TODO: Maybe rename to `incident_light`.
    fn li(&self, interaction: &SurfaceInteraction)
        -> (RgbSpectrum, Vector3<f32>, VisibilityTester);

    // TODO: See p. 716 for explanation.
    fn sample_li(
        &self,
        interaction: &SurfaceInteraction,
        _u: &Point2<f32>,
    ) -> (RgbSpectrum, Vector3<f32>, VisibilityTester, f32) {
        let (li, wi, vis) = self.li(interaction);
        (li, wi, vis, 1.0)
    }

    /// Return an approximation of the light's total emitted power.
    ///
    /// This is useful for light transport algorithms that will spend more time
    /// sampling and modeling lights that emit more power.
    fn power(&self) -> RgbSpectrum;

    /// Determine characteristics of the scene that could affect the light
    /// before rendering starts. This method should be called before reding
    /// begins.
    fn preprocess(&mut self, _scene: &Scene) {
        // Do nothing by default.
    }

    /// Returns the light flags that describe the type of light source.
    fn flags(&self) -> LightFlags;
}

bitflags! {
    /// A bit flag representing the different types of bidirectional relectance
    /// or transmittance distribution functions.
    pub struct LightFlags: u8 {
        /// The light source's position is represented by a delta distribution.
        /// (e.g., a point light)
        const DELTA_POSITION = 0b00000001;

        /// The light source's direction is represented by a delta distribution.
        /// (e.g., a directional light)
        const DELTA_DIRECTION = 0b00000010;

        const AREA = 0b00000100;

        const INFINITE = 0b00001000;
    }
}
