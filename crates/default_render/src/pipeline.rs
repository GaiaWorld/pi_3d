use pi_hash::XHashMap;
use pi_render::rhi::{device::RenderDevice, bind_group_layout::BindGroupLayout};
use pi_slotmap::DefaultKey;
use render_geometry::geometry::VertexAttributeMeta;
use render_pipeline_key::{pipeline_key::{PipelineKeyCalcolator, gen_pipeline_key}, fragment_state::gen_fragment_state_key};

use pi_scene_context::{resources::SingleRenderObjectPipelinePool, cameras::camera::CameraRenderData, environment::{fog::SceneFog, ambient_light::AmbientLight}, scene::scene_time::SceneTime, shaders::FragmentUniformBind, meshes::model::BuildinModelBind, vertex_data::{position::AttributePosition, normal::AttributeNormal}};

use super::{shader::DefaultShader, default_material::DefaultMaterialPropertype};


pub struct DefaultMaterialPipeline {
    pub map: XHashMap<u128, DefaultKey>,
}
impl Default for DefaultMaterialPipeline {
    fn default() -> Self {
        Self { map: XHashMap::default() }
    }
}
impl DefaultMaterialPipeline {
    pub fn build(
        &mut self,
        device: &RenderDevice,
        shader: &DefaultShader,
        targets: &[Option<wgpu::ColorTargetState>],
        depth_stencil: Option<wgpu::DepthStencilState>,
        primitive: wgpu::PrimitiveState,
        pipelines: &mut SingleRenderObjectPipelinePool,
    ) -> DefaultKey {

        let mut calcolator = PipelineKeyCalcolator::new();
        gen_pipeline_key(&mut calcolator, &primitive, &depth_stencil, 0, 8);
        match targets.get(0) {
            Some(target) => {
                match target {
                    Some(target) => {
                        gen_fragment_state_key(&mut calcolator, target);
                    },
                    None => {},
                }
            },
            None => {},
        }
        let key = calcolator.key;

        // let key = 1;

        match self.map.get(&key) {
            None => {
                let bind_group_0_layout = BindGroupLayout::from(
                    device.create_bind_group_layout(
                        &wgpu::BindGroupLayoutDescriptor {
                            label: Some("Default"),
                            entries: &[
                                CameraRenderData::ENTRY,
                                SceneFog::ENTRY,
                                SceneTime::ENTRY,
                                AmbientLight::ENTRY,
                            ],
                        }
                    )
                );
        
                let bind_group_1_layout = BindGroupLayout::from(
                    device.create_bind_group_layout(
                        &wgpu::BindGroupLayoutDescriptor {
                            label: Some("Default"),
                            entries: &[
                                BuildinModelBind::ENTRY,
                                DefaultMaterialPropertype::ENTRY,
                            ],
                        }
                    )
                );

                let vertex_layouts = vec![
                    AttributePosition::layout(&AttributePosition::ATTRIBUTES),
                    AttributeNormal::layout(&AttributeNormal::ATTRIBUTES),
                ];
                
                let vs_state = wgpu::VertexState {
                    module: &shader.vs_module,
                    entry_point: "main",
                    buffers: &vertex_layouts,
                };
                let fs_state = wgpu::FragmentState {
                    module: &shader.fs_module,
                    entry_point: "main",
                    targets,
                };
        
                let pipeline_layout = device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts: &[
                            bind_group_0_layout.value(),
                            bind_group_1_layout.value(),
                        ],
                        push_constant_ranges: &[],
                    }
                );
        
                let pipeline = device.create_render_pipeline(
                    &wgpu::RenderPipelineDescriptor {
                        label: Some("Default"),
                        layout: Some(&pipeline_layout),
                        vertex: vs_state,
                        fragment: Some(fs_state),
                        primitive,
                        depth_stencil,
                        multisample: wgpu::MultisampleState {
                            count: 1,
                            mask: !0,
                            alpha_to_coverage_enabled: false
                        },
                        multiview: None,
                    }
                );
                
                let id = pipelines.map.insert(pipeline);
                self.map.insert(key, id);
                id
            },
            Some(id) => {
                *id
            },
        }
    }
}