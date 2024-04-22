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
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("./standard.vert"));
        nodemat.fs = String::from(include_str!("./standard.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from("vec3"),
                    name: Atom::from("v_normal"),
                },
                Varying { 
                    format: Atom::from("vec4"),
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
