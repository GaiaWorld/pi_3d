
use std::sync::Arc;

use pi_assets::asset::*;
use pi_engine_shell::prelude::*;

use crate::pass::*;

use super::base::*;

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

#[derive(Deref, DerefMut, Resource)]
pub struct AssetDataCenterShader3D(pub AssetDataCenter<KeyShader3D, Shader3D, ()>);
impl AssetDataCenterShader3D {
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(AssetDataCenter::new(ref_garbage, capacity, timeout))
    }
}
#[derive(Default, Deref, DerefMut, Resource)]
pub struct AssetLoaderShader3D(pub AssetLoader<KeyShader3D, ObjectID, Shader3D, ()>);

#[derive(Deref, DerefMut, Resource)]
pub struct AssetDataCenterPipeline3D(pub AssetDataCenter<u64, Pipeline3D, ()>);
impl AssetDataCenterPipeline3D {
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(AssetDataCenter::new(ref_garbage, capacity, timeout))
    }
}
#[derive(Default, Deref, DerefMut, Resource)]
pub struct AssetLoaderPipeline3D(pub AssetLoader<u64, ObjectID, Pipeline3D, ()>);
