use pi_hash::XHashMap;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, device::RenderDevice};
use pi_slotmap::DefaultKey;
use render_geometry::geometry::VertexAttributeMeta;
use render_pipeline_key::{
    fragment_state::gen_fragment_state_key,
    pipeline_key::{gen_pipeline_key, PipelineKey, PipelineKeyCalcolator},
};

use material_textures::main_texture::{MainTextureKey, MainTextureRes, MainTextureSampler};
use pi_scene_context::{
    cameras::camera::CameraRenderData,
    environment::{ambient_light::AmbientLight, fog::SceneFog},
    main_camera_render::bind_group::IDMainCameraRenderBindGroup,
    meshes::model::BuildinModelBind,
    resources::SingleRenderObjectPipelinePool,
    scene::scene_time::SceneTime,
    shaders::{FragmentUniformBind, FragmentUniformBindTexture, FragmentUniformBindTextureSampler},
    vertex_data::{normal::AttributeNormal, position::AttributePosition, uv::AttributeUV},
};

use super::{
    bind_group::{UnlitMaterialBindGroup, UnlitMaterialTextureBindGroup},
    define::UnlitMaterialMode,
    material::UnlitMaterialPropertype,
    shader::UnlitShader,
};

pub struct UnlitMaterialPipeline {
    pub map: XHashMap<UnlitMaterialMode, XHashMap<PipelineKey, DefaultKey>>,
}
impl Default for UnlitMaterialPipeline {
    fn default() -> Self {
        Self {
            map: XHashMap::default(),
        }
    }
}
impl UnlitMaterialPipeline {
    pub fn build(
        &mut self,
        mode: UnlitMaterialMode,
        device: &RenderDevice,
        shader: &UnlitShader,
        targets: &[Option<wgpu::ColorTargetState>],
        depth_stencil: Option<wgpu::DepthStencilState>,
        primitive: wgpu::PrimitiveState,
        pipelines: &mut SingleRenderObjectPipelinePool,
    ) -> DefaultKey {
        let mut calcolator = PipelineKeyCalcolator::new();
        gen_pipeline_key(&mut calcolator, &primitive, &depth_stencil, 0, 8);
        match targets.get(0) {
            Some(target) => match target {
                Some(target) => {
                    gen_fragment_state_key(&mut calcolator, target);
                }
                None => {}
            },
            None => {}
        }
        let key = calcolator.key;

        // let key = 1;

        if self.map.contains_key(&mode) == false {
            self.map.insert(mode, XHashMap::default());
        }

        let map = self.map.get_mut(&mode).unwrap();

        match map.get_mut(&key) {
            None => {
                let bind_group_0_layout = IDMainCameraRenderBindGroup::layout(device);
                let bind_group_1_layout = UnlitMaterialBindGroup::layout(device);
                let bind_group_2_layout = UnlitMaterialTextureBindGroup::layout(mode, device);

                let vertex_layouts = vec![
                    AttributePosition::layout(&AttributePosition::ATTRIBUTES),
                    AttributeNormal::layout(&AttributeNormal::ATTRIBUTES),
                    AttributeUV::layout(&AttributeUV::ATTRIBUTES),
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

                let pipeline_layout =
                    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts: &[
                            bind_group_0_layout.value(),
                            bind_group_1_layout.value(),
                            bind_group_2_layout.value(),
                        ],
                        push_constant_ranges: &[],
                    });

                println!("{:?}", pipeline_layout);

                let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Unlit"),
                    layout: Some(&pipeline_layout),
                    vertex: vs_state,
                    fragment: Some(fs_state),
                    primitive,
                    depth_stencil,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

                let id = pipelines.map.insert(pipeline);
                map.insert(key, id);
                id
            }
            Some(id) => *id,
        }
    }
}
