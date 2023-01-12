use pi_atom::Atom;
use pi_scene_context::materials::shader_effect::{ShaderEffectMeta, UniformPropertyVec4};
use pi_scene_math::Vector4;
use render_shader::{
    block_code::{BlockCodeAtom},
    unifrom_code::{
        MaterialTextureBindDesc, MaterialValueBindDesc, UniformPropertyName, UniformTextureDesc,
    },
    varying_code::{Varying, Varyings},
};

pub struct UnlitShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl UnlitShader {
    pub const KEY: &'static str = "UnlitShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![
                    UniformPropertyVec4(Atom::from("emissive"), Vector4::new(1., 1., 1., 1.)),
                    UniformPropertyVec4(
                        Atom::from("emissive_scaleoffset"),
                        Vector4::new(1., 1., 0., 0.),
                    ),
                ],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            Some(MaterialTextureBindDesc {
                list: vec![UniformTextureDesc::new2d(
                    UniformPropertyName::from("_MainTex"),
                    wgpu::ShaderStages::FRAGMENT,
                )],
            }),
            Varyings(vec![
                Varying {
                    format: Atom::from("vec3"),
                    name: Atom::from("v_normal"),
                },
                Varying {
                    format: Atom::from("vec3"),
                    name: Atom::from("v_pos"),
                },
                Varying {
                    format: Atom::from("vec2"),
                    name: Atom::from("v_uv"),
                },
                Varying {
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]),
            BlockCodeAtom {
                define: Atom::from(include_str!("./unlit_define.vert")),
                running: Atom::from(include_str!("./unlit.vert")),
            },
            BlockCodeAtom {
                define: Atom::from(include_str!("./unlit_define.frag")),
                running: Atom::from(include_str!("./unlit.frag")),
            },
        )
    }
}
