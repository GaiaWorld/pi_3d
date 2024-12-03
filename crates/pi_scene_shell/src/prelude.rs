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
pub use pi_scene_math::{Vector3, Matrix, Rotation3, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolRotation, TToolVector3}, Number, Isometry3};
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
use simba::simd::{SimdBool, SimdComplexField, SimdPartialOrd, SimdRealField};
use wgpu::RenderPass;

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

////////////////////////////////////// Vertex Buffer
#[derive(Resource)]
pub struct ArgVertexBufferAllocator3DSize(pub u32);

#[derive(Resource, DerefMut, Deref)]
pub struct VertexBufferAllocator3D(pub VertexBufferAllocator);

#[derive(Default, Deref, DerefMut, Resource)]
pub struct VBLoaderSlot<T: Clone + core::hash::Hash + PartialEq + Eq, D: From<EVerticesBufferUsage>>(pub VertexBufferLoader<T, D>);

#[derive(Resource, DerefMut, Deref)]
pub struct VertexBufferDataMap3D(pub SingleVertexBufferDataMap);

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
        }
    }
}

pub trait TRenderAlignmentCalc {
    fn calc_rotation(&self, g_rotation: &Rotation3, g_velocity: &Vector3, result: &mut Rotation3) -> bool;
    fn calc_local(&self, g_velocity: &Vector3, length_scale: Number, length_modify: Number, temp: &mut Matrix, temp2: &mut Matrix, result: &mut Matrix) -> bool;
    fn calc_matrix(&self, g_positon: &Vector3, g_scale: &Vector3, g_rotation: &Rotation3, g_velocity: &Vector3, l_positon: &Vector3, l_scale: &Vector3, l_rotation: &Rotation3, l_euler: &Vector3, refwmatrix: & mut Matrix, reflmatrix: & mut Matrix, result: & mut Matrix);
}
impl TRenderAlignmentCalc for ERenderAlignment {
    #[inline(always)]
    fn calc_rotation(&self, g_rotation: &Rotation3, g_velocity: &Vector3, result: &mut Rotation3) -> bool {
        // let mut m = Rotation3::identity();
        match self {
            ERenderAlignment::View => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
                false
            },
            ERenderAlignment::World => {
                // m = Rotation3::identity();
                false
            },
            ERenderAlignment::Local => {
                // m = g_rotation.clone();
                result.clone_from(g_rotation);
                true
            },
            ERenderAlignment::Facing => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
                false
            },
            ERenderAlignment::Velocity => {
                let vlen = CoordinateSytem3::length(g_velocity);
                let z_axis = if vlen > f32::EPSILON {
                    // log::warn!("Vel A");
                    g_velocity.scale(1.0 / vlen)
                } else {
                    // log::warn!("Vel B");
                    Vector3::new(0., 0., 1.)
                };
                *result = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::z_axis(), &z_axis).to_rotation_matrix();

                // let mut y_axis = Vector3::new(0., 1., 0.);
                // let mut x_axis = y_axis.cross(&z_axis);
                // if CoordinateSytem3::length(&x_axis) > f32::EPSILON {
                //     x_axis.normalize_mut();
                //     y_axis = z_axis.cross(&x_axis);
                // } else {
                //     y_axis = Vector3::new(1., 0., 0.);
                //     x_axis = y_axis.cross(&z_axis);
                // }
                // m = CoordinateSytem3::rotation_matrix_from_axises(&x_axis, &y_axis, &z_axis);
                true
            },
            ERenderAlignment::StretchedBillboard => {
                let vlen = CoordinateSytem3::length(g_velocity);
                let x_axis = if vlen > f32::EPSILON {
                    g_velocity.scale(-1.0 / vlen)
                } else {
                    Vector3::new(1., 0., 0.)
                };
                *result = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();

                // let mut y_axis = Vector3::new(0., 1., 0.);
                // let mut z_axis = x_axis.cross(&y_axis);
                // if CoordinateSytem3::length(&z_axis) > f32::EPSILON {
                //     z_axis.normalize_mut();
                //     y_axis = z_axis.cross(&x_axis);
                // } else {
                //     y_axis = Vector3::new(0., 0., 1.);
                //     z_axis = x_axis.cross(&y_axis);
                // }
                // m = CoordinateSytem3::rotation_matrix_from_axises(&x_axis, &y_axis, &z_axis);
                true
            },
            ERenderAlignment::HorizontalBillboard => {
                // let (_, _, z) =  g_rotation_euler;
                *result = CoordinateSytem3::rotation_matrix_from_euler_angles((-90_f32).to_radians(), 0., 0.);
                true
            },
            ERenderAlignment::VerticalBillboard => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
                false
            },
        }
    }
    #[inline(always)]
    fn calc_local(&self, _g_velocity: &Vector3, length_scale: Number, length_modify: Number, temp: &mut Matrix, temp2: &mut Matrix, result: &mut Matrix) -> bool {
        match self {
            ERenderAlignment::View => false,
            ERenderAlignment::World => false,
            ERenderAlignment::Local => false,
            ERenderAlignment::Facing => false,
            ERenderAlignment::Velocity => false,
            ERenderAlignment::StretchedBillboard => {
                // let mut result = Matrix::identity();
                temp2.fill_with_identity();
                // log::warn!("Velocity: {:?}", _g_velocity);

                // let v = Vector3::new(0., 1., 0.);
                // let _g_velocity = &v;
                let vlen = CoordinateSytem3::length(_g_velocity);
                let x_axis = if vlen > f32::EPSILON {
                    _g_velocity.scale(-1.0 / vlen)
                } else {
                    Vector3::new(1., 0., 0.)
                };
                let d_rotation = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();
                // result = result * &d_rotation.to_homogeneous();
                temp2.fixed_view_mut::<3, 3>(0, 0).copy_from(d_rotation.matrix());

                let vlen = length_scale + length_modify;
                let scaling = Vector3::new(vlen, 1., 1.);
                let translation = Vector3::new(0.5, 0., 0.);
                matrix4_compose_no_rotation(&scaling, &translation, temp);
                CoordinateSytem3::mul_to(&temp2, &temp, result);
                // temp2.mul_to(&temp, result);
                true
            },
            ERenderAlignment::HorizontalBillboard => false,
            ERenderAlignment::VerticalBillboard => false,
        }
    }

    #[inline(always)]
    fn calc_matrix(&self, g_positon: &Vector3, g_scale: &Vector3, g_rotation: &Rotation3, g_velocity: &Vector3, l_positon: &Vector3, l_scale: &Vector3, l_rotation: &Rotation3, l_euler: &Vector3, refwmatrix: & mut Matrix, reflmatrix: & mut Matrix, result: & mut Matrix) {

        match self {
            ERenderAlignment::View => {
                calc_matrix_view(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::World => {
                calc_matrix_world(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::Local => {
                calc_matrix_local(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::Facing => {
                calc_matrix_facing(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::Velocity => {
                calc_matrix_velocity(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::StretchedBillboard => {
                calc_matrix_strentched(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::HorizontalBillboard => {
                calc_matrix_horizontal(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::VerticalBillboard => {
                calc_matrix_vertical(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
        }
    }
}

#[inline(always)]
pub fn calc_matrix_view<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a Rotation3, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a Rotation3, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();
    
    // let g_rotation = Rotation3::identity();
    // matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    // let mut l_matrix = Matrix::identity();
    matrix4_compose_rotation(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_world<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a Rotation3, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a Rotation3, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();

    // let g_rotation = Rotation3::identity();
    // matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    // let mut l_matrix = Matrix::identity();
    matrix4_compose_rotation(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_local<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, g_rotation: &'a Rotation3, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a Rotation3, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();
    
    matrix4_compose_rotation(g_scale, g_rotation, g_positon, refwmatrix);
    // let mut l_matrix = Matrix::identity();
    matrix4_compose_rotation(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
    // refwmatrix.mul_to(reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_facing<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a Rotation3, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a Rotation3, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();
    
    // let g_rotation = Rotation3::identity();
    // matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    // let mut l_matrix = Matrix::identity();
    matrix4_compose_rotation(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
    // refwmatrix.mul_to(reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_velocity<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a Rotation3, g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a Rotation3, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();

    // let g_rotation = Rotation3::identity();
    // matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    // let mut l_matrix = Matrix::identity();
    matrix4_compose_rotation(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);

    let mut lookat = Isometry3::identity();
    let mut look_target = g_velocity.clone();
    if look_target.magnitude_squared() < 0.000001 {
        // matrix = matrix * &lookat.to_matrix();
    } else {
        let cood = CoordinateSytem3::left();
        CoordinateSytem3::transform_normal_floats(look_target.x, look_target.y, look_target.z, &result, &mut look_target);
        CoordinateSytem3::lookat(&cood, &Vector3::zeros(), g_velocity, &Vector3::new(0., 1., 0.), &mut lookat);
        CoordinateSytem3::mul_to(&result, &lookat.to_matrix(), refwmatrix);
        result.copy_from(&refwmatrix);
    }
}
#[inline(always)]
pub fn calc_matrix_strentched<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a Rotation3, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, _l_rotation: &'a Rotation3, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();
    
    // let g_rotation = Rotation3::identity();
    // matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    // let mut l_matrix = Matrix::identity();
    // matrix4_compose_rotation(l_scale, &g_rotation, l_positon, &mut l_matrix);
    matrix4_compose_no_rotation(l_scale, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_horizontal<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a Rotation3, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, _l_rotation: &'a Rotation3, l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();
    
    // let g_rotation = Rotation3::identity();
    // matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    // let mut l_matrix = Matrix::identity();
    let l_rotation = CoordinateSytem3::rotation_matrix_from_euler_angles((-90_f32).to_radians(), 0., l_euler.z);
    matrix4_compose_rotation(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_vertical<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a Rotation3, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, _l_rotation: &'a Rotation3, l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    // let mut matrix = Matrix::identity();
    
    // let g_rotation = Rotation3::identity();
    // matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    // let mut l_matrix = Matrix::identity();
    let l_rotation = CoordinateSytem3::rotation_matrix_from_euler_angles(0., l_euler.y, l_euler.z);
    matrix4_compose_rotation(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_local_strentched<'a>(_g_velocity: &'a Vector3, length_scale: Number, length_modify: Number) -> Option<Matrix> {
    let mut result = Matrix::identity();
    // log::warn!("Velocity: {:?}", _g_velocity);

    // let v = Vector3::new(0., 1., 0.);
    // let _g_velocity = &v;
    let vlen = CoordinateSytem3::length(_g_velocity);
    let x_axis = if vlen > f32::EPSILON {
        _g_velocity.scale(-1.0 / vlen)
    } else {
        Vector3::new(1., 0., 0.)
    };
    let d_rotation = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();
    result = result * &d_rotation.to_homogeneous();

    let mut temp = Matrix::identity();
    let vlen = length_scale + length_modify;
    let scaling = Vector3::new(vlen, 1., 1.);
    let translation = Vector3::new(0.5, 0., 0.);
    matrix4_compose_no_rotation(&scaling, &translation, &mut temp);
    // let mut temp = Matrix::identity();
    // temp.append_translation_mut(&translation);
    // Some(result * temp)
    Some(result * temp)
}

#[inline(always)]
pub fn calc_local_strentched_call<'a>(_g_velocity: &'a Vector3, length_scale: Number, length_modify: Number, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    result.fill_with_identity();
    refwmatrix.fill_with_identity();
    reflmatrix.fill_with_identity();

    // let mut result = Matrix::identity();
    // let v = Vector3::new(0., 1., 0.);
    // let _g_velocity = &v;
    let vlen = CoordinateSytem3::length(_g_velocity);
    let x_axis = if vlen > f32::EPSILON {
        _g_velocity.scale(-1.0 / vlen)
    } else {
        Vector3::new(1., 0., 0.)
    };
    let d_rotation = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();
    
    refwmatrix.fixed_view_mut::<3, 3>(0, 0).copy_from(d_rotation.matrix());
    // result = result * &d_rotation.to_homogeneous();

    // let mut temp = Matrix::identity();
    let vlen = length_scale + length_modify;
    let scaling = Vector3::new(vlen, 1., 1.);
    let translation = Vector3::new(0.5, 0., 0.);
    matrix4_compose_no_rotation(&scaling, &translation, reflmatrix);
    // matrix4_compose_no_rotation(&scaling, &translation, &mut temp);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
    // Some(result * temp)
}

#[inline(always)]
pub fn matrix4_compose_rotation(scaling: &Vector3, rotmat: &Rotation3, translation: &Vector3, result: &mut Matrix) {
    result.fill_with_identity();

    result.fixed_view_mut::<3, 3>(0, 0).copy_from(rotmat.matrix());
    // result.append_nonuniform_scaling_mut(scaling);
    result.prepend_nonuniform_scaling_mut(scaling);

    result.append_translation_mut(translation);
    // CoordinateSytem3::matrix4_compose_rotation(scaling, rotmat, translation, result)
}
#[inline(always)]
pub fn matrix4_compose_no_rotation(scaling: &Vector3, translation: &Vector3, result: &mut Matrix) {
    result.fill_with_identity();
    result.append_nonuniform_scaling_mut(scaling);
    result.append_translation_mut(translation);
    // CoordinateSytem3::matrix4_compose_rotation(scaling, rotmat, translation, result)
}

#[inline(always)]
pub fn matrix4_compose_quaternion(scale: &Vector3, rotation: &Quaternion, translation: &Vector3, result: &mut Matrix) {
    let x = rotation.i; let y = rotation.j; let z = rotation.k; let w = rotation.w;
    let x2 = x + x; let y2 = y + y; let z2 = z + z;
    let xx = x * x2; let xy = x * y2; let xz = x * z2;
    let yy = y * y2; let yz = y * z2; let zz = z * z2;
    let wx = w * x2; let wy = w * y2; let wz = w * z2;
    let sx = scale.x; let sy = scale.y; let sz = scale.z;
    result[0] = (1. - (yy + zz)) * sx;
    result[1] = (xy + wz) * sx;
    result[2] = (xz - wy) * sx;
    result[3] = 0.;
    result[4] = (xy - wz) * sy;
    result[5] = (1. - (xx + zz)) * sy;
    result[6] = (yz + wx) * sy;
    result[7] = 0.;
    result[8] = (xz + wy) * sz;
    result[9] = (yz - wx) * sz;
    result[10] = (1. - (xx + yy)) * sz;
    result[11] = 0.;
    result[12] = translation.x;
    result[13] = translation.y;
    result[14] = translation.z;
    result[15] = 1.;
}
#[inline(always)]
pub fn calc_local_other<'a>(_g_velocity: &'a Vector3, _length_scale: Number, _length_modify: Number) -> Option<Matrix> {
    None
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
