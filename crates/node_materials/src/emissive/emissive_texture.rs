use crate::base::TNodeMaterialBlock;
use pi_engine_shell::prelude::*;

pub struct BlockEmissiveTexture;
impl BlockEmissiveTexture {
    pub const KEY_INFO: &'static str = "uEmissiveInfo";
    pub const KEY_TILLOFF: &'static str = "uEmissiveTilloff";
    pub const KEY_TEX: &'static str = "_EmissiveTex";
    pub const KEY_TEX_LEVEL: &'static str = "_EmissiveTexLevel";
}
impl TNodeMaterialBlock for BlockEmissiveTexture {
    const KEY: &'static str = "EMISSIVE_TEXTURE";

    const FS_DEFINED: &'static str = include_str!("./emissive_texture.glsl");

    const VS_DEFINED: &'static str = "";

    fn vec4() -> Vec<UniformPropertyVec4> {
        vec![
            UniformPropertyVec4(Atom::from(Self::KEY_TILLOFF), [1., 1., 0., 0.], true),
        ]
    }

    fn vec3() -> Vec<UniformPropertyVec3> {
        vec![
            UniformPropertyVec3(Atom::from(Self::KEY_INFO), [0., 0., 0.], true),
        ]
    }

    fn float() -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(Atom::from(Self::KEY_TEX_LEVEL), 1., true),
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