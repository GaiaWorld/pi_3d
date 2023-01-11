use pi_atom::Atom;
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::materials::shader_effect::{
    ShaderEffectMeta, UniformPropertyFloat, UniformPropertyVec2, UniformPropertyVec4,
};
use pi_scene_math::{Vector2, Vector4};
use render_shader::{
    block_code::{BlockCode, BlockCodeAtom},
    shader::KeyShaderEffect,
    unifrom_code::MaterialValueBindDesc,
    varying_code::{Varying, Varyings},
};

pub struct WaterShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl WaterShader {
    pub const KEY: &'static str = "WaterShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![
                    UniformPropertyVec4(Atom::from("sea_base"), Vector4::new(0.0, 0.09, 0.18, 1.0)),
                    UniformPropertyVec4(
                        Atom::from("sea_water_color"),
                        Vector4::new(0.48, 0.54, 0.36, 1.0),
                    ),
                ],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from("width"), 800.),
                    UniformPropertyFloat(Atom::from("height"), 600.),
                    UniformPropertyFloat(Atom::from("iTime"), 0.2),
                    UniformPropertyFloat(Atom::from("phantom_data"), 0.),
                ],
                int_list: vec![],
                uint_list: vec![],
            },
            None,
            Varyings(vec![]),
            BlockCodeAtom {
                define: Atom::from(""),
                running: Atom::from(include_str!("../assets/skybox.vert")),
            },
            BlockCodeAtom {
                define: Atom::from(include_str!("./water_define.frag")),
                running: Atom::from(include_str!("./water.frag")),
            },
        )
    }
}
