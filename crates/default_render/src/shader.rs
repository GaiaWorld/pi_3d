use pi_atom::Atom;
use pi_scene_context::materials::material_meta::{ShaderEffectMeta, UniformPropertyVec4, ShaderEffectValueUniformDesc, UniformPropertyInt};
use pi_scene_math::Vector4;
use render_shader::{unifrom_code::MaterialValueBindDesc, varying_code::{Varyings, Varying}, block_code::{BlockCodeAtom}};

pub struct DefaultShader;
impl DefaultShader {
    pub const KEY: &'static str = "Default";
    pub fn res() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            ShaderEffectValueUniformDesc {
                set: 1,
                bind: 1,
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                mat2_list: vec![],
                vec4_list: vec![],
                vec2_list: vec![],
                float_list: vec![],
                int_list: vec![UniformPropertyInt(Atom::from("debug_normal"), 0)],
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
                    Varying { 
                        format: Atom::from("vec4"),
                        name: Atom::from("v_color"),
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
