use crate::base::TNodeMaterialBlock;
use pi_engine_shell::prelude::*;

pub struct BlockEmissiveTexture;
impl BlockEmissiveTexture {
    pub const KEY_INFO: &'static str = "uEmissiveInfo";
    pub const KEY_TILLOFF: &'static str = "uEmissiveTilloff";
    pub const KEY_TEX: &'static str = "_EmissiveTex";
}
impl TNodeMaterialBlock for BlockEmissiveTexture {
    const KEY: &'static str = "EMISSIVE_TEXTURE";

    const FS_DEFINED: &'static str = include_str!("./emissive_texture.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_INFO), [0., 0., 0., 1.]),
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