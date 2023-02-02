use pi_ecs::{prelude::{Query, ResMut, Commands, Res}, query::{Changed, Or, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::rhi::device::RenderDevice;
use render_resource::{uniform_buffer::RenderDynUniformBuffer, sampler::{SamplerDesc, SamplerPool}};
use render_shader::shader_set::ShaderSetModelAbout;

use crate::{
    bindgroup::{
        RenderBindGroupKey, RenderBindGroupPool,
        uniform_buffer::{SysDynUnifromBufferUpdate, DynUnifromBufferReBindFlag},
    }, skeleton::{SkeletonID, skin_texture::SkinTexture, skeleton::Skeleton}
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
        mut models: Query<GameObject, (ObjectID, &ShaderSetModelAbout, &mut DynUnifromBufferReBindFlag, Option<&SkeletonID>), Or<(Changed<DynUnifromBufferReBindFlag>, Changed<ShaderSetModelAbout>)>>,
        skins: Query<GameObject, Option<&SkinTexture>, With<Skeleton>>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        device: Res<RenderDevice>,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        models.iter_mut().for_each(|(id_model, model_set, flag, skin)| {
            let (bone_tex, bone_samp) = if let Some(id_skin) = skin {
                if let Some(skltex) = skins.get(id_skin.0) {
                    if let Some(skltex) = skltex {
                        (Some(skltex.tex.texture_view()), Some(&skltex.sampler))
                    } else {
                        (None, None)
                    }
                } else {
                    return;
                }
            } else {
                (None, None)
            };

            if let Some(group) = bindgrouppool.get_mut(&RenderBindGroupKey::ModelAbout(id_model)) {
                let entries = model_set.bind_group_entries(
                    &dynbuffer,
                    bone_tex, 
                    bone_samp,
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