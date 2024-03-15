
use std::{sync::Arc, ops::{Deref, Range}};

use bevy_ecs::component::Component;
use pi_assets::asset::Handle;
use crate::bindgroup::*;

pub use pi_scene_shell::prelude::*;
use pi_map::smallvecmap::SmallVecMap;

pub enum DrawObj3D {
    Tmp(DrawObjTmp),
    Draw(Arc<DrawObj>)
}

pub struct DrawObjTmp {
    pub pipeline: Option<Handle<RenderRes<RenderPipeline>>>,
    pub bindgroups: BindGroups3D,
    ///
    /// * MAX_VERTEX_BUFFER : 可能的最大顶点Buffer数目, 本地电脑 16
    pub vertices: SmallVecMap<RenderVertices, 3>,
    pub instances: Range<u32>,
    pub vertex: Range<u32>,
    pub indices: Option<RenderIndices>,
}

pub trait TPassData<T: Clone> {
    fn new(val: T) -> Self;
    fn val(&self) -> &T;
}

#[derive(Component)]
pub struct PassReset;

#[derive(Component)]
pub struct PassDirtyBindEffectValue(pub PassTagValue);

#[derive(Component)]
pub struct PassDirtyBindEffectTextures(pub PassTagValue);

#[derive(Component)]
pub struct FlagPassDirtyBindEffectValue;

#[derive(Component)]
pub struct FlagPassDirtyBindEffectTextures;

#[derive(Component)]
pub struct PassTransparent(pub bool);

#[derive(Component)]
pub struct PassModelID(pub Entity);

#[derive(Component)]
pub struct PassRendererID(pub Entity);

#[derive(Component)]
pub struct PassSceneID(pub Entity);

#[derive(Component)]
pub struct PassSceneForSet3(pub Entity);

#[derive(Component)]
pub struct PassViewerID(pub Entity);

#[derive(Component)]
pub struct PassMaterialID(pub Entity);

#[derive(Component)]
pub struct PassGeometryID(pub Entity);

#[derive(Component)]
pub struct PassPipelineStateDirty;

#[derive(Component)]
pub struct PassDrawDirty;

pub trait TPass: Default {
    const TAG: PassTag;
}

#[derive(Component, Default)]
pub struct Pass01;
impl TPass for Pass01 {
    const TAG: PassTag = PassTag::PASS_TAG_01;
}

#[derive(Component, Default)]
pub struct Pass02;
impl TPass for Pass02 {
    const TAG: PassTag = PassTag::PASS_TAG_02;
}

#[derive(Component, Default)]
pub struct Pass03;
impl TPass for Pass03 {
    const TAG: PassTag = PassTag::PASS_TAG_03;
}

#[derive(Component, Default)]
pub struct Pass04;
impl TPass for Pass04 {
    const TAG: PassTag = PassTag::PASS_TAG_04;
}

#[derive(Component, Default)]
pub struct Pass05;
impl TPass for Pass05 {
    const TAG: PassTag = PassTag::PASS_TAG_05;
}

#[derive(Component, Default)]
pub struct Pass06;
impl TPass for Pass06 {
    const TAG: PassTag = PassTag::PASS_TAG_06;
}

#[derive(Component, Default)]
pub struct Pass07;
impl TPass for Pass07 {
    const TAG: PassTag = PassTag::PASS_TAG_07;
}

#[derive(Component, Default)]
pub struct Pass08;
impl TPass for Pass08 {
    const TAG: PassTag = PassTag::PASS_TAG_08;
}

// #[derive(Component, Default)]
// pub struct Pass09;
// impl TPass for Pass09 {
//     const TAG: PassTag = PassTag::PASS_TAG_09;
// }

// #[derive(Component, Default)]
// pub struct Pass10;
// impl TPass for Pass10 {
//     const TAG: PassTag = PassTag::PASS_TAG_10;
// }

// #[derive(Component, Default)]
// pub struct Pass11;
// impl TPass for Pass11 {
//     const TAG: PassTag = PassTag::PASS_TAG_11;
// }

// #[derive(Component, Default)]
// pub struct Pass12;
// impl TPass for Pass12 {
//     const TAG: PassTag = PassTag::PASS_TAG_12;
// }

pub trait TPassID {
    const TAG: PassTag;
    fn check(val: PassTagValue) -> bool { let tag = Self::TAG.deref().clone(); tag & val == tag  }
    fn new(id: Entity) -> Self;
    fn id(&self) -> Entity;
}

#[derive(Component)]
pub struct PassID01(pub Entity);
impl TPassID for PassID01 {
    const TAG: PassTag = PassTag::PASS_TAG_01;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

#[derive(Component)]
pub struct PassID02(pub Entity);
impl TPassID for PassID02 {
    const TAG: PassTag = PassTag::PASS_TAG_02;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

#[derive(Component)]
pub struct PassID03(pub Entity);
impl TPassID for PassID03 {
    const TAG: PassTag = PassTag::PASS_TAG_03;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

#[derive(Component)]
pub struct PassID04(pub Entity);
impl TPassID for PassID04 {
    const TAG: PassTag = PassTag::PASS_TAG_04;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

#[derive(Component)]
pub struct PassID05(pub Entity);
impl TPassID for PassID05 {
    const TAG: PassTag = PassTag::PASS_TAG_05;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

#[derive(Component)]
pub struct PassID06(pub Entity);
impl TPassID for PassID06 {
    const TAG: PassTag = PassTag::PASS_TAG_06;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

#[derive(Component)]
pub struct PassID07(pub Entity);
impl TPassID for PassID07 {
    const TAG: PassTag = PassTag::PASS_TAG_07;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

#[derive(Component)]
pub struct PassID08(pub Entity);
impl TPassID for PassID08 {
    const TAG: PassTag = PassTag::PASS_TAG_08;
    fn new(id: Entity) -> Self { Self(id) }
    fn id(&self) -> Entity { self.0.clone() }
}

// #[derive(Component)]
// pub struct PassID09(pub Entity);
// impl TPassID for PassID09 {
//     const TAG: PassTag = PassTag::PASS_TAG_09;
//     fn new(id: Entity) -> Self { Self(id) }
//     fn id(&self) -> Entity { self.0.clone() }
// }

// #[derive(Component)]
// pub struct PassID10(pub Entity);
// impl TPassID for PassID10 {
//     const TAG: PassTag = PassTag::PASS_TAG_10;
//     fn new(id: Entity) -> Self { Self(id) }
//     fn id(&self) -> Entity { self.0.clone() }
// }

// #[derive(Component)]
// pub struct PassID11(pub Entity);
// impl TPassID for PassID11 {
//     const TAG: PassTag = PassTag::PASS_TAG_11;
//     fn new(id: Entity) -> Self { Self(id) }
//     fn id(&self) -> Entity { self.0.clone() }
// }

// #[derive(Component)]
// pub struct PassID12(pub Entity);
// impl TPassID for PassID12 {
//     const TAG: PassTag = PassTag::PASS_TAG_12;
//     fn new(id: Entity) -> Self { Self(id) }
//     fn id(&self) -> Entity { self.0.clone() }
// }

#[derive(Default, Clone, Component)]
pub struct PassBlend(pub Option<bool>);
impl TPassData<Option<bool>> for PassBlend {
    fn new(val: Option<bool>) -> Self { Self(val) }
    fn val(&self) -> &Option<bool> { &self.0 }
}

#[derive(Default, Clone, Component)]
pub struct PassColorFormat(pub Option<wgpu::TextureFormat>);
impl TPassData<Option<wgpu::TextureFormat>> for PassColorFormat {
    fn new(val: Option<wgpu::TextureFormat>) -> Self { Self(val) }
    fn val(&self) -> &Option<wgpu::TextureFormat> { &self.0 }
}


#[derive(Default, Clone, Component)]
pub struct PassDepthFormat(pub Option<wgpu::TextureFormat>);
impl TPassData<Option<wgpu::TextureFormat>> for PassDepthFormat {
    fn new(val: Option<wgpu::TextureFormat>) -> Self { Self(val) }
    fn val(&self) -> &Option<wgpu::TextureFormat> { &self.0 }
}

/// * 标识物体 已准备好的 Passs
/// * 材质没有纹理时 在使用材质时即准备好
/// * 材质有纹理时 在纹理准备好时才准备好
#[derive(Component)]
pub struct PassEffectReady(pub Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>);
impl TPassData<Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>> for PassEffectReady {
    fn new(val: Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShaderMeta, Handle<ShaderEffectMeta>)> { &self.0 }
}

#[derive(Component)]
pub struct PassBindEffectValue(pub Option<Arc<ShaderBindEffectValue>>);
impl TPassData<Option<Arc<ShaderBindEffectValue>>> for PassBindEffectValue {
    fn new(val: Option<Arc<ShaderBindEffectValue>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<ShaderBindEffectValue>> { &self.0 }
}

#[derive(Component)]
pub struct PassBindEffectTextures(pub Option<EffectTextureSamplers>);
impl TPassData<Option<EffectTextureSamplers>> for PassBindEffectTextures {
    fn new(val: Option<EffectTextureSamplers>) -> Self { Self(val) }
    fn val(&self) -> &Option<EffectTextureSamplers> { &self.0 }
}


/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone, Component)]
pub struct PassBindGroupScene(pub Option<Arc<BindGroupScene>>);
impl TPassData<Option<Arc<BindGroupScene>>> for PassBindGroupScene {
    fn new(val: Option<Arc<BindGroupScene>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<BindGroupScene>> { &self.0 }
}

/// * Set1
/// * 更新依赖: BindModel, BindEffectValues
#[derive(Default, Clone, Component)]
pub struct PassBindGroupModel(pub Option<Arc<BindGroupModel>>);
impl TPassData<Option<Arc<BindGroupModel>>> for PassBindGroupModel {
    fn new(val: Option<Arc<BindGroupModel>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<BindGroupModel>> { &self.0 }
}

/// * Set2
/// * 更新依赖: BindTextureSamplers
#[derive(Default, Clone, Component)]
pub struct PassBindGroupTextureSamplers(pub Option<Arc<BindGroupTextureSamplers>>);
impl TPassData<Option<Arc<BindGroupTextureSamplers>>> for PassBindGroupTextureSamplers {
    fn new(val: Option<Arc<BindGroupTextureSamplers>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<BindGroupTextureSamplers>> { &self.0 }
}

/// * Set3
/// * 更新依赖: BindGroupLightingShadow
#[derive(Default, Clone, Component)]
pub struct PassBindGroupLightingShadow(pub Option<Arc<BindGroupSetExtend>>);
impl TPassData<Option<Arc<BindGroupSetExtend>>> for PassBindGroupLightingShadow {
    fn new(val: Option<Arc<BindGroupSetExtend>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<BindGroupSetExtend>> { &self.0 }
}

#[derive(Default, Clone, Component)]
pub struct RecordPassDraw(pub [Option<ObjectID>; 8]);

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone, Component)]
pub struct PassBindGroups(pub Option<BindGroups3D>);
impl TPassData<Option<BindGroups3D>> for PassBindGroups {
    fn new(val: Option<BindGroups3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroups3D> { &self.0 }
}

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone, Component)]
pub struct PassShader(pub Option<Handle<Shader3D>>);
impl TPassData<Option<Handle<Shader3D>>> for PassShader {
    fn new(val: Option<Handle<Shader3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Handle<Shader3D>> { &self.0 }
}
impl From<(Handle<Shader3D>, Option<()>)> for PassShader {
    fn from(value: (Handle<Shader3D>, Option<()>)) -> Self {
        Self(Some(value.0))
    }
}

#[derive(Default, Clone, Component)]
pub struct PassPipelineKey(pub Option<KeyPipeline3D>);
impl TPassData<Option<KeyPipeline3D>> for PassPipelineKey {
    fn new(val: Option<KeyPipeline3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<KeyPipeline3D> { &self.0 }
}


#[derive(Default, Clone, Component)]
pub struct PassPipeline(pub Option<Pipeline3DUsage>);
impl TPassData<Option<Pipeline3DUsage>> for PassPipeline {
    fn new(val: Option<Pipeline3DUsage>) -> Self { Self(val) }
    fn val(&self) -> &Option<Pipeline3DUsage> { &self.0 }
}
impl From<(Pipeline3DUsage, Option<()>)> for PassPipeline {
    fn from(value: (Pipeline3DUsage, Option<()>)) -> Self {
        Self(Some(value.0))
    }
}
impl PassPipeline {
    pub fn key(&self) -> u64 {
        if let Some(val) = &self.0 {
            let key = val.key();
            *key
        } else {
            u64::MAX
        }
    }
}
#[derive(Component)]
pub struct PassDraw(pub Option<DrawObj3D>);
impl PassDraw {
    pub fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    pub fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}

// #[derive(Deref, DerefMut, Resource)]
// pub struct AssetDataCenterShader3D(pub AssetDataCenter<KeyShader3D, Shader3D, ()>);
// impl AssetDataCenterShader3D {
//     pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
//         Self(AssetDataCenter::new(ref_garbage, capacity, timeout))
//     }
// }
// #[derive(Default, Deref, DerefMut, Resource)]
// pub struct AssetLoaderShader3D(pub AssetLoader<KeyShader3D, ObjectID, Shader3D, ()>);

// #[derive(Deref, DerefMut, Resource)]
// pub struct AssetDataCenterPipeline3D(pub AssetDataCenter<u64, Pipeline3D, ()>);
// impl AssetDataCenterPipeline3D {
//     pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
//         Self(AssetDataCenter::new(ref_garbage, capacity, timeout))
//     }
// }
// #[derive(Default, Deref, DerefMut, Resource)]
// pub struct AssetLoaderPipeline3D(pub AssetLoader<u64, ObjectID, Pipeline3D, ()>);


