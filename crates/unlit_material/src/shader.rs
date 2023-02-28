use pi_atom::Atom;
use pi_render::{render_3d::shader::{uniform_value::{MaterialValueBindDesc, UniformPropertyVec4}, uniform_texture::UniformTexture2DDesc, UniformPropertyName, varying_code::{Varyings, Varying}, block_code::BlockCodeAtom, shader_defines::ShaderDefinesSet}, renderer::{buildin_data::EDefaultTexture, shader_stage::EShaderStage}};
use pi_scene_context::materials::shader_effect::{ShaderEffectMeta};

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
                    UniformPropertyVec4(Atom::from("emissive"), [1., 1., 1., 1.]),
                    UniformPropertyVec4(
                        Atom::from("emissive_scaleoffset"),
                        [1., 1., 0., 0.],
                    ),
                ],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            vec![
                UniformTexture2DDesc::new(
                    UniformPropertyName::from("_MainTex"),
                    wgpu::TextureSampleType::Float { filterable: true },
                    false,
                    EShaderStage::FRAGMENT,
                    EDefaultTexture::White
                )
            ],
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
            ShaderDefinesSet::default()
        )
    }
}
