
use pi_atom::Atom;
use pi_engine_shell::prelude::*;

pub struct BRDFShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl BRDFShader {
    pub const KEY: &'static str     = "BrdfShader";

    pub fn meta() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                // mat2_list: vec![],
                vec4_list: vec![UniformPropertyVec4(Atom::from("skyColor"), [0.15, 0.68, 1.0, 1.0]),],
                vec2_list: vec![],
                float_list: vec![],
                // int_list: vec![],
                uint_list: vec![],
            },
            vec![
                UniformTexture2DDesc::new(
                    UniformPropertyName::from("_MainTex"),
                    wgpu::TextureSampleType::Float { filterable: false },
                    false,
                    EShaderStage::FRAGMENT,
                    EDefaultTexture::White,
                )
            ],
            Varyings(
                vec![
                    Varying{ format: Atom::from("vec2"), name: Atom::from("v_UV") }, 
                ]
            ),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from(include_str!("./brdf.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./brdf_define.frag")),
                running: Atom::from(include_str!("./brdf.frag"))
            },
            ShaderDefinesSet::default()
        )
    }
}