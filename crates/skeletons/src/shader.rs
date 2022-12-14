use pi_atom::Atom;
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::materials::material_meta::{
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

pub struct SkinShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl SkinShader {
    pub const KEY: &'static str = "SkinShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![
                    UniformPropertyMat4(
                        Atom::from("u_jointMat0"),
                        Matrix::new(
                            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0,
                        ),
                    ),
                    UniformPropertyMat4(
                        Atom::from("u_jointMat1"),
                        Matrix::new(
                            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0,
                            0.0, 1.0,
                        ),
                    ),
                    UniformPropertyMat4(
                        Atom::from("u_jointMat2"),
                        Matrix::new(
                            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0,
                        ),
                    ),
                    UniformPropertyMat4(
                        Atom::from("u_jointMat3"),
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
            Varyings(vec![]),
            BlockCodeAtom {
                define: Atom::from(include_str!("./assets/skin_define.vert")),
                running: Atom::from(include_str!("./assets/skin.vert")),
            },
            BlockCodeAtom {
                define: Atom::from(include_str!("./assets/skin_define.frag")),
                running: Atom::from(include_str!("./assets/skin.frag")),
            },
        )
    }
}
