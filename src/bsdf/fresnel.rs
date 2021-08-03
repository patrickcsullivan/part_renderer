use super::{geometry, Bxdf, BxdfType};
use crate::{bsdf::geometry::abs_cos_theta, color::RgbSpectrum, TransportMode};
use cgmath::{vec3, Point2, Vector3};

/// A BRDF that models reflection off a perfectly specular surface, scattering
/// incident light in a single directions.
pub struct FresnelSpecularReflection {
    /// A spectrum that scales the reflected color.
    r: RgbSpectrum,

    /// The Fresnel properties at the surface.
    fresnel: Fresnel,
}

impl FresnelSpecularReflection {
    pub fn dielectric(r: RgbSpectrum, eta_i: f32, eta_t: f32) -> Self {
        Self {
            r,
            fresnel: Fresnel::Dielectric { eta_i, eta_t },
        }
    }

    pub fn conductor(
        r: RgbSpectrum,
        eta_i: RgbSpectrum,
        eta_t: RgbSpectrum,
        k: RgbSpectrum,
    ) -> Self {
        Self {
            r,
            fresnel: Fresnel::Conductor { eta_i, eta_t, k },
        }
    }
}

impl Bxdf for FresnelSpecularReflection {
    fn bxdf_type(&self) -> BxdfType {
        BxdfType::REFLECTION | BxdfType::SPECULAR
    }

    fn f(&self, _wo: &Vector3<f32>, _wi: &Vector3<f32>) -> RgbSpectrum {
        RgbSpectrum::black()
    }

    fn sample_f(
        &self,
        wo: &Vector3<f32>,
        _sample: Point2<f32>,
        _sampled_type: BxdfType,
    ) -> (Vector3<f32>, f32, RgbSpectrum) {
        let wi = geometry::reflect(wo);
        let pdf = 1.0;
        let light =
            self.fresnel.evaluate(geometry::cos_theta(&wi)) * self.r / geometry::abs_cos_theta(&wi);
        (wi, pdf, light)
    }
}

/// A BTDF that models transmission through a perfectly specular surface,
/// scattering incident light in a single directions.
pub struct FresnelSpecularTransmission {
    /// The index of refraction "above" the surface (in the direction that the
    /// normal points).
    eta_above: f32,

    /// The index of refraction "below" the surface (in the opposite direction
    /// that the normal points).
    eta_below: f32,

    /// Transmission scale factor.
    t: f32,

    transport_mode: TransportMode,

    /// The Fresnel properties at the surface.
    fresnel: Fresnel,
}

impl FresnelSpecularTransmission {
    pub fn dielectric(
        eta_above: f32,
        eta_below: f32,
        t: f32,
        transport_mode: TransportMode,
    ) -> Self {
        Self {
            eta_above,
            eta_below,
            t,
            transport_mode,
            fresnel: Fresnel::Dielectric {
                eta_i: eta_above,
                eta_t: eta_below,
            },
        }
    }
}

impl Bxdf for FresnelSpecularTransmission {
    fn bxdf_type(&self) -> BxdfType {
        BxdfType::TRANSMISSION | BxdfType::SPECULAR
    }

    fn f(&self, _wo: &Vector3<f32>, _wi: &Vector3<f32>) -> RgbSpectrum {
        RgbSpectrum::black()
    }

    fn sample_f(
        &self,
        wo: &Vector3<f32>,
        _sample: Point2<f32>,
        _sampled_type: BxdfType,
    ) -> (Vector3<f32>, f32, RgbSpectrum) {
        // Determine if the incident ray would be entering or exiting the
        // refractive medium.
        let wi_is_entering = geometry::cos_theta(wo) > 0.0;
        let (eta_incident, eta_transmitted) = if wi_is_entering {
            (self.eta_above, self.eta_below)
        } else {
            (self.eta_below, self.eta_above)
        };

        use crate::geometry::vector::face_forward;
        if let Some(wi) = geometry::refract(
            wo,
            &face_forward(vec3(0.0, 0.0, 1.0), *wo),
            eta_incident / eta_transmitted,
        ) {
            let pdf = 1.0;
            let mut ft = self.t
                * (RgbSpectrum::constant(1.0) - self.fresnel.evaluate(geometry::cos_theta(&wi)));

            // Account for non-symmetry with transmission to a different medium.
            if self.transport_mode == TransportMode::Radiance {
                ft *= (eta_incident * eta_incident) / (eta_transmitted * eta_transmitted);
            }

            let light = ft / abs_cos_theta(&wi);
            (wi, pdf, light)
        } else {
            // Total internal reflection occurs, so just return the reflected
            // vector even though nothing should use it since there's no
            // transmitted light.
            let wi = geometry::reflect(wo);
            let pdf = 1.0;
            let light = RgbSpectrum::black();
            (wi, pdf, light)
        }
    }
}

/// A description of the Fresnel properties at the boundry between two media.
enum Fresnel {
    /// A description of the boundry between two dielectric media.
    Dielectric {
        /// The index of refraction for the incident media.
        eta_i: f32,

        /// The index of refraction for the transmitted media.
        eta_t: f32,
    },

    /// A description of the boundry between an incident dielectric media and a
    /// conductive media.
    Conductor {
        /// The index of refraction across a spectrum for the incident media.
        eta_i: RgbSpectrum,

        /// The index of refraction across a spectrum for the transmitted media.
        eta_t: RgbSpectrum,

        /// Absorption coefficient.
        k: RgbSpectrum,
    },
}

impl Fresnel {
    /// Calculate the fraction of incoming light that is reflected or
    /// transmitted at the boundry.
    ///
    /// * cos_theta_i -  The cosine of theta for the incident light direction,
    ///   where theta is the angle from the vector to the z axis.
    pub fn evaluate(&self, cos_theta_i: f32) -> RgbSpectrum {
        match self {
            Fresnel::Dielectric { eta_i, eta_t } => {
                RgbSpectrum::constant(fresnel_dielectric(cos_theta_i, *eta_i, *eta_t))
            }
            Fresnel::Conductor { eta_i, eta_t, k } => {
                fresnel_conductor(cos_theta_i, eta_i, eta_t, k)
            }
        }
    }
}

/// Calculate the fraction of incoming light that is reflected or transmitted at
/// the boundry between two dielectric mediums.
///
/// * cos_theta_i -  The cosine of theta for the incident light direction, where
///   theta is the angle from the vector to the z axis.
/// * eta_i - The index of refraction for the incident media.
/// * eta_t - The index of refraction for the transmitted media.
fn fresnel_dielectric(cos_theta_i: f32, eta_i: f32, eta_t: f32) -> f32 {
    let cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);

    // Swap the indices of refraction if light is leaving the surface rather
    // than entering it.
    let is_entering = cos_theta_i > 0.0;
    let (eta_i, eta_t) = if is_entering {
        (eta_i, eta_t)
    } else {
        (eta_t, eta_i)
    };

    let cos_theta_i = cos_theta_i.abs();

    // Use Snell's law to calculate the cosine of theta for the transmitted
    // light direction.
    let sin_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0).sqrt();
    let sin_theta_t = eta_i / eta_t * sin_theta_i;
    if sin_theta_t >= 1.0 {
        // Total internal refraction occurs, so all light is reflected.
        return 1.0;
    }
    let cos_theta_t = (1.0 - sin_theta_t * sin_theta_t).max(0.0).sqrt();

    let refl_parallel = ((eta_t * cos_theta_i) - (eta_i * cos_theta_t))
        / ((eta_t * cos_theta_i) + (eta_i * cos_theta_t));
    let refl_perp = ((eta_i * cos_theta_i) - (eta_t * cos_theta_t))
        / ((eta_i * cos_theta_i) + (eta_t * cos_theta_t));
    (refl_parallel * refl_parallel + refl_perp * refl_perp) / 2.0
}

/// Calculate the fraction of incoming light that is reflected when light from a
/// dielectric medium interacts with the surface of a conductive medium.
///
/// * cos_theta_i -  The cosine of theta for the incident light direction, where
///   theta is the angle from the vector to the z axis.
/// * eta_i - The index of refraction across a spectrum for the incident media.
/// * eta_t - The index of refraction across a spectrum for the transmitted
///   media.
/// * k - Absorption coefficient.
fn fresnel_conductor(
    cos_theta_i: f32,
    eta_i: &RgbSpectrum,
    eta_t: &RgbSpectrum,
    k: &RgbSpectrum,
) -> RgbSpectrum {
    let cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
    let eta = eta_t / eta_i;
    let eta_k = k / eta_i;

    let cos_theta_i2 = cos_theta_i * cos_theta_i;
    let sin_theta_i2 = 1.0 - cos_theta_i2;
    let eta2 = eta * eta;
    let eta_k2 = eta_k * eta_k;

    let t0 = eta2 - eta_k2 - RgbSpectrum::constant(sin_theta_i2);
    let a2_plus_b2 = (t0 * t0 + 4.0 * eta2 * eta_k2).sqrt();
    let t1 = a2_plus_b2 + RgbSpectrum::constant(cos_theta_i2);
    let a = (0.5 * (a2_plus_b2 + t0)).sqrt();
    let t2 = RgbSpectrum::constant(2.0 * cos_theta_i) * a;
    let rs = (t1 - t2) / (t1 + t2);

    let sin_theta_i4 = sin_theta_i2 * sin_theta_i2;
    let t3 = (cos_theta_i2 * a2_plus_b2) + RgbSpectrum::constant(sin_theta_i4);
    let t4 = t2 * sin_theta_i2;
    let rp = rs * (t3 - t4) / (t3 + t4);

    0.5 * (rp + rs)
}
