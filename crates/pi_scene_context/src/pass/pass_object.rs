
use std::{sync::Arc, ops::Range};

use crate::{bindgroup::*, prelude::*};

pub use pi_scene_shell::prelude::*;

#[derive(Clone)]
pub struct DrawTmpRef<'w> {
    pub rendergeo: &'w RenderGeometry,
    pub pipeline: &'w Pipeline3DUsage,
    pub bindgroups: &'w BindGroups3D,
    pub indicerange: &'w IndiceRenderRange,
    pub vertexrange: &'w VertexRenderRange,
    // 用于记录 Mesh 的实例的排序后实例数据
    pub instancessortinfo: &'w InstancedMeshTransparentSortCollection,
    pub inscombinerange: Range<u32>,
    pub pass: u8,
    pub distance: f32,
    pub queue: RenderQueueSortParam,
}
impl<'w> DrawTmpRef<'w> {
    pub fn can_batch_instance_memory<'a>(&'a self, other: &'a Self, _debug: bool) -> bool {
        // if _debug {
        //     log::warn!(
        //         "pipeline: {:?}, vertexhash: {:?}, bindgroupshash: {:?}, instance_memory: {:?}",
        //         (self.pipeline.key() , other.pipeline.key()),
        //         (self.rendergeo.hashresource , other.rendergeo.hashresource),
        //         (self.bindgroups.hashresource , other.bindgroups.hashresource),
        //         (self.rendergeo.instance_slot.is_some() , other.rendergeo.instance_slot.is_some())
        //     );
        // }
        if self.indicerange.is_some() || other.indicerange.is_some() { return false; }
        if self.vertexrange.is_some() || other.vertexrange.is_some() { return false; }
        if self.pipeline.key() == other.pipeline.key()
            && self.rendergeo.hashresource == other.rendergeo.hashresource
            && self.bindgroups.hashresource == other.bindgroups.hashresource
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
        (self.inscombinerange.end - self.inscombinerange.start) as usize * self.instancessortinfo.sizeperinstance as usize
    }
    pub fn cmp_opaque<'a>(a: &'a Self, other: &'a Self) -> std::cmp::Ordering {
        match a.pass.cmp(&other.pass) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                match a.pipeline.key().cmp(&other.pipeline.key()) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => {
                        match a.bindgroups.hashresource.cmp(&other.bindgroups.hashresource) {
                            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                            std::cmp::Ordering::Equal => {
                                match a.rendergeo.hashresource.cmp(&other.rendergeo.hashresource) {
                                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                    std::cmp::Ordering::Equal => {
                                        match a.distance.partial_cmp(&other.distance) {
                                            Some(order) => order,
                                            None => std::cmp::Ordering::Equal,
                                        }
                                    },
                                }
                            },
                        }
                    },
                }
            },
        }
    }
    pub fn cmp_transparent<'a>(a: &'a Self, other: &'a Self) -> std::cmp::Ordering {
        match a.pass.cmp(&other.pass) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                match a.queue.cmp(&other.queue) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => {
                        match other.distance.partial_cmp(&a.distance) {
                            Some(order) => order,
                            None => {
                                match a.pipeline.key().cmp(&other.pipeline.key()) {
                                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                    std::cmp::Ordering::Equal => {
                                        match a.bindgroups.hashresource.cmp(&other.bindgroups.hashresource) {
                                            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                            std::cmp::Ordering::Equal => {
                                                a.rendergeo.hashresource.cmp(&other.rendergeo.hashresource)
                                            },
                                        }
                                    },
                                }
                            }
                        }
                    },
                }
            },
        }
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

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Clone, Component)]
pub struct PassBindGroups(Option<BindGroups3D>);
impl PassBindGroups {
    pub fn new(val: Option<BindGroups3D>) -> Self {
        Self(val)
    }
    pub fn val(&self) -> Option<&BindGroups3D> {
        self.0.as_ref()
    }
}
impl Default for PassBindGroups {
    fn default() -> Self {
        Self(None)
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

pub fn _set3_modify(
    _key_meta: &Atom,
    meta: &Handle<ShaderEffectMeta>,
    effect_texture_samplers: &EffectTextureSamplers,
    device: &PiRenderDevice,
    asset_mgr_bindgroup_layout: &ShareAssetMgr<BindGroupLayout>,
    asset_mgr_bindgroup: &ShareAssetMgr<BindGroup>,
) -> Option<Arc<BindGroupTextureSamplers>> {
    let mut result = None;

    let key = KeyBindGroupTextureSamplers::new(effect_texture_samplers.clone());

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
