

use pi_scene_shell::prelude::*;

use super::BindSceneEffect;

#[derive(Debug, Component)]
pub struct AmbientColor(pub f32, pub f32, pub f32, pub f32);

pub fn update_ambient_uniform(color: &AmbientColor, bind: &mut BindSceneEffect) {
    let values = vec![color.0, color.1, color.2, color.3];
    bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_AMBIENT as usize, bytemuck::cast_slice(&values));

}
