use crate::{base::TNodeMaterialBlock, common::BlockTextureChannel};
use pi_engine_shell::prelude::*;

pub struct BlockMixTexture;
impl BlockMixTexture {
    pub const KEY_TILLOFF: &'static str = "uMixTilloff";
    pub const KEY_TEX: &'static str = "_MixTex";
}
impl TNodeMaterialBlock for BlockMixTexture {
    const KEY: &'static str = "MIX_TEXTURE";

    const FS_DEFINED: &'static str = include_str!("./mix_texture.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_TILLOFF), [1., 1., 0., 0.])
        ]
    }


    fn textures() -> Vec<UniformTexture2DDesc> {
        vec![
            UniformTexture2DDesc::new(
                UniformPropertyName::from(Self::KEY_TEX),
                wgpu::TextureSampleType::Float { filterable: true },
                false,
                EShaderStage::FRAGMENT,
                EDefaultTexture::White
            )
        ]
    }
}

pub struct BlockMixTextureUVOffsetSpeed;
impl BlockMixTextureUVOffsetSpeed {
    pub const KEY_PARAM: &'static str = "uMixUVOS";
}
impl TNodeMaterialBlock for BlockMixTextureUVOffsetSpeed {
    const KEY: &'static str = "MIX_TEXTURE_UVOS";

    const FS_DEFINED: &'static str = "";

    const VS_DEFINED: &'static str = "";

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [0., 0.])
        ]
    }
}
