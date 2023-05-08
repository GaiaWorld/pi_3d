use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};

pub struct TwoOpacityMixShader;

impl TwoOpacityMixShader {
    pub const KEY: &'static str = "TwoOpacityMixShader";

    pub const KEY_MIX_CONTROL: &'static str = "uTwoOpacityMixControl";
    pub const KEY_MIX_CHANNEL: &'static str = "uTwoOpacityMixChannel";
    

    pub fn create(infos: &NodeMaterialBlocks) -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("\r\nlayout(location = 0) out vec4 gl_FragColor; \r\n");

        nodemat.vs = String::from(include_str!("../base.vert"));
        nodemat.fs = String::from(include_str!("./two_opacity_mix.frag"));

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

        nodemat.values.float_list.push(
            UniformPropertyFloat(Atom::from(Self::KEY_MIX_CONTROL), 0.5)  
        );

        nodemat.values.int_list.push(
            UniformPropertyInt(Atom::from(Self::KEY_MIX_CHANNEL), BlockTextureChannel::CHANNEL_A)  
        );

        nodemat.include(BlockFloat::KEY, infos);
        nodemat.include(BlockColorGray::KEY, infos);
        nodemat.include(BlockTextureChannel::KEY, infos);
        nodemat.include(BlockFresnel::KEY, infos);
        nodemat.include(BlockViewDirection::KEY, infos);
        nodemat.include(BlockUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockMainTexture::KEY, infos);
        nodemat.include(BlockMainTextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockOpacityTexture::KEY, infos);
        nodemat.include(BlockOpacityTextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockOpacity2Texture::KEY, infos);
        nodemat.include(BlockOpacity2TextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockMixTexture::KEY, infos);
        nodemat.include(BlockMixTextureUVOffsetSpeed::KEY, infos);
        nodemat.include(BlockOpacityFresnel::KEY, infos);
        nodemat.include(BlockEmissiveBase::KEY, infos);
        nodemat.include(BlockEmissiveFresnel::KEY, infos);
        
        nodemat.meta()
    }
}
