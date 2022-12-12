use pi_render::rhi::{device::RenderDevice};
use render_material::{binding::BindingDesc, material::{UniformDesc, EUniformDataFormat}};

use pi_scene_context::{materials::MBKK, shaders::BuildinShaderDefined};

pub struct SkeletonsShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl SkeletonsShader {
    pub const A_POSITION: MBKK                          = BuildinShaderDefined::A_POSITION;
    pub const A_POSITION_SLOT: u32                      = 0;
    pub const A_POSITION_SIZE: u32                      = 3 * 4;
    pub const U_EMISSIVE: MBKK                          = BuildinShaderDefined::MBKK_START_FOR_OTHER + 01 as MBKK;
    
    pub fn new(device: &RenderDevice) -> Self {
        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skeletons-VS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./assets/skeletons.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: naga::FastHashMap::default(),
            },
        });
        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skeletons-FS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./assets/skeletons.frag")),
                stage: naga::ShaderStage::Fragment,
                defines: naga::FastHashMap::default(),
            },
        });

        Self {
            vs_module,
            fs_module,
        }
    }
}