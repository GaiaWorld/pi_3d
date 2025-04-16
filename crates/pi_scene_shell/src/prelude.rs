use std::{hash::Hash, mem::{replace, size_of}, ops::Range, vec::Drain};

pub use parry3d::{
    bounding_volume::Aabb,
    na::{Isometry3 as NAIsometry3, Point3},
    query::{Ray, RayCast},
    shape::{ConvexPolyhedron, Cuboid},
};

pub use derive_deref::{DerefMut, Deref};
pub use pi_atom::Atom;
pub use pi_bevy_render_plugin::FrameDataPrepare;
pub use pi_bevy_winit_window::*;
pub use pi_bevy_ecs_extend::prelude::*;
pub use pi_bevy_asset::{
    AssetMgrConfigs,
    AssetCapacity,
    ShareAssetMgr,
    TAssetCapacity,
};
pub use pi_bevy_render_plugin::{
    PiRenderDevice, PiRenderQueue, PiRenderGraph, PiRenderWindow, PiRenderOptions, PiSafeAtlasAllocator, PiScreenTexture,
    node::*, RenderContext, GraphError, constant::{ render_state::*, texture_sampler::* }, 
    asset_config::*, should_run, render_cross::GraphId, SimpleInOut,
    InstanceCacheBuffer, PiInstanceBufferAllocator
};
pub use pi_null::Null;
pub use pi_map::smallvecmap::SmallVecMap;
use pi_render::renderer::attributes::KeyAttributesLayouts;
use pi_scene_math::{Quaternion, SQuaternion, Vector4};
pub use pi_render::{
    asset::*,
    renderer::{
        bind_group::*,
        bind_buffer::*,
        vertex_buffer::*,
        vertex_buffer_loader::*,
        vertices::*,
        instance::*,
        sampler::*,
        texture::*,
        shader::*,
        sampler::*,
        shader_stage::*,
        draw_obj::*,
        draw_obj_list::*,
        pipeline::*,
        buildin_data::*,
        vertex_format::TVertexFormatByteSize
    },
    rhi::{
        asset::*,
        pipeline::*,
        device::RenderDevice,
        RenderQueue,
        shader::WriteBuffer,
        texture::*,
        buffer::Buffer,
        sampler::{SamplerDesc, EAddressMode, EAnisotropyClamp, EFilterMode}
    },
    components::view::target_alloc::*,
};
pub use pi_assets::{asset::{GarbageEmpty, Asset, Size, Handle, Garbageer}, mgr::{AssetMgr, LoadResult}, homogeneous::{HomogeneousMgr, GarbageEmpty as HomoGarbageEmpty}};
pub use pi_curves::curve::{ FrameIndex, FramePerSecond };
pub use pi_spatial::oct_helper::OctTree;
pub use pi_hash::{XHashSet, XHashMap, DefaultHasher};
pub use pi_async_rt::prelude::AsyncRuntime;
pub use pi_hal::{runtime::RENDER_RUNTIME, loader::AsyncLoader};
pub use pi_share::{Share, ThreadSync, ShareRefCell};
pub use pi_scene_math::{Vector3, Matrix, Rotation3, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolRotation, TToolVector3}, Number, Isometry3};

pub use crate::run_stage::{ERunStageChap, runif_3d, EngineCustomPlugins};
pub use crate::object::ObjectID;
pub use crate::engine_shell::*;
pub use crate::assets::texture::*;
pub use crate::assets::image_texture_load::*;
pub use crate::assets::texture_atlas::*;
pub use crate::effect_sampler2d::*;
pub use crate::effect_texture2d::*;
pub use crate::frame_time::SingleFrameTimeCommand;
pub use crate::entity_ref::*;
pub use crate::animation::*;
pub use crate::interpolation::*;
pub use crate::extends::*;
pub use crate::pass::*;
pub use crate::log::*;
pub use crate::error::*;
pub use crate::lighting_shadow::*;
pub use crate::bind_defines::*;
pub use crate::forward_rendering::*;
pub use crate::{shader::*, bind_groups::*, binds::*};
pub use crate::pipeline::*;
pub use crate::custom_rendertarget::*;
pub use crate::object::*;
pub use crate::batch::*;
pub use crate::static_string::*;
pub use crate::ecs::*;
pub use crate::render_state::*;
pub use crate::math::*;

#[derive(Resource, Deref)]
pub struct DeviceLimits3D(pub(crate) wgpu::Limits);

#[derive(Resource)]
pub struct EngineInstant(pub pi_time::Instant);

///
#[derive(Clone, Hash, PartialEq, Eq, Component, Default)]
pub struct EffectTextureSamplersComp(pub Option<EffectTextureSamplers>);

/////////////////////////////////////// Global Control
#[derive(Component, Default)]
pub enum GlobalColorSpace {
    #[default]
    Linear,
    Gamma,
}

/////////////////////////////////////// Bind Buffer
#[derive(Resource, DerefMut, Deref)]
pub struct ResBindBufferAllocator(pub BindBufferAllocator);
impl MemSize for ResBindBufferAllocator {
    fn memsize(&self) -> usize {
        self.0.size()
    }
}

////////////////////////////////////// Vertex Buffer
#[derive(Resource)]
pub struct ArgVertexBufferAllocator3DSize(pub u32);

#[derive(Resource, DerefMut, Deref)]
pub struct VertexBufferAllocator3D(pub VertexBufferAllocator);

#[derive(Default, Deref, DerefMut, Resource)]
pub struct VBLoaderSlot<T: Clone + core::hash::Hash + PartialEq + Eq, D: From<EVerticesBufferUsage>>(pub VertexBufferLoader<T, D>);

#[derive(Resource, DerefMut, Deref)]
pub struct VertexBufferDataMap3D(pub SingleVertexBufferDataMap);
impl MemSize for VertexBufferDataMap3D {
    fn memsize(&self) -> usize {
        self.0.size()
    }
}

#[derive(Clone)]
pub struct IndicesBufferDesc {
    pub format: wgpu::IndexFormat,
    /// bytes 范围
    // pub buffer_range: Option<Range<wgpu::BufferAddress>>,
    pub buffer_range: Option<Range<u32>>,
    pub buffer: KeyVertexBuffer,
}

#[derive(Component, Default, Clone)]
pub struct IndicesBufferDescComp(pub Option<IndicesBufferDesc>);

#[derive(Deref, Clone, Hash, Component, Default)]
pub struct AssetKeyBufferIndices(pub Option<KeyVertexBuffer>);

// TODO Send问题， 临时解决
unsafe impl Send for AssetKeyBufferIndices {}
unsafe impl Sync for AssetKeyBufferIndices {}

pub trait EqAsResource {
    fn eq_resource(&self, other: &Self) -> bool;
}
pub trait EqAsLayout {
    fn eq_layout(&self, other: &Self) -> bool;
}
pub trait HashAsResource {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H);
}
pub trait HashAsLayout {
    fn hash_layout<H: std::hash::Hasher>(&self, state: &mut H);
}

impl HashAsLayout for VertexBufferDesc {
    fn hash_layout<H: std::hash::Hasher>(&self, state: &mut H) {
        self.attributes().hash(state);
        self.instance().hash(state);
    }
}
impl HashAsResource for VertexBufferDesc {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.range().hash(state);
        self.attributes().hash(state);
        self.instance().hash(state);
    }
}

#[derive(Deref, Component, Default)]
pub struct AssetResBufferIndicesComp(pub Option<AssetResBufferIndices>);

#[derive(Deref)]
pub struct AssetResBufferIndices(pub EVerticesBufferUsage);

// TODO Send问题， 临时解决
unsafe impl Send for AssetResBufferIndices {}
unsafe impl Sync for AssetResBufferIndices {}

impl From<EVerticesBufferUsage> for AssetResBufferIndices {
    fn from(value: EVerticesBufferUsage) -> Self {
        Self(value)
    }
}

pub trait FromVertexBufferDescs {
    fn from_descs(value: &Vec<VertexBufferDesc>) -> Self;
}

impl FromVertexBufferDescs for VertexBufferLayouts {
    fn from_descs(value: &Vec<VertexBufferDesc>) -> Self {
        let mut layouts = vec![];
        let mut datasize = 0;

        // 按 EVertexDataKind 排序确定 shader_location
        let mut shader_location = 0;
        let mut attrcount = 0;
        let mut desccount = 0;
        value.iter().for_each(|buffer_desc| {
            let mut attrs = vec![];
            let mut offset = 0;
            buffer_desc.attributes().iter().for_each(|attribute| {
                let format = attribute.format();
                let stride = format.use_bytes();
                attrs.push(wgpu::VertexAttribute {
                    format,
                    offset,
                    shader_location,
                });
                offset += stride;
                shader_location += 1;
                attrcount += 1;

                datasize += size_of::<wgpu::VertexAttribute>();
            });

            desccount += 1;
            layouts.push((attrs, buffer_desc.step_mode(), offset as u32));
            datasize += 8;
        });

        Self { size: datasize, desccount, attrcount, layout_list: KeyAttributesLayouts(layouts),  }
    }
}

#[derive(Component)]
pub struct VertexBufferLayoutsComp(pub VertexBufferLayouts, pub KeyShaderFromAttributes);
impl Default for VertexBufferLayoutsComp {
    fn default() -> Self {
        Self(
            VertexBufferLayouts::from_descs(&vec![]),
            KeyShaderFromAttributes::new(&vec![])
        )
    }
}

////////////////////////////////////// Shader
pub type KeyShaderMeta = Atom;

#[derive(Resource)]
pub struct ResBindsRecorder();

/// 操作队列 最多等待 ACTION_WAIT_FRAME 帧
pub const ACTION_WAIT_FRAME: u16 = 10;

pub trait TAction {
    fn again<T: Clone>(entity: Entity, val: T, count: u16) -> Self;
}

pub trait MemSize {
    fn memsize(&self) -> usize;
}

////////////////////////////////////// Commands
#[derive(Resource)]
pub struct ActionList<T: Send + Sync + 'static>(Vec<T>);
impl<T: Send + Sync> Default for ActionList<T> {
    fn default() -> Self {
        // Self(Events::default())
        Self(Vec::default())
    }
}
impl<T: Send + Sync> ActionList<T> {
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.0.capacity() * size_of::<T>()
    }
    #[inline(always)]
    pub fn append(&mut self, val: &mut Self) -> &mut Self {
        self.0.append(&mut val.0);
        self
    }
    #[inline(always)]
    pub fn push_some(&mut self, val: impl IntoIterator<Item = T>) -> &mut Self {
        self.0.extend(val);
        self
    }
    #[inline(always)]
    pub fn push(&mut self, val: T) -> &mut Self {
        // self.0.extend([val]);
        self.0.push(val);
        self
    }
    #[inline(always)]
    pub fn drain(&mut self) -> Drain<T> {
        self.0.drain(..)
    }
    #[inline(always)]
    pub fn exchange_empty(&mut self) -> Vec<T> {
        // self.0.drain().collect()
        replace(&mut self.0, vec![])
    }
    #[inline(always)]
    pub fn exchange(&mut self, other: Vec<T>) -> Vec<T> {
        // self.0.drain().collect()
        replace(&mut self.0, other)
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.0.len();
    }
}
impl<T: Send + Sync> MemSize for ActionList<T> {
    fn memsize(&self) -> usize {
        return self.0.capacity();
    }
}

#[derive(Resource)]
pub struct SingleEmptyEntity(Entity);
impl SingleEmptyEntity {
    pub fn new(id: Entity) -> Self {
        Self(id)
    }
    pub fn id(&self) -> Entity {
        self.0
    }
}


#[derive(Resource)]
pub struct Performance {
    pub debug: bool,
    pub t_worldmatrix: pi_time::Instant,
    pub t_culling: pi_time::Instant,
    pub t_drawobjs: pi_time::Instant,
    pub t_gltfanaly: pi_time::Instant,
    pub t_animation: pi_time::Instant,
    pub t_animationgroup: pi_time::Instant,
    pub t_particlesystem: pi_time::Instant,
    pub t_trialcalc: pi_time::Instant,
    pub t_uniformbufferupdate: pi_time::Instant,
    pub t_uniformupdate: pi_time::Instant,
    pub worldmatrix: u32,
    pub culling: u32,
    pub drawobjs: u32,
    pub gltfanaly: u32,
    pub animation: u32,
    pub animationgroup: u32,
    pub particlesystem: u32,
    pub trialcalc: u32,
    pub uniformbufferupdate: u32,
    pub uniformupdate: u32,
    pub drawcalls: u32,
    
    pub systems: Vec<String>,
}
impl Default for Performance {
    fn default() -> Self {
        Self {
            debug: false,
            t_worldmatrix: pi_time::Instant::now(),
            t_culling: pi_time::Instant::now(),
            t_drawobjs: pi_time::Instant::now(),
            t_gltfanaly: pi_time::Instant::now(),
            t_animation: pi_time::Instant::now(),
            t_animationgroup: pi_time::Instant::now(),
            t_particlesystem: pi_time::Instant::now(),
            t_trialcalc: pi_time::Instant::now(),
            t_uniformbufferupdate: pi_time::Instant::now(),
            t_uniformupdate: pi_time::Instant::now(),
            worldmatrix: 0,
            culling: 0,
            drawobjs: 0,
            gltfanaly: 0,
            animation: 0,
            animationgroup: 0,
            particlesystem: 0,
            trialcalc: 0,
            uniformbufferupdate: 0,
            uniformupdate: 0,
            drawcalls: 0,
            systems: vec![],
        }
    }
}


#[inline(always)]
pub fn unsafe_vec_append_slice<T>(vec: &mut Vec<T>, slice: &[T]) {
    unsafe {
        let other = slice as *const [T];
        let count = (*other).len();
        vec.reserve(count);
        let len = vec.len();
        std::ptr::copy_nonoverlapping(other as *const T, vec.as_mut_ptr().add(len), count);
        vec.set_len(len + count);
    }
}

pub fn runif_acts<T: Send + Sync + 'static>(
    acts: Res<ActionList<T>>
) -> bool {
    acts.len() > 0
}
pub fn runif_acts2<T: Send + Sync + 'static, T2: Send + Sync + 'static>(
    acts: Res<ActionList<T>>,
    acts2: Res<ActionList<T2>>,
) -> bool {
    acts.len() + acts2.len() > 0
}
pub fn runif_comp<T: Component>(
    acts: ComponentChanged<T>,
    acts2: ComponentAdded<T>
) -> bool {
    acts.len() + acts2.len() > 0
}
pub fn runif_changes<T: Component>(
    acts: ComponentChanged<T>
) -> bool {
    acts.len() > 0
}
pub fn runif_changes2<T: Component, T2: Component>(
    acts: ComponentChanged<T>,
    acts2: ComponentChanged<T2>
) -> bool {
    acts.len() + acts2.len() > 0
}
