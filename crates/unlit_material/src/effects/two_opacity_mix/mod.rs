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

        nodemat.values.uint_list.push(
            UniformPropertyUint(Atom::from(Self::KEY_MIX_CHANNEL), BlockTextureChannel::CHANNEL_A)  
        );

        nodemat.include(&Atom::from(BlockFloat::KEY), infos);
        nodemat.include(&Atom::from(BlockColorGray::KEY), infos);
        nodemat.include(&Atom::from(BlockTextureChannel::KEY), infos);
        nodemat.include(&Atom::from(BlockFresnel::KEY), infos);
        nodemat.include(&Atom::from(BlockViewDirection::KEY), infos);
        nodemat.include(&Atom::from(BlockUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockMainTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacity::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacity2Texture::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacity2TextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockMixTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockMixTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockOpacityFresnel::KEY), infos);
        nodemat.include(&Atom::from(BlockEmissiveTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockEmissiveTextureUVOffsetSpeed::KEY), infos);
        nodemat.include(&Atom::from(BlockEmissiveFresnel::KEY), infos);
        nodemat.include(&Atom::from(BlockMaskTexture::KEY), infos);
        nodemat.include(&Atom::from(BlockMaskTextureUVOffsetSpeed::KEY), infos);
        
        nodemat.meta()
    }
}
