use pi_atom::Atom;
use pi_engine_shell::prelude::*;

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
                // mat2_list: vec![],
                vec4_list: vec![
                    UniformPropertyVec4(Atom::from("sea_base"), [0.0, 0.09, 0.18, 1.]),
                    UniformPropertyVec4(
                        Atom::from("sea_water_color"),
                        [0.48, 0.54, 0.36, 1.0],
                    ),
                ],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from("width"), 800.),
                    UniformPropertyFloat(Atom::from("height"), 600.),
                    UniformPropertyFloat(Atom::from("iTime"), 0.2),
                    UniformPropertyFloat(Atom::from("phantom_data"), 0.),
                ],
                // int_list: vec![],
                uint_list: vec![],
            },
            vec![],
            Varyings(vec![]),
            BlockCodeAtom {
                define: Atom::from(""),
                running: Atom::from(include_str!("../assets/skybox.vert")),
            },
            BlockCodeAtom {
                define: Atom::from(include_str!("./water_define.frag")),
                running: Atom::from(include_str!("./water.frag")),
            },
            ShaderDefinesSet::default()
        )
    }
}
