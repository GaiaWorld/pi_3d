use pi_atom::Atom;
use pi_engine_shell::prelude::*;


pub struct CloudShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl CloudShader {
    pub const KEY: &'static str     = "CloudShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                // mat2_list: vec![],
                vec4_list: vec![
                    UniformPropertyVec4(Atom::from("skyColor"), [0.15, 0.68, 1.0, 1.0]),
                    UniformPropertyVec4(Atom::from("cloudColor"), [1., 1., 1., 1.]),
                ],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from("amplitude"), 1.),
                    UniformPropertyFloat(Atom::from("numOctaves"), 4.),
                    UniformPropertyFloat(Atom::from("width"), 800.),
                    UniformPropertyFloat(Atom::from("height"), 600.),
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
            EVerticeExtendCode::default(),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from(include_str!("../assets/skybox.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./cloud_define.frag")),
                running: Atom::from(include_str!("./cloud.frag"))
            },
            ShaderDefinesSet::default()
        )
    }
}