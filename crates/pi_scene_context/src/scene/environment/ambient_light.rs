

use pi_engine_shell::prelude::*;

use super::BindSceneEffect;

#[derive(Component)]
pub struct AmbientColor(pub f32, pub f32, pub f32);

#[derive(Component)]
pub struct AmbientIntensity(pub f32);

pub fn update_ambient_uniform(color: &AmbientColor, intensity: &AmbientIntensity, bind: &mut BindSceneEffect) {
    let values = vec![color.0, color.1, color.2, intensity.0];
    bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_AMBIENT as usize, bytemuck::cast_slice(&values));

}
