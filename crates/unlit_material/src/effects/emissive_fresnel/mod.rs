use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;

pub struct EmissiveFresnelShader;

impl EmissiveFresnelShader {
    pub const KEY: &'static str = "EmissiveFresnelShader";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./emissive_fresnel.frag"));

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

        nodemat.apply::<BlockFloat>();
        nodemat.apply::<BlockFresnel>();
        nodemat.apply::<BlockUVOffsetSpeed>();
        nodemat.apply::<BlockViewDirection>();
        nodemat.apply::<BlockEmissiveTexture>();
        nodemat.apply::<BlockEmissiveTextureUVOffsetSpeed>();
        nodemat.apply::<BlockEmissiveFresnel>();

        nodemat.meta()
    }
}
