//! Provides geometric operations, where all operations are assumed to take
//! place in the shading coordinate system. The functions in this module use the
//! assumption that all vector parameters are unit vectors to optimize the
//! calculations they perform.
//!
//! The shading coordinate system has its origin at a point of interest on a
//! surface. The z axis points along the surface's normal vector. The x and y
//! axes are tangent to the surface.
//!
//! Spherical coordinates, theta and phi, can be expresed in the shading
//! coordinate system. Theta is the angle from the z axis. Phi is the angle from
//! the x axis after the point is projected onto the xy plane.

use cgmath::Vector3;

/// Return the cosine of theta, where theta is the angle from the unit vector
/// `w` to the z axis.
pub fn cos_theta(w: &Vector3<f32>) -> f32 {
    w.z
}

/// Return the cosine squared of theta, where theta is the angle from the unit
/// vector `w` to the z axis.
pub fn cos2_theta(w: &Vector3<f32>) -> f32 {
    w.z * w.z
}

/// Return the absolute value of the cosine of theta, where theta is the angle
/// from the unit vector `w` to the z axis.
pub fn abs_cos_theta(w: &Vector3<f32>) -> f32 {
    w.z.abs()
}

/// Return the sine of theta, where theta is the angle from the unit vector `w`
/// to the z axis.
pub fn sin_theta(w: &Vector3<f32>) -> f32 {
    sin2_theta(w).sqrt()
}

/// Return the sine squared of theta, where theta is the angle from the unit
/// vector `w` to the z axis.
pub fn sin2_theta(w: &Vector3<f32>) -> f32 {
    // We use the rule that sin^2(theta) + cos^2(theta) = 1.

    // Floating point rounding error could cause this to result in a negative
    // number that is very close to zero when `cos2_theta(w)` is very close to
    // 1. We never want to return a negative value since we might want to take
    // the square root, and this is only due to rounding error, so we clamp the
    // lower bound at 0.
    (1.0 - cos2_theta(w)).max(0.0)
}

/// Return the tangent of theta, where theta is the angle from the unit vector
/// `w` to the z axis.
pub fn tan_theta(w: &Vector3<f32>) -> f32 {
    sin_theta(w) / cos_theta(w)
}

/// Return the tangent squared of theta, where theta is the angle from the unit
/// vector `w` to the z axis.
pub fn tan2_theta(w: &Vector3<f32>) -> f32 {
    sin2_theta(w) / cos2_theta(w)
}

/// Return the cosine of phi, where phi is the angle from the unit vector `w`'s
/// projection on the xy plan to the x axis.
pub fn cos_phi(w: &Vector3<f32>) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        1.0
    } else {
        (w.x / sin_theta).clamp(-1.0, 1.0)
    }
}

/// Return the cosine squared of phi, where phi is the angle from the unit
/// vector `w`'s projection on the xy plan to the x axis.
pub fn cos2_phi(w: &Vector3<f32>) -> f32 {
    let cos_phi = cos_phi(w);
    cos_phi * cos_phi
}

/// Return the sine of phi, where phi is the angle from the unit vector `w`'s
/// projection on the xy plan to the x axis.
pub fn sin_phi(w: &Vector3<f32>) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        0.0
    } else {
        (w.y / sin_theta).clamp(-1.0, 1.0)
    }
}

/// Return the sine squared of phi, where phi is the angle from the unit vector
/// `w`'s projection on the xy plan to the x axis.
pub fn sin2_phi(w: &Vector3<f32>) -> f32 {
    let sin_phi = sin_phi(w);
    sin_phi * sin_phi
}
