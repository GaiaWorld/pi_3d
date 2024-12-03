
use pi_assets::asset::{Asset, Size};
use pi_bevy_asset::{AssetCapacity, TAssetCapacity};
use pi_render::{renderer::shader::*, asset::ASSET_SIZE_FOR_UNKOWN};
use crate::bind_groups::*;

use super::{BindDefine, EBuildinVertexAtribute, ERenderAlignment, ERenderAlignmentForShader, EVertexAttribute, KeyShaderFromAttributes, ShaderEffectMeta};

pub trait TShaderAttributesCode {
    fn define_code(&self, location: u32) -> String;
    fn running_code(&self, meta: &ShaderEffectMeta) -> String;
}


pub trait TShaderBlockCode {
    fn vs_define_code(&self) -> String;
    fn fs_define_code(&self) -> String;
    fn vs_running_code(&self, meta: &ShaderEffectMeta) -> String;
    fn fs_running_code(&self) -> String;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EKeyShader3DSetBlock {
    Scene(KeyShaderSetScene),
    Model(KeyShaderSetModel),
    TextureSampler(KeyShaderSetTextureSamplers),
    Other(u64),
}
impl TKeyShaderSetBlock for EKeyShader3DSetBlock {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyShader3D {
    pub key_meta: pi_atom::Atom,
    pub key_attributes: KeyShaderFromAttributes,
    pub bind_defines: BindDefine,
    pub renderalignment: ERenderAlignmentForShader,
}

// pub type Shader3D = Shader<4, EKeyShader3DSetBlock>;
pub struct Shader3D {
    pub vs: pi_render::rhi::ShaderModule,
    pub vs_point: &'static str,
    pub fs: pi_render::rhi::ShaderModule,
    pub fs_point: &'static str,
}

impl Asset for Shader3D {
    type Key = KeyShader3D;
}

impl Size for Shader3D {
    fn size(&self) -> usize {
        ASSET_SIZE_FOR_UNKOWN
    }
}
impl TAssetCapacity for Shader3D {
	const ASSET_TYPE: &'static str = "SHADER_3D";
	fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 64 * 1024, max: 1, timeout: 10 * 1000 }
	}
}