use pi_atom::Atom;
use pi_engine_shell::prelude::*;

pub struct PerlinNoiseShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl PerlinNoiseShader {
    pub const KEY: &'static str     = "PerlinNoiseShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                // mat2_list: vec![],
                vec4_list: vec![],
                vec3_list: vec![],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from("size"), 1., true),
                    UniformPropertyFloat(Atom::from("width"), 1., true),
                    UniformPropertyFloat(Atom::from("height"), 1., true),
                ],
                // int_list: vec![],
                uint_list: vec![],
            },
            vec![],
            Varyings(
                vec![
                ]
            ),
            String::from(""),
            // EVerticeExtendCode::default(),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from(include_str!("../assets/skybox.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./perlin_noise_define.frag")),
                running: Atom::from(include_str!("./perlin_noise.frag"))
            },
            ShaderDefinesSet::default()
        )
    }
}