use pi_atom::Atom;
use pi_scene_shell::prelude::*;
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
    

    pub fn create(_infos: &NodeMaterialBlocks) -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./stripes_virtual.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from(S_VEC3),
                    name: Atom::from(S_V_NORMAL),
                },
                Varying { 
                    format: Atom::from(S_VEC3),
                    name: Atom::from(S_V_POS),
                },
                Varying {
                    format: Atom::from(S_VEC2),
                    name: Atom::from(S_V_UV),
                },
                Varying { 
                    format: Atom::from(S_VEC4),
                    name: Atom::from(S_V_COLOR),
                },
            ]
        );

        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_STEP), 1., true)); 
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_SPEED), 0., true));
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_FADESTART), 0., true));
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_FADEEND), 1., true));

        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from(Self::KEY_DIRECTION), [0.5, 0.5, 0.5, 1.], false));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from(Self::KEY_COLOR0), [0.1, 0.5, 0.1, 0.5], true));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from(Self::KEY_COLOR1), [0.1, 1.0, 0.1, 1.0], true));
        
        nodemat.meta()
    }
}
