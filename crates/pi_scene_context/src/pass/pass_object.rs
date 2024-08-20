
use std::{sync::Arc, ops::Range};

use crate::{bindgroup::*, prelude::*};

pub use pi_scene_shell::prelude::*;

pub enum DrawObj3D {
    InstanceNotClip(DrawObjTmp),
    Draw(Arc<DrawObj>)
}

#[derive(Clone)]
pub struct DrawObjInfo {
    pub instance_memory: Option<Arc<EVerteicesInstance>>,
    pub pipeline: u64,
    pub passentity: Entity,
    pub bindgroupshash: BindGroups3DHashResource,
    pub vertexentity: Entity,
    pub vertexhash: GeometryResourceHash,
    pub indice_range: IndiceRenderRange,
    pub vertex_range: VertexRenderRange,
}
impl DrawObjInfo {
    pub fn can_batch_instance_memory(&self, other: &Self, _debug: bool, max_combine_bytes: usize) -> bool {
        // if debug {
        //     log::warn!(
        //         "pipeline: {:?}, vertexhash: {:?}, bindgroupshash: {:?}, instance_memory: {:?}",
        //         (self.pipeline , other.pipeline),
        //         (self.vertexhash.0 , other.vertexhash.0),
        //         (self.bindgroupshash.0 , other.bindgroupshash.0),
        //         (self.instance_memory.is_some() , other.instance_memory.is_some())
        //     );
        // }
        if self.indice_range.is_some() || other.indice_range.is_some() { return false; }
        if self.vertex_range.is_some() || other.vertex_range.is_some() { return false; }
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
impl Drop for DrawObjInfo {
    fn drop(&mut self) {
        self.instance_memory = None;
        log::error!("DrawObjInfo Drop.");
    }
}

#[derive(Clone)]
pub struct DrawTmpRef<'w> {
    pub rendergeo: &'w RenderGeometry,
    pub pipeline: &'w Pipeline3DUsage,
    pub bindgroups: &'w BindGroups3D,
    pub indicerange: &'w IndiceRenderRange,
    pub vertexrange: &'w VertexRenderRange,
    pub instancessortinfo: &'w InstancedMeshTransparentSortCollection,
    pub inscombinerange: Range<u32>,
    pub vertexhash: u64,
    pub bindgroupshash: u64,
}
impl<'w> DrawTmpRef<'w> {
    pub fn can_batch_instance_memory<'a>(&'a self, other: &'a Self, _debug: bool) -> bool {
        // if debug {
        //     log::warn!(
        //         "pipeline: {:?}, vertexhash: {:?}, bindgroupshash: {:?}, instance_memory: {:?}",
        //         (self.pipeline , other.pipeline),
        //         (self.vertexhash.0 , other.vertexhash.0),
        //         (self.bindgroupshash.0 , other.bindgroupshash.0),
        //         (self.instance_memory.is_some() , other.instance_memory.is_some())
        //     );
        // }
        if self.indicerange.is_some() || other.indicerange.is_some() { return false; }
        if self.vertexrange.is_some() || other.vertexrange.is_some() { return false; }
        if self.pipeline.key() == other.pipeline.key()
            && self.vertexhash == other.vertexhash
            && self.bindgroupshash == other.bindgroupshash
        {
            match (&self.rendergeo.instance_slot, &other.rendergeo.instance_slot) {
                (Some(ins1), Some(ins2)) => {
                    ins1 == ins2
                },
                _ => false,
            }
        } else {
            false
        }
    }
    pub fn instancecount<'a>(&'a self) -> u32 {
        self.inscombinerange.end - self.inscombinerange.start
    }
    pub fn instancedatasize<'a>(&'a self) -> usize {
        (self.inscombinerange.end - self.inscombinerange.start) as usize * self.instancessortinfo.sizeperinstance
    }
}

#[derive(Clone)]
pub struct DrawObjTmp {
    pub instance_memory: Option<EVerteicesInstance>,
    pub pipeline: u64,
    pub passentity: Entity,
    pub bindgroupshash: u64,
    pub vertexentity: Entity,
    pub vertexhash: u64,
    pub indice_range: IndiceRenderRange,
    pub vertex_range: VertexRenderRange,
}
impl DrawObjTmp {
    pub fn can_batch_instance_memory(&self, other: &Self, _debug: bool, max_combine_bytes: usize) -> bool {
        // if debug {
        //     log::warn!(
        //         "pipeline: {:?}, vertexhash: {:?}, bindgroupshash: {:?}, instance_memory: {:?}",
        //         (self.pipeline , other.pipeline),
        //         (self.vertexhash.0 , other.vertexhash.0),
        //         (self.bindgroupshash.0 , other.bindgroupshash.0),
        //         (self.instance_memory.is_some() , other.instance_memory.is_some())
        //     );
        // }
        if self.indice_range.is_some() || other.indice_range.is_some() { return false; }
        if self.vertex_range.is_some() || other.vertex_range.is_some() { return false; }
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
impl Drop for DrawObjTmp {
    fn drop(&mut self) {
        self.instance_memory = None;
        log::error!("DrawObjTmp Drop.");
    }
}

pub trait TPassData<T: Clone> {
    fn new(val: T) -> Self;
    fn val(&self) -> &T;
}

#[derive(Component, Default)]
pub struct PassReset;

#[derive(Component, Default)]
pub struct PassModelID(pub Entity);

#[derive(Component, Default)]
pub struct PassRendererID(pub Entity);

#[derive(Component, Default)]
pub struct PassMaterialID(pub Entity);

#[derive(Component, Default)]
pub struct PassGeometryID(pub Entity);

#[derive(Component, Default)]
pub struct PassPipelineStateDirty;

#[derive(Component, Default)]
pub struct PassBindGroupsDirty;

#[derive(Component, Default)]
pub struct PassDrawDirty;

pub trait TPass: Default {
    const TAG: PassTag;
}


#[derive(Component, Default)]
pub struct PassIDs(pub [Entity;8]);

#[derive(Clone, Component, Default)]
pub struct RecordPassDraw(pub [Option<ObjectID>; 8]);

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Clone, Component)]
pub struct PassBindGroups(BindGroups3D, BindGroups3DHashResource);
impl PassBindGroups {
    pub fn new(val: Option<BindGroups3D>) -> Self {
        if let Some(val) = val {
            let hash = BindGroups3DHashResource::from(&val);
            Self(val, hash)
        } else {
            Self(BindGroups3D::default(), BindGroups3DHashResource(0))
        }
    }
    pub fn val(&self) -> Option<&BindGroups3D> {
        if self.1.0 != 0 {
            Some(&self.0)
        } else {
            None
        }
    }
}
impl Default for PassBindGroups {
    fn default() -> Self {
        Self(BindGroups3D::default(), BindGroups3DHashResource(0))
    }
}

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Clone, Component, Default)]
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

#[derive(Component, Default)]
pub struct PassFlagShader;

#[derive(Clone, Component, Default)]
pub struct PassPipelineKey(pub Option<KeyPipeline3D>);
impl TPassData<Option<KeyPipeline3D>> for PassPipelineKey {
    fn new(val: Option<KeyPipeline3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<KeyPipeline3D> { &self.0 }
}


#[derive(Clone, Component, Default)]
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
#[derive(Component, Default)]
pub struct PassDraw(pub bool);
impl PassDraw {
    // pub fn new(val: Option<DrawObjTmp>) -> Self { Self(val) }
    pub fn val(&self) -> bool { self.0 }
}

pub fn _set2_modify(
    _key_meta: &Atom,
    meta: &Handle<ShaderEffectMeta>,
    effect_texture_samplers: &EffectTextureSamplers,
    device: &PiRenderDevice,
    asset_mgr_bindgroup_layout: &ShareAssetMgr<BindGroupLayout>,
    asset_mgr_bindgroup: &ShareAssetMgr<BindGroup>,
) -> Option<Arc<BindGroupTextureSamplers>> {
    let mut result = None;

    let key = KeyBindGroupTextureSamplers::new(effect_texture_samplers.clone(), meta.clone());

    if let Some(key) = key {
        let key_bind_group = key.key_bind_group();
        if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
            let data = BindGroupTextureSamplers::new(key, BindGroupUsage::new(key_bind_group, bind_group));
            let data = Arc::new(data);
            result = Some(data.clone());
        } else {
            // log::error!("Set2: NN");
        };
    } else {
        // log::error!("Set2: NNN");
    }

    return result;
}
