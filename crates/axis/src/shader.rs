use pi_atom::Atom;
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::materials::shader_effect::{
    ShaderEffectMeta, UniformPropertyFloat, UniformPropertyMat4, UniformPropertyVec2,
    UniformPropertyVec4,
};
use pi_scene_math::{Matrix, Vector2, Vector4};
use render_shader::{
    block_code::{BlockCode, BlockCodeAtom},
    shader::KeyShaderEffect,
    unifrom_code::MaterialValueBindDesc,
    varying_code::{Varying, Varyings},
};

pub struct AxisShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl AxisShader {
    pub const KEY: &'static str = "AxisShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![
                    UniformPropertyMat4(
                        Atom::from("u_jointMat0"),
                        Matrix::new(
                            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0,
                        ),
                    ),
                ],
                mat2_list: vec![],
                vec4_list: vec![],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            None,
            Varyings(vec![
                Varying{ format: Atom::from("vec4"), name: Atom::from("v_color") }, 
            ]),
            BlockCodeAtom {
                define: Atom::from(""),
                running: Atom::from(include_str!("./assets/axis.vert")),
            },
            BlockCodeAtom {
                define: Atom::from(include_str!("./assets/axis_define.frag")),
                running: Atom::from(include_str!("./assets/axis.frag")),
            },
        )
    }
}
