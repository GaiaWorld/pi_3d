use pi_ecs::prelude::{Query, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;

use crate::{object::GameObject, scene::SceneTime, environment::{fog::SceneFog, ambient_light::AmbientLight}, shaders::buildin_uniforms::{BuildinTimeBind, BuildinFogBind, BuildinAmbientLightBind}};

pub struct SceneUniformTickUpdate;
#[setup]
impl SceneUniformTickUpdate {
    #[system]
    pub fn tick(
        query_scenes: Query<GameObject, (&SceneTime, &SceneFog, &AmbientLight, &BuildinTimeBind, &BuildinFogBind, &BuildinAmbientLightBind)>,
        dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        query_scenes.iter().for_each(|(time, fog, ambl, time_bind, fog_bind, ambl_bind)| {
            dynbuffer.set_uniform::<SceneTime>(&time_bind.bind_offset, time);
            dynbuffer.set_uniform::<SceneFog>(&fog_bind.bind_offset, fog);
            dynbuffer.set_uniform::<AmbientLight>(&ambl_bind.bind_offset, ambl);
        });
    }
}
