
use pi_scene_shell::prelude::*;

#[derive(Component)]
pub struct HemiGrounds(pub f32);
impl Default for HemiGrounds {
    fn default() -> Self {
        Self(0.)
    }
}