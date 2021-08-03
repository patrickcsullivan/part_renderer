use crate::bsdf::{LambertianDiffuseReflection, OrenNayarDiffuseReflection};
use crate::color::RgbSpectrum;
use crate::TransportMode;
use crate::{bsdf::Bsdf, interaction::SurfaceInteraction};

/// Describes the material properties of a surface. For any given point on a
/// surface, a material can return a bidirectional scattering distribution
/// function (BSDF) for that point.
pub trait Material {
    /// Determine the reflective properties at the given surface interaction
    /// point and return the bidirectional scattering distribution function
    /// (BSDF) for that point. If the material includes subsurface scattering
    /// then a bidirectional scattering surface reflectance distribution
    /// function (BSSRDF) is returned as well.
    ///
    /// * interaction -
    /// * transport_mode -
    /// * allow_multiple_lobes - Indicates whether the material should use BxDFs
    ///   that aggregate multiple types of scattering into a single BxDF when
    ///   such BxDFs are available. Setting this to `true` can improve results
    ///   when used with Monte Carlo light transport algorithms but can
    ///   introduce noise when used with direct light or Whitted integrators.
    fn scattering_functions(
        &self,
        interaction: &SurfaceInteraction,
        // transport_mode: TransportMode,
        // allow_multiple_lobes: bool,
    ) -> Bsdf;
}

/// A purely diffuse surface.
pub struct MatteMaterial {
    /// Spectral diffuse reflection.
    kd: RgbSpectrum,

    /// Roughness. The standard deviation of microfacet orientation angle in
    /// radians.
    sigma: f32,
}

impl MatteMaterial {
    pub fn new(kd: RgbSpectrum, sigma: f32) -> Self {
        Self { kd, sigma }
    }
}

impl Material for MatteMaterial {
    fn scattering_functions(
        &self,
        interaction: &SurfaceInteraction,
        // transport_mode: TransportMode,
        // allow_multiple_lobes: bool,
    ) -> Bsdf {
        let mut bsdf = Bsdf::new(interaction);
        if self.sigma == 0.0 {
            bsdf.add(Box::new(LambertianDiffuseReflection::new(self.kd)));
        } else {
            bsdf.add(Box::new(OrenNayarDiffuseReflection::new(
                self.kd, self.sigma,
            )));
        }
        bsdf
    }
}

/// A purely diffuse surface.
pub struct PlasticMaterial {
    /// Diffuse reflection.
    kd: RgbSpectrum,

    /// Glossy specular reflection
    ks: RgbSpectrum,

    roughness: f32,

    remap_roughness: bool,
}

// impl PlasticMaterial {
//     pub fn new(kd: RgbSpectrum, ks: RgbSpectrum, roughness: f32) -> Self {
//         Self {
//             kd,
//             ks,
//             roughness,
//         }
//     }
// }

// impl Material for PlasticMaterial {
//     fn scattering_functions(
//         &self,
//         interaction: &SurfaceInteraction,
//         // transport_mode: TransportMode,
//         // allow_multiple_lobes: bool,
//     ) -> Bsdf {
//         let mut bsdf = Bsdf::new(interaction);

//         if !self.kd.is_black() {
//             bsdf.add(Box::new(LambertianDiffuseReflection::new(self.kd)));
//         }

//         if !self.ks.is_black() {

//         }

//         bsdf
//     }
// }
