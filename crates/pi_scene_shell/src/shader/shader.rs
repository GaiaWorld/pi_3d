
use pi_assets::asset::{Asset, Size};
use pi_bevy_asset::{AssetCapacity, TAssetCapacity};
use pi_render::{renderer::{
        shader::*,
        attributes::{KeyShaderFromAttributes, EVertexAttribute, EBuildinVertexAtribute}
    }, asset::ASSET_SIZE_FOR_UNKOWN};
use crate::bind_groups::*;

use super::{BindDefine, ERenderAlignment, ERenderAlignmentForShader, ShaderEffectMeta};

pub trait TShaderAttributesCode {
    fn define_code(&self, location: u32) -> String;
    fn running_code(&self, meta: &ShaderEffectMeta) -> String;
}

impl TShaderAttributesCode for EVertexAttribute {
    fn define_code(&self, location: u32) -> String {
        let mut result = String::from("layout(location = ");
        result += location.to_string().as_str();
        result += ") in ";
        result += self.kind().as_str();
        result += crate::prelude::S_SPACE;

        match self {
            EVertexAttribute::Buildin(_) => result += "V",
            EVertexAttribute::Custom(_) => {},
        }
        result += self.var_code();
        result += ";\r\n";

        result
    }
    fn running_code(&self, meta: &ShaderEffectMeta) -> String {
        let mut result = String::from("");

        match self {
            EVertexAttribute::Buildin(val) => {
                match val {
                    EBuildinVertexAtribute::Color4 => {},
                    EBuildinVertexAtribute::UV => {},
                    EBuildinVertexAtribute::Normal => {},
                    _ => { result += val.kind().as_str(); result += crate::prelude::S_SPACE; },
                }
                result += val.var_code(); result += " = V"; result += val.var_code(); result += ";\r\n";

                match val {
                    EBuildinVertexAtribute::InsWorldRow4 => {
                        result += Self::matrix().as_str();
                    },
                    EBuildinVertexAtribute::Trail => {
                        result += Self::trail().as_str();
                    },
                    EBuildinVertexAtribute::TrailBillboard => {
                        result += Self::trail_billboard().as_str();
                    },
                    _ => {},
                }
            },
            EVertexAttribute::Custom(val) => {
                if let Some(uniform) = val.foruniform() {
                    if meta.uniforms.query_instance(uniform) {
                        result += val.vs_running_code(); result += crate::prelude::S_BREAK;
                    }
                } else {
                    result += val.vs_running_code(); result += crate::prelude::S_BREAK;
                }
            },
        }

        result
    }
}

pub trait TShaderBlockCode {
    fn vs_define_code(&self) -> String;
    fn fs_define_code(&self) -> String;
    fn vs_running_code(&self, meta: &ShaderEffectMeta) -> String;
    fn fs_running_code(&self) -> String;
}

impl TShaderBlockCode for KeyShaderFromAttributes {
    fn vs_define_code(&self) -> String {
        let mut result = String::from("");
        let mut idx = 0;
        self.0.iter().for_each(|attr| {
            result += attr.define_code(idx).as_str();
            idx += 1;
        });

        result
    }

    fn fs_define_code(&self) -> String {
        String::from("")
    }

    fn vs_running_code(&self, meta: &ShaderEffectMeta) -> String {
        let mut result = String::from("");
        self.0.iter().for_each(|attr| {
            result += attr.running_code(meta).as_str();
        });

        result
    }

    fn fs_running_code(&self) -> String {
        String::from("")
    }
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
        AssetCapacity { flag: false, min: 64 * 1024, max: 128 * 1024, timeout: 10 * 1000 }
	}
}