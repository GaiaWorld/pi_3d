
use pi_scene_shell::prelude::*;


pub struct HemiGrounds(pub f32);
impl Default for HemiGrounds {
    fn default() -> Self {
        Self(0.)
    }
}