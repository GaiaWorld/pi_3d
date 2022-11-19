use pi_hash::XHashMap;
use pi_render::rhi::{device::RenderDevice, shader::Shader};
use render_material::{binding::BindingDesc, material::{UniformDesc, EUniformDataFormat}};

use pi_scene_context::{materials::MBKK, shaders::BuildinShaderDefined};

use crate::define::{StandardMaterialMode, StandardMaterialDefines};

#[derive(Debug, Default)]
pub struct StandardShaderPool {
    pub map: XHashMap<StandardMaterialMode, StandardShader>,
}
impl StandardShaderPool {
    pub fn get(
        &mut self,
        defines: &StandardMaterialDefines,
        device: &RenderDevice,
    ) -> &StandardShader {
        let mode = defines.mode();
        if self.map.contains_key(&mode) == false {
            let shader = StandardShader::new(defines, device);
            self.map.insert(mode, shader);
        }

        self.map.get(&mode).unwrap()
    }
}

#[derive(Debug)]
pub struct StandardShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl StandardShader {
    pub const A_POSITION: MBKK                          = BuildinShaderDefined::A_POSITION;
    pub const A_POSITION_SLOT: u32                      = 0;
    pub const A_POSITION_SIZE: u32                      = 3 * 4;
    pub const U_EMISSIVE: MBKK                          = BuildinShaderDefined::MBKK_START_FOR_OTHER + 01 as MBKK;
    
    pub fn new(defines: &StandardMaterialDefines, device: &RenderDevice) -> Self {
        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Default-VS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./standard.vert")),
                stage: naga::ShaderStage::Vertex,
                defines: naga::FastHashMap::default(),
            },
        });
        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Default-FS"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!("./standard.frag")),
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