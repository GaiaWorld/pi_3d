use pi_scene_shell::prelude::*;




#[derive(Component, Default)]
pub struct SpotLightAngle {
    pub(crate) in_value: f32,
    pub(crate) out_value: f32,
}