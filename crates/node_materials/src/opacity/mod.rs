use crate::{base::TNodeMaterialBlock, common::BlockTextureChannel};
use pi_engine_shell::prelude::*;

pub struct BlockOpacity;
impl BlockOpacity {
    pub const KEY_ALPHA: &'static str = "uOpacity";
}
impl TNodeMaterialBlock for BlockOpacity {
    const KEY: &'static str = "OPACITY";
    const FS_DEFINED: &'static str = "
float opacity() {
    return uOpacity;
}
    ";

    const VS_DEFINED: &'static str = "";

    fn float() -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(Atom::from(Self::KEY_ALPHA), 1.0)
        ]
    }
}


pub struct BlockOpacityTexture;
impl BlockOpacityTexture {
    pub const KEY_LEVEL: &'static str = "uOpacityLevel";
    pub const KEY_TILLOFF: &'static str = "uOpacityTilloff";
    pub const KEY_CHANNEL: &'static str = "uOpacityChannel";
    pub const KEY_TEX: &'static str = "_OpacityTex";
}
impl TNodeMaterialBlock for BlockOpacityTexture {
    const KEY: &'static str = "OPACITY_TEXTURE";

    const FS_DEFINED: &'static str = include_str!("./opacity.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_TILLOFF), [1., 1., 0., 0.])
        ]
    }

    fn float() -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(Atom::from(Self::KEY_LEVEL), 1.0)
        ]
    }

    fn uint() -> Vec<UniformPropertyUint> {
        vec![
            UniformPropertyUint(Atom::from(Self::KEY_CHANNEL), BlockTextureChannel::CHANNEL_A)
        ]
    }

    fn textures() -> Vec<UniformTexture2DDesc> {
        vec![
            UniformTexture2DDesc::new(
                UniformPropertyName::from(Self::KEY_TEX),
                wgpu::TextureSampleType::Float { filterable: true },
                wgpu::TextureViewDimension::D2,
                false,
                EShaderStage::FRAGMENT,
                EDefaultTexture::White
            )
        ]
    }
}

pub struct BlockOpacityTextureUVOffsetSpeed;
impl BlockOpacityTextureUVOffsetSpeed {
    pub const KEY_PARAM: &'static str = "uOpacityUVOS";
}
impl TNodeMaterialBlock for BlockOpacityTextureUVOffsetSpeed {
    const KEY: &'static str = "OPACITY_TEXTURE_UVOS";

    const FS_DEFINED: &'static str = "";

    const VS_DEFINED: &'static str = "";

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [0., 0.])
        ]
    }
}


pub struct BlockOpacity2Texture;
impl BlockOpacity2Texture {
    pub const KEY_LEVEL: &'static str = "uOpacity2Level";
    pub const KEY_TILLOFF: &'static str = "uOpacity2Tilloff";
    pub const KEY_CHANNEL: &'static str = "uOpacity2Channel";
    pub const KEY_TEX: &'static str = "_Opacity2Tex";
}
impl TNodeMaterialBlock for BlockOpacity2Texture {
    const KEY: &'static str = "OPACITY2_TEXTURE";

    const FS_DEFINED: &'static str = include_str!("./opacity_second.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_TILLOFF), [1., 1., 0., 0.])
        ]
    }

    fn float() -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(Atom::from(Self::KEY_LEVEL), 1.0)
        ]
    }

    fn uint() -> Vec<UniformPropertyUint> {
        vec![
            UniformPropertyUint(Atom::from(Self::KEY_CHANNEL), BlockTextureChannel::CHANNEL_A)
        ]
    }

    fn textures() -> Vec<UniformTexture2DDesc> {
        vec![
            UniformTexture2DDesc::new(
                UniformPropertyName::from(Self::KEY_TEX),
                wgpu::TextureSampleType::Float { filterable: true },
                wgpu::TextureViewDimension::D2,
                false,
                EShaderStage::FRAGMENT,
                EDefaultTexture::White
            )
        ]
    }
}

pub struct BlockOpacity2TextureUVOffsetSpeed;
impl BlockOpacity2TextureUVOffsetSpeed {
    pub const KEY_PARAM: &'static str = "uOpacity2UVOS";
}
impl TNodeMaterialBlock for BlockOpacity2TextureUVOffsetSpeed {
    const KEY: &'static str = "OPACITY2_TEXTURE_UVOS";

    const FS_DEFINED: &'static str = "";

    const VS_DEFINED: &'static str = "";

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [0., 0.])
        ]
    }
}
