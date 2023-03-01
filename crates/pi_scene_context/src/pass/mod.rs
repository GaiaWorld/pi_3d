
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
#[derive(Debug, Clone, Copy)]
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

pub struct Pass01Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass01Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}
pub struct Pass02Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass02Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}
pub struct Pass03Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass03Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}
pub struct Pass04Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass04Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}
pub struct Pass05Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass05Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}
pub struct Pass06Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass06Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}
pub struct Pass07Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass07Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}
pub struct Pass08Geometry(pub Option<RenderGeometry>);
impl TPassData<Option<RenderGeometry>> for Pass08Geometry {
    fn new(val: Option<RenderGeometry>) -> Self { Self(val) }
    fn val(&self) -> &Option<RenderGeometry> { &self.0 }
}

/// * 标识物体 已准备好的 Passs
/// * 材质没有纹理时 在使用材质时即准备好
/// * 材质有纹理时 在纹理准备好时才准备好
pub struct Pass01Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass01Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}
pub struct Pass02Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass02Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}
pub struct Pass03Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass03Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}
pub struct Pass04Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass04Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}
pub struct Pass05Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass05Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}
pub struct Pass06Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass06Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}
pub struct Pass07Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass07Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}
pub struct Pass08Ready(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for Pass08Ready {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}

pub struct Pass01BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass01BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}

pub struct Pass02BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass02BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}
pub struct Pass03BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass03BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}
pub struct Pass04BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass04BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}
pub struct Pass05BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass05BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}
pub struct Pass06BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass06BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}
pub struct Pass07BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass07BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}
pub struct Pass08BindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for Pass08BindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}

pub struct Pass01BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass01BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}
pub struct Pass02BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass02BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}
pub struct Pass03BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass03BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}
pub struct Pass04BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass04BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}
pub struct Pass05BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass05BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}
pub struct Pass06BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass06BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}
pub struct Pass07BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass07BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}
pub struct Pass08BindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for Pass08BindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}



/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone)]
pub struct Pass01BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass01BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass02BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass02BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass03BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass03BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass04BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass04BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass05BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass05BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass06BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass06BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass07BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass07BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass08BindGroupScene(pub Option<BindGroupScene>);
impl TPassData<Option<BindGroupScene>> for Pass08BindGroupScene {
    fn new(val: Option<BindGroupScene>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupScene> { &self.0 }
}

/// * Set1
/// * 更新依赖: BindModel, BindEffectValues
#[derive(Default, Clone)]
pub struct Pass01BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass01BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass02BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass02BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass03BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass03BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass04BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass04BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass05BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass05BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass06BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass06BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass07BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass07BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass08BindGroupModel(pub Option<BindGroupModel>);
impl TPassData<Option<BindGroupModel>> for Pass08BindGroupModel {
    fn new(val: Option<BindGroupModel>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupModel> { &self.0 }
}

/// * Set2
/// * 更新依赖: BindTextureSamplers
#[derive(Default, Clone)]
pub struct Pass01BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass01BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass02BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass02BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass03BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass03BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass04BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass04BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass05BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass05BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass06BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass06BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass07BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass07BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass08BindGroupTextureSamplers(pub Option<BindGroupTextureSamplers>);
impl TPassData<Option<BindGroupTextureSamplers>> for Pass08BindGroupTextureSamplers {
    fn new(val: Option<BindGroupTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroupTextureSamplers> { &self.0 }
}

#[derive(Default, Clone)]
pub struct RecordPassDraw(pub [Option<ObjectID>; 8]);