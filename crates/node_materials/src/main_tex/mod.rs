use crate::base::TNodeMaterialBlock;
use pi_engine_shell::prelude::*;

pub struct BlockMainTexture;
impl BlockMainTexture {
    pub const KEY_COLOR: &'static str = "uMainInfo";
    pub const KEY_TILLOFF: &'static str = "uMainTilloff";
    pub const KEY_TEX: &'static str = "_MainTex";
}
impl TNodeMaterialBlock for BlockMainTexture {
    const KEY: &'static str = "MAIN_TEXTURE";

    const FS_DEFINED: &'static str = include_str!("./main_tex.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_COLOR), [1., 1., 1., 1.]),
            UniformPropertyVec4(Atom::from(Self::KEY_TILLOFF), [1., 1., 0., 0.]),
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


pub struct BlockMainTextureUVOffsetSpeed;
impl BlockMainTextureUVOffsetSpeed {
    pub const KEY_PARAM: &'static str = "uMainUVOS";
}
impl TNodeMaterialBlock for BlockMainTextureUVOffsetSpeed {
    const KEY: &'static str = "MAIN_TEXTURE_UVOS";

    const FS_DEFINED: &'static str = "";

    const VS_DEFINED: &'static str = "";

    fn vec2() -> Vec<UniformPropertyVec2> {
        vec![
            UniformPropertyVec2(Atom::from(Self::KEY_PARAM), [0., 0.])
        ]
    }
}
