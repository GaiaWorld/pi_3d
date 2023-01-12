use pi_ecs::{prelude::{Query, ResMut, Res, Commands}, query::Changed};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::rhi::device::RenderDevice;
use render_resource::uniform_buffer::RenderDynUniformBuffer;
use render_shader::shader_set::ShaderSetSceneAbout;

use crate::{
    scene::{
        scene_time::SceneTime,
        environment::fog::SceneFog,
    },
    flags::SceneID,
    bindgroup::{
        RenderBindGroupKey, RenderBindGroupPool,
        uniform_buffer::{SysDynUnifromBufferUpdate, DynUnifromBufferReBindFlag},
    }
};


pub struct SysMainCameraRendererBindGroup;
impl TSystemStageInfo for SysMainCameraRendererBindGroup {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysDynUnifromBufferUpdate::key()
        ]
    }
}
#[setup]
impl SysMainCameraRendererBindGroup {
    #[system]
    fn sys(
        scenes: Query<GameObject, (&SceneTime, &SceneFog)>,
        renderers: Query<GameObject, (ObjectID, &SceneID, &ShaderSetSceneAbout, &DynUnifromBufferReBindFlag), Changed<DynUnifromBufferReBindFlag>>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        device: Res<RenderDevice>,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        renderers.iter().for_each(|(id_renderer, id_scene, renderer_set, flag)|{
                if let Some((scenetime, scenefog)) = scenes.get(id_scene.0) {
                    if renderer_set.brdf() == false && renderer_set.env() == false {
                        if let Some(group) = bindgrouppool.get_mut(&RenderBindGroupKey::SceneAbout(id_renderer)) {
                            let entries = renderer_set.bind_group_entries(
                                &dynbuffer,
                                None, 
                                None,
                                None,
                                None
                            );
    
                            group.bind_group = Some(
                                device.create_bind_group(&wgpu::BindGroupDescriptor {
                                    label: Some(renderer_set.label()),
                                    layout: &group.layout,
                                    entries: entries.as_slice()
                                })
                            );
                            flag_delete.delete(id_renderer);
                        }
                    }
                }
        });
    }
}