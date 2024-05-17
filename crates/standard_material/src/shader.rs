use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;

use crate::block_lighting::BlockStandardLighting;

pub struct StandardShader {
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
}

impl StandardShader {
    pub const KEY: &'static str = "Standard";

    pub fn meta() -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from(include_str!("./standard.vert"));
        nodemat.fs = String::from(include_str!("./standard.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from(S_VEC3),
                    name: Atom::from(S_V_NORMAL),
                },
                Varying { 
                    format: Atom::from(S_VEC4),
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

        nodemat.apply::<BlockUVAtlas>();
        nodemat.apply::<BlockUVOffsetSpeed>();
        nodemat.apply::<BlockMainTexture>();
        nodemat.apply::<BlockMainTextureUVOffsetSpeed>();
        nodemat.apply::<BlockViewDirection>();
        nodemat.apply::<BlockShadowMapping>();
        nodemat.apply::<BlockStandardLighting>();

        nodemat.meta()
    }
}
