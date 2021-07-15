/// Returns a conservative bound on polynomial error accumulation.
///
/// PBR ed. 3 (on p. 216) states that (1 + εₘ)ⁿ <= θₙ, where |θₙ| <= 𝛾ₙ and
/// where εₘ is the machine epsilon value. This function returns 𝛾ₙ.
pub fn gamma(n: i32) -> f32 {
    n as f32 * f32::EPSILON / (1.0 - n as f32 * f32::EPSILON)
}
