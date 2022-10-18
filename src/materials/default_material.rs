use pi_hash::XHashMap;
use pi_render::rhi::{dyn_uniform_buffer::{Bind, BindOffset, DynUniformBuffer, Uniform}, device::RenderDevice, bind_group::BindGroup, pipeline::RenderPipeline, bind_group_layout::BindGroupLayout};
use pi_scene_math::{Color4, Matrix};
use render_material::material::{Material, UnifromData};

use crate::{shaders::{default::DefaultShader, BuildinShaderDefined, buildin_uniforms::{BuildinModelBind, bind_group_entry_buffer}, FragmentUniformBind, buildin_attributes::{BuildinAttributePosition, BuildinAttributeColor4}, VertexAttributeMeta }, cameras::camera::CameraRenderData, environment::{fog::SceneFog, ambient_light::AmbientLight}, scene::SceneTime};

use super::{MBKK, bytes_write_to_memory};

/// 
/// 暴露材质 Unifrom 修改
/// 需要在渲染前 应用到 MaterialMeta 上
pub struct DefaultMaterial {
    emissive_color: (f32, f32, f32),
    emissive_intenisty: f32,
    material: Material<MBKK>,
}

impl Default for DefaultMaterial {
    fn default() -> Self {
        let mut material = Material::default();

        // material.init();

        Self {
            emissive_color: (1., 1., 1.),
            emissive_intenisty: 1.,
            material,
        }
    }
}

impl DefaultMaterial {
    pub fn init(
        &mut self,
    ) {
        self.material.init(&DefaultShader::bind_desc());
    }

    pub fn apply(
        &mut self,
        m_view: &Matrix,
        m_project: &Matrix,
        m_model: &Matrix,
    ) {
        let mut data = UnifromData::Color4(Color4::new(self.emissive_color.0, self.emissive_color.1, self.emissive_color.2, self.emissive_intenisty));
        self.material.modify(DefaultShader::U_EMISSIVE, data);

        data = UnifromData::Mat4(m_view.clone());
        self.material.modify(BuildinShaderDefined::U_MATRIX_V, data);

        data = UnifromData::Mat4(m_project.clone());
        self.material.modify(BuildinShaderDefined::U_MATRIX_P, data);
        
        data = UnifromData::Mat4(m_model.clone());
        self.material.modify(BuildinShaderDefined::U_OBJECT_TO_WORLD, data);

    }
}

pub struct DefaultMaterialMeta {
    pub set: u32,
    pub bind_group: Option<BindGroup>,
    pub model_bind_offset: Option<BindOffset>,
    pub value: Option<DefaultMaterialPropertype>,
}
impl DefaultMaterialMeta {
    pub fn new() -> Self {

        Self {
            bind_group: None,
            value: None,
            model_bind_offset: None,
            set: 1,
        }
    }

    pub fn init(
        &mut self,
        device: &RenderDevice,
        dynbuffer: &mut DynUniformBuffer,
    ) {
        if self.bind_group.is_none() {
            let model_bind_offset = dynbuffer.alloc_binding::<BuildinModelBind>();
            let value = DefaultMaterialPropertype::new(dynbuffer);
    
            let bind_group_layout = BindGroupLayout::from(
                device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: Some("DefaultMaterial"),
                        entries: &[
                            BuildinModelBind::ENTRY,
                            DefaultMaterialPropertype::ENTRY,
                        ],
                    }
                )
            );
            
            let bind_group = BindGroup::from(
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some("DefaultMaterial"),
                        layout: &bind_group_layout,
                        entries: &[
                            BuildinModelBind::entry(&model_bind_offset, dynbuffer),
                            DefaultMaterialPropertype::entry(&value.bind_offset, dynbuffer),
                        ],
                    }
                )
            );
    
            self.value = Some(value);
            self.bind_group = Some(bind_group);
            self.model_bind_offset = Some(model_bind_offset);
        }
    }
}
pub struct DefaultMaterialPropertype {
    pub bind_offset: BindOffset,
    pub emissive_color: (f32, f32, f32),
    pub emissive_intensity: f32,
}
impl DefaultMaterialPropertype {
    pub const EMISSIVE: usize = 4;
    pub const EMISSIVE_OFFSET: usize = 0 * 4;

    fn new(dynbuffer: &mut DynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            emissive_color: (1., 1., 1.),
            emissive_intensity: 1.,
        }
    }
}
impl FragmentUniformBind for DefaultMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_OFFSET + Self::EMISSIVE * 4;
}
impl Bind for DefaultMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for DefaultMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![
            self.emissive_color.0, self.emissive_color.1, self.emissive_color.2, self.emissive_intensity
        ];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

// pub struct 

pub struct DefaultMaterialPipeline {
    pub pipeline: RenderPipeline,
}

impl DefaultMaterialPipeline {
    pub const ID: usize = 00;
    pub fn build(
        device: &RenderDevice,
        dynbuffer: &mut DynUniformBuffer,
        shader: &DefaultShader,
        targets: &[Option<wgpu::ColorTargetState>],
        depth_stencil: Option<wgpu::DepthStencilState>,
        primitive: wgpu::PrimitiveState,
    ) -> Self {

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
            BuildinAttributePosition::layout(&BuildinAttributePosition::ATTRIBUTES),
            BuildinAttributeColor4::layout(&BuildinAttributeColor4::ATTRIBUTES),
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

        Self {
            pipeline,
        }
    }
}
