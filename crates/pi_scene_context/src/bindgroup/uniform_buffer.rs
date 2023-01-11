use pi_ecs::{prelude::{Setup, ResMut, Res, Query, Commands}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_shader::shader_set::{ShaderSetEffectAbout, ShaderSetSceneAbout, ShaderSetModelAbout};


#[derive(Debug, Default)]
pub struct SingleDynUnifromBufferReBindFlag(pub bool);

pub struct DynUnifromBufferReBindFlag;

pub struct SysDynUnifromBufferUpdate;
impl TSystemStageInfo for SysDynUnifromBufferUpdate {}
#[setup]
impl SysDynUnifromBufferUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut flag: ResMut<SingleDynUnifromBufferReBindFlag>,
        mut scenes: Query<
            GameObject,
            (ObjectID, &ShaderSetSceneAbout)
        >,
        mut models: Query<
            GameObject,
            (ObjectID, &ShaderSetModelAbout)
        >,
        mut materials: Query<
            GameObject,
            (ObjectID, &ShaderSetEffectAbout)
        >,
        mut flag_insert: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        // log::debug!("SysDynUnifromBuffer Update");
        if dynbuffer.write_buffer(&device, &queue) {
            scenes.iter_mut().for_each(|(obj, _)| {
                flag_insert.insert(obj.clone(), DynUnifromBufferReBindFlag);
            });
            models.iter_mut().for_each(|(obj, _)| {
                flag_insert.insert(obj.clone(), DynUnifromBufferReBindFlag);
            });
            materials.iter_mut().for_each(|(obj, _)| {
                flag_insert.insert(obj.clone(), DynUnifromBufferReBindFlag);
            });
        }
    }
}