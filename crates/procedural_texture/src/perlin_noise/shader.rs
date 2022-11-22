use pi_render::rhi::{device::RenderDevice};
use render_material::{binding::BindingDesc, material::{UniformDesc, EUniformDataFormat}};

use pi_scene_context::{materials::MBKK, shaders::BuildinShaderDefined};

pub struct PerlinNoiseShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl PerlinNoiseShader {
    pub const A_POSITION: MBKK                          = BuildinShaderDefined::A_POSITION;
    pub const A_POSITION_SLOT: u32                      = 0;
    pub const A_POSITION_SIZE: u32                      = 3 * 4;
    pub const U_EMISSIVE: MBKK                          = BuildinShaderDefined::MBKK_START_FOR_OTHER + 01 as MBKK;
    
    pub fn new(device: &RenderDevice) -> Self {
        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("PerlinNoise-VS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("../assets/skybox.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: naga::FastHashMap::default(),
            },
        });
        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("PerlinNoise-FS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./perlin_noise.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: naga::FastHashMap::default(),
            },
        });

        Self {
            vs_module,
            fs_module,
        }
    }

    pub fn bind_desc() -> Vec<BindingDesc<MBKK>> {
        vec![
            BindingDesc {
                uniforms: vec![
                    UniformDesc {
                        kind: Self::U_EMISSIVE,
                        bind: 1,
                        format: EUniformDataFormat::Color4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        byte_offset_in_bind: 0,
                    }
                ],
                size: 4 * 4,
                id: 1
            }
        ]
    }
}