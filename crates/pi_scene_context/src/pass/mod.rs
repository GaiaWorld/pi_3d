
use std::sync::Arc;

use pi_assets::asset::Handle;
use pi_engine_shell::object::ObjectID;
use pi_render::{render_3d::{binds::effect_value::ShaderBindEffectValue, bind_groups::{texture_sampler::{EffectTextureSamplers, BindGroupTextureSamplers}, scene::BindGroupScene, model::BindGroupModel}, shader::shader_effect_meta::ShaderEffectMeta}, renderer::shader::KeyShaderMeta};

use crate::geometry::geometry::RenderGeometry;

pub type PassTag = u16;

#[derive(Debug, Clone)]
pub struct PassTagOrders(pub Vec<EPassTag>, pub PassTag);
impl PassTagOrders {
    pub fn new(orders: Vec<EPassTag>) -> Self {
        let mut tag = 0;
        orders.iter().for_each(|item| {
            if tag & item.as_pass() == 0 {
                tag += item.as_pass();
            }
        });

        Self(orders, tag)
    }
}

/// * 渲染 Pass
///   * 每个 Pass 对应一个渲染流程
///   * 每个材质 只对应 一个Pass
///   * example: ShadowCast, DepthPrePass, Opaque, Skybox, Transparent,
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EPassTag {
    ShadowCast,
    Opaque,
    Sky,
    Water,
    AlphaTest,
    Transparent,
    OpaqueExtend,
    TransparentExtend,
}
impl EPassTag {
    pub const PASS_TAG_01: PassTag = 0b0000_0000_0000_0001;
    pub const PASS_TAG_02: PassTag = 0b0000_0000_0000_0010;
    pub const PASS_TAG_03: PassTag = 0b0000_0000_0000_0100;
    pub const PASS_TAG_04: PassTag = 0b0000_0000_0000_1000;
    pub const PASS_TAG_05: PassTag = 0b0000_0000_0001_0000;
    pub const PASS_TAG_06: PassTag = 0b0000_0000_0010_0000;
    pub const PASS_TAG_07: PassTag = 0b0000_0000_0100_0000;
    pub const PASS_TAG_08: PassTag = 0b0000_0000_1000_0000;
    
    pub fn color_format(val: PassTag) -> wgpu::TextureFormat {
        match val {
            Self::PASS_TAG_01 => wgpu::TextureFormat::Rgba16Float,
            _ => wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }
    pub fn depth_format(val: PassTag) -> Option<wgpu::TextureFormat> {
        match val {
            Self::PASS_TAG_01 => Some(wgpu::TextureFormat::Depth32Float),
            _ => Some(wgpu::TextureFormat::Depth32Float),
        }
    }
    pub fn depth_write(val: PassTag) -> bool {
        match val {
            Self::PASS_TAG_01 => true,
            _ => false,
        }
    }
    pub fn depth_compare(val: PassTag) -> Option<wgpu::CompareFunction> {
        match val {
            Self::PASS_TAG_01 => Some(wgpu::CompareFunction::GreaterEqual),
            _ => None,
        }
    }
    pub fn blend(val: PassTag) -> bool {
        match val {
            Self::PASS_TAG_01 => false,
            Self::PASS_TAG_02 => false,
            Self::PASS_TAG_03 => false,
            _ => true,
        }
    }
    pub fn index(&self) -> usize {
        match self {
            EPassTag::ShadowCast => 1,
            EPassTag::Opaque => 2,
            EPassTag::Sky => 3,
            EPassTag::Water => 4,
            EPassTag::AlphaTest => 5,
            EPassTag::Transparent => 6,
            EPassTag::OpaqueExtend => 7,
            EPassTag::TransparentExtend => 8,
        }
    }
    pub fn as_pass(&self) -> PassTag {
        match self {
            EPassTag::ShadowCast => Self::PASS_TAG_01,
            EPassTag::Opaque => Self::PASS_TAG_02,
            EPassTag::Sky => Self::PASS_TAG_03,
            EPassTag::Water => Self::PASS_TAG_04,
            EPassTag::AlphaTest => Self::PASS_TAG_05,
            EPassTag::Transparent => Self::PASS_TAG_06,
            EPassTag::OpaqueExtend => Self::PASS_TAG_07,
            EPassTag::TransparentExtend => Self::PASS_TAG_08,
        }
    }
}

// pub const PASS_TAG_09: TPassTag = 0b0000_0001_0000_0000;
// pub const PASS_TAG_10: TPassTag = 0b0000_0010_0000_0000;
// pub const PASS_TAG_11: TPassTag = 0b0000_0100_0000_0000;
// pub const PASS_TAG_12: TPassTag = 0b0000_1000_0000_0000;
// pub const PASS_TAG_13: TPassTag = 0b0001_0000_0000_0000;
// pub const PASS_TAG_14: TPassTag = 0b0010_0000_0000_0000;
// pub const PASS_TAG_15: TPassTag = 0b0100_0000_0000_0000;
// pub const PASS_TAG_16: TPassTag = 0b1000_0000_0000_0000;

pub trait TPassData<T: Clone> {
    fn new(val: T) -> Self;
    fn val(&self) -> &T;
}

pub struct PassDirtyBindEffectValue(pub PassTag);
pub struct PassDirtyBindEffectTextures(pub PassTag);
pub struct FlagPassDirtyBindEffectValue;
pub struct FlagPassDirtyBindEffectTextures;

pub struct PassSource(pub ObjectID);

pub trait TPass {
    fn new() -> Self;
    const TAG: PassTag;
}
pub struct Pass01;
impl TPass for Pass01 {
    const TAG: PassTag = EPassTag::PASS_TAG_01;
    fn new() -> Self { Self }
}
pub struct Pass02;
impl TPass for Pass02 {
    const TAG: PassTag = EPassTag::PASS_TAG_02;
    fn new() -> Self { Self }
}
pub struct Pass03;
impl TPass for Pass03 {
    const TAG: PassTag = EPassTag::PASS_TAG_03;
    fn new() -> Self { Self }
}
pub struct Pass04;
impl TPass for Pass04 {
    const TAG: PassTag = EPassTag::PASS_TAG_04;
    fn new() -> Self { Self }
}
pub struct Pass05;
impl TPass for Pass05 {
    const TAG: PassTag = EPassTag::PASS_TAG_05;
    fn new() -> Self { Self }
}
pub struct Pass06;
impl TPass for Pass06 {
    const TAG: PassTag = EPassTag::PASS_TAG_06;
    fn new() -> Self { Self }
}
pub struct Pass07;
impl TPass for Pass07 {
    const TAG: PassTag = EPassTag::PASS_TAG_07;
    fn new() -> Self { Self }
}
pub struct Pass08;
impl TPass for Pass08 {
    const TAG: PassTag = EPassTag::PASS_TAG_08;
    fn new() -> Self { Self }
}

pub trait TPassID {
    const TAG: PassTag;
    fn new(id: ObjectID) -> Self;
    fn id(&self) -> ObjectID;
}
pub struct PassID01(pub ObjectID);
impl TPassID for PassID01 {
    const TAG: PassTag = EPassTag::PASS_TAG_01;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}
pub struct PassID02(pub ObjectID);
impl TPassID for PassID02 {
    const TAG: PassTag = EPassTag::PASS_TAG_02;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}
pub struct PassID03(pub ObjectID);
impl TPassID for PassID03 {
    const TAG: PassTag = EPassTag::PASS_TAG_03;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}
pub struct PassID04(pub ObjectID);
impl TPassID for PassID04 {
    const TAG: PassTag = EPassTag::PASS_TAG_04;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}
pub struct PassID05(pub ObjectID);
impl TPassID for PassID05 {
    const TAG: PassTag = EPassTag::PASS_TAG_05;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}
pub struct PassID06(pub ObjectID);
impl TPassID for PassID06 {
    const TAG: PassTag = EPassTag::PASS_TAG_06;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}
pub struct PassID07(pub ObjectID);
impl TPassID for PassID07 {
    const TAG: PassTag = EPassTag::PASS_TAG_07;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}
pub struct PassID08(pub ObjectID);
impl TPassID for PassID08 {
    const TAG: PassTag = EPassTag::PASS_TAG_08;
    fn new(id: ObjectID) -> Self { Self(id) }
    fn id(&self) -> ObjectID { self.0.clone() }
}

#[derive(Default, Clone)]
pub struct PassBlend(pub Option<bool>);
impl TPassData<Option<bool>> for PassBlend {
    fn new(val: Option<bool>) -> Self { Self(val) }
    fn val(&self) -> &Option<bool> { &self.0 }
}

#[derive(Default, Clone)]
pub struct PassColorFormat(pub Option<wgpu::TextureFormat>);
impl TPassData<Option<wgpu::TextureFormat>> for PassColorFormat {
    fn new(val: Option<wgpu::TextureFormat>) -> Self { Self(val) }
    fn val(&self) -> &Option<wgpu::TextureFormat> { &self.0 }
}


#[derive(Default, Clone)]
pub struct PassDepthFormat(pub Option<wgpu::TextureFormat>);
impl TPassData<Option<wgpu::TextureFormat>> for PassDepthFormat {
    fn new(val: Option<wgpu::TextureFormat>) -> Self { Self(val) }
    fn val(&self) -> &Option<wgpu::TextureFormat> { &self.0 }
}

/// * 标识物体 已准备好的 Passs
/// * 材质没有纹理时 在使用材质时即准备好
/// * 材质有纹理时 在纹理准备好时才准备好
pub struct PassReady(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for PassReady {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}

pub struct PassBindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for PassBindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}

pub struct PassBindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for PassBindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}


/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone)]
pub struct PassBindGroupScene(pub Option<Arc<BindGroupScene>>);
impl TPassData<Option<Arc<BindGroupScene>>> for PassBindGroupScene {
    fn new(val: Option<Arc<BindGroupScene>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<BindGroupScene>> { &self.0 }
}

/// * Set1
/// * 更新依赖: BindModel, BindEffectValues
#[derive(Default, Clone)]
pub struct PassBindGroupModel(pub Option<Arc<BindGroupModel>>);
impl TPassData<Option<Arc<BindGroupModel>>> for PassBindGroupModel {
    fn new(val: Option<Arc<BindGroupModel>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<BindGroupModel>> { &self.0 }
}

/// * Set2
/// * 更新依赖: BindTextureSamplers
#[derive(Default, Clone)]
pub struct PassBindGroupTextureSamplers(pub Option<Arc<BindGroupTextureSamplers>>);
impl TPassData<Option<Arc<BindGroupTextureSamplers>>> for PassBindGroupTextureSamplers {
    fn new(val: Option<Arc<BindGroupTextureSamplers>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<BindGroupTextureSamplers>> { &self.0 }
}

#[derive(Default, Clone)]
pub struct RecordPassDraw(pub [Option<ObjectID>; 8]);