
use pi_atom::Atom;

use pi_render::renderer::{buildin_data::EDefaultTexture, shader_stage::EShaderStage};

use super::{uniform_texture::UniformTexture2DDesc, ShaderVarUniform};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ESkinBonesPerVertex {
    One,
    Two,
    Three,
    Four,
}
impl ESkinBonesPerVertex {
    const DEFINE: &'static str = include_str!("./skin/tex_define.hlsl");
    const RUNNING_1: &'static str = include_str!("./skin/running_1.hlsl");
    const RUNNING_2: &'static str = include_str!("./skin/running_2.hlsl");
    const RUNNING_3: &'static str = include_str!("./skin/running_3.hlsl");
    const RUNNING_4: &'static str = include_str!("./skin/running_4.hlsl");
    const TEX_RUNNING_1: &'static str = include_str!("./skin/tex_running_1.hlsl");
    const TEX_RUNNING_2: &'static str = include_str!("./skin/tex_running_2.hlsl");
    const TEX_RUNNING_3: &'static str = include_str!("./skin/tex_running_3.hlsl");
    const TEX_RUNNING_4: &'static str = include_str!("./skin/tex_running_4.hlsl");
    pub fn define_code_for_ubo(&self) -> String {
        String::from("")
    }
    pub fn running_code_for_ubo(&self) -> String {
        match self {
            ESkinBonesPerVertex::One =>  {
                String::from(Self::RUNNING_1)
            },
            ESkinBonesPerVertex::Two =>  {
                String::from(Self::RUNNING_2)
            },
            ESkinBonesPerVertex::Three =>  {
                String::from(Self::RUNNING_3)
            },
            ESkinBonesPerVertex::Four => {
                String::from(Self::RUNNING_4)
            },
        }
    }
    pub fn define_code_for_tex(&self) -> String {
        String::from(Self::DEFINE)
    }
    pub fn running_code_for_tex(&self) -> String {
        match self {
            ESkinBonesPerVertex::One =>  {
                String::from(Self::TEX_RUNNING_1)
            },
            ESkinBonesPerVertex::Two =>  {
                String::from(Self::TEX_RUNNING_2)
            },
            ESkinBonesPerVertex::Three =>  {
                String::from(Self::TEX_RUNNING_3)
            },
            ESkinBonesPerVertex::Four => {
                String::from(Self::TEX_RUNNING_4)
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ESkinCode {
    None,
    UBO(ESkinBonesPerVertex, EBoneCount, u16),
    RowTexture(ESkinBonesPerVertex),
    FramesTexture(ESkinBonesPerVertex),
}
impl Default for ESkinCode {
    fn default() -> Self {
        Self::None
    }
}
impl ESkinCode {
    pub fn define_code(&self) -> String {
        match self {
            ESkinCode::None => String::from(""),
            ESkinCode::UBO(temp, _, _) => temp.define_code_for_ubo(),
            ESkinCode::RowTexture(temp) => temp.define_code_for_tex(),
            ESkinCode::FramesTexture(temp) => temp.define_code_for_tex(),
        }
    }
    pub fn running_code(&self) -> String {
        match self {
            ESkinCode::None => String::from(""),
            ESkinCode::UBO(temp, _, _) => temp.running_code_for_ubo(),
            ESkinCode::RowTexture(temp) => temp.running_code_for_tex(),
            ESkinCode::FramesTexture(temp) => temp.running_code_for_tex(),
        }
    }
    pub fn uniform_desc_tex() -> UniformTexture2DDesc {
        UniformTexture2DDesc::new(
            Atom::from(ShaderVarUniform::BONE_TEX),
            crate::shader::ESamplerType::Float,
            false,
            EShaderStage::VERTEX,
            EDefaultTexture::White,
        )
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EBoneCount {
    N16 = 16,
    N32 = 32,
    N64 = 64,
    N128 = 128,
    N256 = 256,
}
impl EBoneCount {
    pub fn new(bone_count: u8) -> Self {
        if bone_count <= 16 {
            Self::N16
        }
        else if bone_count <= 32 {
            Self::N32
        }
        else if bone_count <= 64 {
            Self::N64
        }
        else if bone_count <= 128 {
            Self::N128
        }
        else {
            Self::N256
        }
    }
    pub fn use_bytes(&self) -> usize {
        match self {
            EBoneCount::N16 => 16 * 16 * 4,
            EBoneCount::N32 => 32 * 16 * 4,
            EBoneCount::N64 => 64 * 16 * 4,
            EBoneCount::N128 => 128 * 16 * 4,
            EBoneCount::N256 => 256 * 16 * 4,
        }
    }
    pub fn count(&self) -> u32 {
        match self {
            EBoneCount::N16     =>  16,
            EBoneCount::N32     =>  32,
            EBoneCount::N64     =>  64,
            EBoneCount::N128    => 128,
            EBoneCount::N256    => 256,
        }
    }
}