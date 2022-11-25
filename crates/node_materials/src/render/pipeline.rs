use pi_ecs::{prelude::{Query, ResMut, Res}, query::{Write, Or, Changed}};
use pi_engine_shell::object::GameObject;
use pi_hash::XHashMap;
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::{renderers::pipeline::PipelineKey, resources::SingleRenderObjectPipelinePool};
use pi_slotmap::DefaultKey;

use crate::uniforms::{value_uniform::ValueBindDesc, texture_uniform::TextureBindDesc};

pub struct SingleNodeMaterialPipelinePool {
    pub map: XHashMap<pi_atom::Atom, DefaultKey>,
}

pub struct SysPipeline;
impl SysPipeline {
    pub fn sys(
        mut items: Query<GameObject, (&ValueBindDesc, &TextureBindDesc, Write<PipelineKey>), Or<(Changed<ValueBindDesc>, Changed<TextureBindDesc>)>>,
        mut pipelines: ResMut<SingleNodeMaterialPipelinePool>,
        mut pipelinepool: ResMut<SingleRenderObjectPipelinePool>,
        device: Res<RenderDevice>,
    ) {
        items.iter_mut().for_each(|(value_uniforms, tex_uniforms, mut pipelineKey)| {
            let key = value_uniforms.label() + tex_uniforms.label().as_str();
            let key = pi_atom::Atom::from(key);
            // let key = if let Some(key) = pipelines.map.get(&key) {
            //     key.clone()
            // } else {
            //     todo!()
            // };
        });
    }
}