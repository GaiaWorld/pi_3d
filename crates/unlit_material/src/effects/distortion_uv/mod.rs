use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};

pub struct DistortionUVShader;

impl DistortionUVShader {
    pub const KEY: &'static str = "DistortionUVShader";

    pub const KEY_MODE: &'static str = "uFlowMode";
    pub const KEY_STRENGTH: &'static str = "uStrength";

    pub fn create(infos: &NodeMaterialBlocks) -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from(S_BREAK) + "layout(location = 0) out vec4 gl_FragColor;" + S_BREAK;

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./distortion_uv.frag"));

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

        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from(Self::KEY_MODE), 0., false));
        nodemat.values.vec2_list.push(UniformPropertyVec2(Atom::from(Self::KEY_STRENGTH), [0.1, 0.1], true));

        nodemat.include(&Atom::from(BlockFloat::KEY), infos);
        nodemat.include(&Atom::from(BlockTextureChannel::KEY), infos);
        nodemat.include(&Atom::from(BlockUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockUVAtlas::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacity::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockMaskTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockMaskTextureUVOffsetSpeed::KEY), infos);
        
        nodemat.meta()
    }
}
