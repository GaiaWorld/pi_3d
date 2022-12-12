use pi_atom::Atom;
use pi_scene_context::materials::material_meta::{ShaderEffectMeta, UniformPropertyVec4};
use pi_scene_math::Vector4;
use render_shader::{unifrom_code::MaterialValueBindDesc, varying_code::{Varyings, Varying}, block_code::{BlockCodeAtom}};

pub struct DefaultShader;
impl DefaultShader {
    pub const KEY: &str = "Default";
    pub fn res() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            MaterialValueBindDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![UniformPropertyVec4(Atom::from("emissive"), Vector4::new(1., 1., 1., 1.))],
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
