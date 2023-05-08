use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};

pub struct StripesVirtualShader;

impl StripesVirtualShader {
    pub const KEY: &'static str = "StripesVirtualShader";

    pub const KEY_DIRECTION: &'static str = "uDirection";
    pub const KEY_STEP: &'static str = "uStep";
    pub const KEY_SPEED: &'static str = "uSpeed";
    pub const KEY_FADESTART: &'static str = "uFadeStart";
    pub const KEY_FADEEND: &'static str = "uFadeEnd";
    pub const KEY_COLOR0: &'static str = "uColor0";
    pub const KEY_COLOR1: &'static str = "uColor1";
    

    pub fn create(infos: &NodeMaterialBlocks) -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./stripes_virtual.frag"));

        nodemat.varyings = Varyings(
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
                    format: Atom::from("vec2"),
                    name: Atom::from("v_uv"),
                },
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]
        );

        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_STEP), 1.)); 
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_SPEED), 0.));
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_FADESTART), 0.));
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_FADEEND), 1.));

        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from(Self::KEY_DIRECTION), [0.5, 0.5, 0.5, 1.]));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from(Self::KEY_COLOR0), [0.1, 0.5, 0.1, 0.5]));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from(Self::KEY_COLOR1), [0.1, 1.0, 0.1, 1.0]));
        
        nodemat.meta()
    }
}
