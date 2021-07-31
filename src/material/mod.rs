use crate::{bsdf::Bsdf, interaction::SurfaceInteraction};

/// Indicates whether a surface interaction was found along a path starting from
/// a camera or a path starting from a light source.
#[derive(Debug, Clone, Copy)]
pub enum TransportMode {
    Camera,
    LightSource,
}

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
        transport_mode: TransportMode,
        allow_multiple_lobes: bool,
    ) -> Bsdf {
        todo!()
    }
}
