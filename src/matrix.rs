use cgmath::{BaseFloat, Matrix4};

pub fn identity4<S: BaseFloat>() -> Matrix4<S> {
    Matrix4::from_scale(S::one())
}
