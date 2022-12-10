use pi_atom::Atom;
use render_shader::{shader::{PreShaderMeta}, unifrom_code::MaterialValueBindDesc, varying_code::{Varyings, Varying}, block_code::{BlockCodeAtom}};

pub struct DefaultShader;
impl DefaultShader {
    pub const KEY: &str = "Default";
    pub fn res() -> PreShaderMeta {
        PreShaderMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![Atom::from("emissive")],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![],
                uint_list: vec![],
            },
            None,
            Varyings(
                vec![
                    Varying { 
                        format: Atom::from("vec3"),
                        name: Atom::from("v_normal"),
                    },
                    Varying { 
                        format: Atom::from("vec3"),
                        name: Atom::from("v_pos"),
                    },
                ]
            ),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from(include_str!("./default.vert"))
            },
            BlockCodeAtom { 
                define: Atom::from(include_str!("./default_define.frag")), 
                running: Atom::from(include_str!("./default.frag"))
            },
        )
    }
}
