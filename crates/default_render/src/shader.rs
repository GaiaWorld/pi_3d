use pi_atom::Atom;
use pi_scene_context::materials::material_meta::{ShaderEffectMeta, UniformPropertyVec4, ShaderEffectValueUniformDesc};
use pi_scene_math::Vector4;
use render_shader::{unifrom_code::MaterialValueBindDesc, varying_code::{Varyings, Varying}, block_code::{BlockCodeAtom}};

pub struct DefaultShader;
impl DefaultShader {
    pub const KEY: &'static str = "Default";
    pub fn res() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            ShaderEffectValueUniformDesc::none(1, 1, wgpu::ShaderStages::VERTEX_FRAGMENT),
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
