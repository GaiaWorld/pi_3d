use std::hash::{Hasher, Hash};

use pi_assets::asset::Handle;
use pi_hash::DefaultHasher;
use pi_render::{renderer::{pipeline::{KeyRenderPipelineState, KeyPipelineFromBindGroup}, vertex_buffer::KeyPipelineFromAttributes, bind_group::BindGroupLayout}, rhi::{device::RenderDevice, pipeline::RenderPipeline, asset::RenderRes}, asset::ASSET_SIZE_FOR_UNKOWN};

use crate::shader::{KeyShader3D, Shader3D};


/// * Pipeline 
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyRenderPipeline3D {
    pub key_state: KeyRenderPipelineState,
    pub key_shader: KeyShader3D,
    pub key_bindgroup_layouts: KeyPipelineFromBindGroup<4>,
    pub key_vertex_layouts: KeyPipelineFromAttributes,
}
impl KeyRenderPipeline3D {
    pub fn to_u64(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
    pub fn create(
        key: KeyRenderPipeline3D,
        shader: Handle<Shader3D>,
        bind_group_layouts: [Option<Handle<BindGroupLayout>>; 4],
        device: &RenderDevice,
    ) -> RenderRes<RenderPipeline> {
        let mut layouts: Vec<&wgpu::BindGroupLayout> = vec![];
        bind_group_layouts.iter().for_each(|v| {
            if let Some(v) = v {
                layouts.push(v.layout())
            }
        });
        // log::warn!("{:?}", &key.key_vertex_layouts.layouts());
        let vs_state = wgpu::VertexState {
            module: &shader.vs,
            entry_point: shader.vs_point,
            buffers: &key.key_vertex_layouts.layouts(),
        };
        let fs_state = wgpu::FragmentState {
            module: &shader.fs,
            entry_point: shader.fs_point,
            targets: &key.key_state.target_state(),
        };

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &layouts,
                push_constant_ranges: &[],
            }
        );

        let depth_stencil = if let Some(depth_stencil) = &key.key_state.depth_stencil {
            Some(depth_stencil.depth_stencil_state())
        } else {
            None
        };

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                // label: Some(shader.key()),
                layout: Some(&pipeline_layout),
                vertex: vs_state,
                fragment: Some(fs_state),
                primitive: key.key_state.primitive.clone(),
                depth_stencil,
                multisample: key.key_state.multisample,
                multiview: None,
            }
        );
        RenderRes::new(pipeline, ASSET_SIZE_FOR_UNKOWN)
    }
}