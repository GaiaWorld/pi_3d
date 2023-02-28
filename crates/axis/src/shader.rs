use pi_atom::Atom;
use pi_render::render_3d::shader::{uniform_value::{MaterialValueBindDesc, UniformPropertyMat4}, varying_code::{Varyings, Varying}, block_code::BlockCodeAtom, shader_defines::ShaderDefinesSet};
use pi_scene_context::materials::shader_effect::{
    ShaderEffectMeta
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
                        [
                            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0,
                        ],
                    ),
                ],
                mat2_list: vec![],
                vec4_list: vec![],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            vec![],
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
            ShaderDefinesSet::default(),
        )
    }
}
