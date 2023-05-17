use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};

pub struct OpacityClipShader;

impl OpacityClipShader {
    pub const KEY: &'static str = "OpacityClipShader";

    pub const KEY_CLIP: &'static str = "uOpacityClip";

    pub fn create(infos: &NodeMaterialBlocks) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./opacity_clip.frag"));

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

        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_CLIP), 0.5));

        nodemat.include(BlockColorGray::KEY, infos);
        nodemat.include(BlockTextureChannel::KEY, infos);
        nodemat.include(BlockUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockMainTexture::KEY, infos);
        nodemat.include(BlockMainTextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockOpacityTexture::KEY, infos);
        nodemat.include(BlockOpacityTextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockEmissiveBase::KEY, infos);

        nodemat.meta()
    }
}
