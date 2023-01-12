use pi_ecs::{prelude::{Query, ResMut, Commands, Res}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::rhi::device::RenderDevice;
use render_resource::uniform_buffer::RenderDynUniformBuffer;
use render_shader::shader_set::ShaderSetModelAbout;

use crate::{
    bindgroup::{
        RenderBindGroupKey, RenderBindGroupPool,
        uniform_buffer::{SysDynUnifromBufferUpdate, DynUnifromBufferReBindFlag},
    }
};


pub struct SysModelAboutBindGroup;
impl TSystemStageInfo for SysModelAboutBindGroup {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysDynUnifromBufferUpdate::key()
        ]
    }
}
#[setup]
impl SysModelAboutBindGroup {
    #[system]
    fn sys(
        models: Query<GameObject, (ObjectID, &ShaderSetModelAbout, &DynUnifromBufferReBindFlag), Or<(Changed<DynUnifromBufferReBindFlag>, Changed<ShaderSetModelAbout>)>>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        device: Res<RenderDevice>,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        models.iter().for_each(|(id_model, model_set, flag)| {
            if let Some(group) = bindgrouppool.get_mut(&RenderBindGroupKey::ModelAbout(id_model)) {
                let entries = model_set.bind_group_entries(
                    &dynbuffer,
                    None, 
                    None,
                );

                group.bind_group = Some(
                    device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some(model_set.label()),
                        layout: &group.layout,
                        entries: entries.as_slice()
                    })
                );
                flag_delete.delete(id_model);
            }
        });
    }
}