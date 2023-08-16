use pi_atom::Atom;
use pi_engine_shell::prelude::*;

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
                mat4_list: vec![],
                // mat2_list: vec![],
                vec4_list: vec![],
                vec2_list: vec![],
                float_list: vec![],
                // int_list: vec![],
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
