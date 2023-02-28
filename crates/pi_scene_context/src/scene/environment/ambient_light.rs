
use pi_ecs::{prelude::Query, query::Changed};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::GameObject, run_stage::TSystemStageInfo};
use pi_render::render_3d::binds::scene::effect::ShaderBindSceneAboutEffect;

use super::BindSceneEffect;


pub struct AmbientLight {
    color: (f32, f32, f32),
    intensity: f32,
    pub dirty: bool,
}
impl AmbientLight {
    pub const AMBIENT_LIGHT: usize = 4;
    pub const AMBIENT_LIGHT_OFFSIZE: usize = 0 * 4;

    pub fn new() -> Self {
        Self {
            color: (1., 1., 1.),
            intensity: 1.0,
            dirty: true,
        }
    }
    pub fn color(&mut self, value: (f32, f32, f32)) {
        if self.color.0 != value.0 || self.color.1 != value.1 || self.color.2 != value.2 {
            self.dirty = true;
            self.color = value;
        }
    }
    pub fn intensity(&mut self, value: f32) {
        if self.intensity != value {
            self.dirty = true;
            self.intensity = value;
        }
    }
    pub fn update(&self, bind: &BindSceneEffect) {
        let values = vec![self.color.0, self.color.1, self.color.2, self.intensity];
        bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_AMBIENT as usize, bytemuck::cast_slice(&values));
    }
}

pub struct SysSceneAmbientUpdate;
impl TSystemStageInfo for SysSceneAmbientUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysSceneCreateCommand::key()
        ]
    }
}
#[setup]
impl SysSceneAmbientUpdate {
    #[system]
    fn sys(
        mut scenes: Query<GameObject, (&AmbientLight, &mut BindSceneEffect), Changed<AmbientLight>>,
    ) {
        scenes.iter_mut().for_each(|(item, mut bind)| {
            item.update(&mut bind);
        });
    }
}
