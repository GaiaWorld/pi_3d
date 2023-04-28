
use pi_engine_shell::prelude::*;

pub mod system;

#[derive(Component)]
pub struct ShadowAngle(pub f32);
impl Default for ShadowAngle {
    fn default() -> Self {
        Self(3.1415926 / 2.)
    }
}

