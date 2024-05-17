
use std::{sync::Arc, ops::{Deref, Range}};

use crate::{bindgroup::*, prelude::{EVerteicesMemory, GeometryDesc, GeometryResourceHash, IndiceRenderRange, VertexRenderRange}};

pub use pi_scene_shell::prelude::*;

pub enum DrawObj3D {
    InstanceNotClip(DrawObjTmp),
    Draw(Arc<DrawObj>)
}

#[derive(Clone)]
pub struct DrawObjTmp {
    pub pipeline: u64,
    pub passentity: Entity,
    pub bindgroupshash: BindGroups3DHashResource,
    pub vertexentity: Entity,
    pub vertexhash: GeometryResourceHash,
    pub instance_memory: Option<EVerteicesMemory>,
    pub indice_range: IndiceRenderRange,
    pub vertex_range: VertexRenderRange,
}
impl DrawObjTmp {
    pub fn can_batch_instance_memory(&self, other: &Self, debug: bool, max_combine_bytes: usize) -> bool {
        // if debug {
        //     log::warn!(
        //         "pipeline: {:?}, vertexhash: {:?}, bindgroupshash: {:?}, instance_memory: {:?}",
        //         (self.pipeline , other.pipeline),
        //         (self.vertexhash.0 , other.vertexhash.0),
        //         (self.bindgroupshash.0 , other.bindgroupshash.0),
        //         (self.instance_memory.is_some() , other.instance_memory.is_some())
        //     );
        // }
        if self.indice_range.0.is_none() || other.indice_range.0.is_none() { return false; }
        if self.vertex_range.0.is_none() || other.vertex_range.0.is_none() { return false; }
        if self.pipeline == other.pipeline
            && self.vertexhash == other.vertexhash
            && self.bindgroupshash == other.bindgroupshash
        {
            match (&self.instance_memory, &other.instance_memory) {
                (Some(ins1), Some(ins2)) => {
                    ins1.data.len() + ins2.data.len() < max_combine_bytes
                },
                _ => false,
            }
        } else {
            false
        }
    }
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


#[derive(Component)]
pub struct PassIDs(pub [Entity;8]);

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
pub struct PassBindGroups(pub Option<(BindGroups3D, BindGroups3DHashResource)>);
impl PassBindGroups {
    pub fn new(val: Option<BindGroups3D>) -> Self {
        if let Some(val) = val {
            let hash = BindGroups3DHashResource::from(&val);
            Self(Some((val, hash)))
        } else {
            Self(None)
        }
    }
    pub fn val(&self) -> Option<&BindGroups3D> {
        if let Some(val) = &self.0 {
            Some(&val.0)
        } else {
            None
        }
    }
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
pub struct PassDraw(pub Option<DrawObjTmp>);
impl PassDraw {
    pub fn new(val: Option<DrawObjTmp>) -> Self { Self(val) }
    pub fn val(&self) -> &Option<DrawObjTmp> { &self.0 }
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


