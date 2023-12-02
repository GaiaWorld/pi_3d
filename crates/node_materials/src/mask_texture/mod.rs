use crate::{base::TNodeMaterialBlock};
use pi_engine_shell::prelude::*;

pub struct BlockMaskTexture;
impl BlockMaskTexture {
    pub const KEY_TILLOFF: &'static str = "uMaskTilloff";
    pub const KEY_TEX: &'static str = "_MaskTex";
}
impl TNodeMaterialBlock for BlockMaskTexture {
    const KEY: &'static str = "MASK_TEXTURE";

    const FS_DEFINED: &'static str = include_str!("./mask_texture.glsl");

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
                wgpu::TextureViewDimension::D2,
                false,
                EShaderStage::FRAGMENT,
                EDefaultTexture::White
            )
        ]
    }
}

pub struct BlockMaskTextureUVOffsetSpeed;
impl BlockMaskTextureUVOffsetSpeed {
    pub const KEY_PARAM: &'static str = "uMaskUVOS";
}
impl TNodeMaterialBlock for BlockMaskTextureUVOffsetSpeed {
    const KEY: &'static str = "MASK_TEXTURE_UVOS";

    const FS_DEFINED: &'static str = "";

    const VS_DEFINED: &'static str = "";

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [0., 0.])
        ]
    }
}
