use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};

pub struct OpacityClipShader;

impl OpacityClipShader {
    pub const KEY: &'static str = "OpacityClipShader";

    pub fn create(infos: &NodeMaterialBlocks) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./opacity_clip.frag"));

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from(pi_scene_shell::prelude::S_VEC3),
                    name: Atom::from(pi_scene_shell::prelude::S_V_NORMAL),
                },
                Varying { 
                    format: Atom::from(pi_scene_shell::prelude::S_VEC3),
                    name: Atom::from(S_V_POS),
                },
                Varying {
                    format: Atom::from(S_VEC2),
                    name: Atom::from(pi_scene_shell::prelude::S_V_UV),
                },
                Varying { 
                    format: Atom::from(S_VEC4),
                    name: Atom::from(pi_scene_shell::prelude::S_V_COLOR),
                },
            ]
        );

        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uMainAtlas"), [11., 11., 0., 0.], true));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uOpacityAtlas"), [11., 11., 0., 0.], true));
        nodemat.values.vec4_list.push(UniformPropertyVec4(Atom::from("uEmissionAtlas"), [11., 11., 0., 0.], true));

        nodemat.include(&Atom::from(BlockColorGray::KEY), infos);
        nodemat.include(&Atom::from(BlockTextureChannel::KEY), infos);
        nodemat.include(&Atom::from(BlockUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockUVAtlas::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockCutoff::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacity::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockEmissiveTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockEmissiveTextureUVOffsetSpeed::KEY), infos);

        nodemat.meta()
    }
}
