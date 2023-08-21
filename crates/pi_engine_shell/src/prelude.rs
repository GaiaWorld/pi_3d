use std::{ops::Range, mem::replace};

pub use bevy::{
    app::{ prelude::*, PluginGroupBuilder }, core::prelude::*, ecs::prelude::*, hierarchy::prelude::*, input::{prelude::*, InputPlugin},
    log::prelude::*, math::prelude::*, reflect::prelude::*, time::prelude::*,
    utils::prelude::*, window::prelude::*,
    ecs::system::{CommandQueue, EntityCommands, SystemState, SystemParam}, prelude::{Deref, DerefMut},
    a11y::*,
    // winit::*,
};
pub use pi_atom::Atom;
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
    asset_config::*, should_run, component::GraphId
};
use pi_scene_math::{Vector3, Matrix, Rotation3, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolRotation, TToolVector3}, Number};
pub use pi_window_renderer::*;
pub use pi_render::{
    asset::*,
    renderer::{
        attributes::*,
        bind_group::*,
        bind_buffer::*,
        vertex_buffer::*,
        vertex_buffer_desc::*,
        vertex_buffer_loader::*,
        vertices::*,
        instance::*,
        sampler::*,
        texture::*,
        shader::*,
        sampler::*,
        shader_stage::*,
        draw_sort::*,
        draw_obj::*,
        draw_obj_list::*,
        pipeline::*,
        buildin_data::*,
        buildin_var::*,
    },
    render_3d::{
        shader::*,
        bind_groups::{ scene::*, model::*, texture_sampler::* },
        binds::{ scene::*, model::*, effect_value::*, effect_sampler2d::*, effect_texture2d::*},
    },
    rhi::{
        asset::*,
        pipeline::*,
        device::RenderDevice,
        RenderQueue,
        shader::WriteBuffer,
        texture::*,
    },
    components::view::target_alloc::*,
};
pub use pi_assets::asset::GarbageEmpty;
pub use pi_curves::curve::{ FrameIndex, FramePerSecond };

pub use crate::run_stage::ERunStageChap;
pub use crate::object::ObjectID;
pub use crate::engine_shell::*;
pub use crate::assets::{
    sync_load::*,
    image_texture_load::*
};
pub use crate::effect_sampler2d::*;
pub use crate::effect_texture2d::*;
pub use crate::frame_time::SingleFrameTimeCommand;
pub use crate::entity_ref::*;
pub use crate::animation::*;
pub use crate::interpolation::*;
pub use crate::extends::*;

///
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Component)]
pub struct EffectTextureSamplersComp(pub pi_render::render_3d::bind_groups::texture_sampler::EffectTextureSamplers);

/////////////////////////////////////// Global Control
#[derive(Component)]
pub enum GlobalColorSpace {
    Linear,
    Gamma,
}

/////////////////////////////////////// Bind Buffer
#[derive(Resource, DerefMut, Deref)]
pub struct ResBindBufferAllocator(pub BindBufferAllocator);

////////////////////////////////////// Vertex Buffer
#[derive(Resource, DerefMut, Deref)]
pub struct VertexBufferAllocator3D(pub VertexBufferAllocator);

#[derive(Default, Deref, DerefMut, Resource)]
pub struct VBLoaderSlot<T: Clone + core::hash::Hash + PartialEq + Eq, D: From<EVerticesBufferUsage>>(pub VertexBufferLoader<T, D>);

#[derive(Resource, DerefMut, Deref)]
pub struct VertexBufferDataMap3D(pub SingleVertexBufferDataMap);

#[derive(Debug, Component, Clone)]
pub struct IndicesBufferDesc {
    pub format: wgpu::IndexFormat,
    /// bytes 范围
    pub buffer_range: Option<Range<wgpu::BufferAddress>>,
    pub buffer: KeyVertexBuffer,
}

#[derive(Debug, Deref, Clone, Hash, Component)]
pub struct AssetKeyBufferIndices(pub KeyVertexBuffer);

#[derive(Deref, Component)]
pub struct AssetResBufferIndices(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResBufferIndices {
    fn from(value: EVerticesBufferUsage) -> Self {
        Self(value)
    }
}

#[derive(Deref, DerefMut, Component)]
pub struct EVerticeExtendCodeComp(pub EVerticeExtendCode);

#[derive(Deref, DerefMut, Component)]
pub struct VertexBufferLayoutsComp(pub VertexBufferLayouts);

////////////////////////////////////// Shader
// #[derive(Deref, DerefMut, Component)]
// pub struct KeyShaderMeta(pub Atom);
pub type KeyShaderMeta = Atom;

#[derive(Deref, DerefMut, Resource)]
pub struct ResBindsRecorder(pub BindsRecorder);

/// 操作队列 最多等待 ACTION_WAIT_FRAME 帧
pub const ACTION_WAIT_FRAME: u16 = 10;

pub trait TAction {
    fn again<T: Clone>(entity: Entity, val: T, count: u16) -> Self;
}

////////////////////////////////////// Commands
#[derive(Resource)]
// pub struct ActionList<T: Send + Sync + 'static>(Events<T>);
pub struct ActionList<T: Send + Sync + 'static>(Vec<T>);
impl<T: Send + Sync> Default for ActionList<T> {
    fn default() -> Self {
        // Self(Events::default())
        Self(Vec::default())
    }
}
impl<T: Send + Sync> ActionList<T> {
    pub fn push_some(&mut self, _val: impl IntoIterator<Item = T>) {
        // self.0.extend(val);
    }
    pub fn push(&mut self, val: T) {
        // self.0.extend([val]);
        self.0.push(val);
    }
    pub fn drain(&mut self) -> Vec<T> {
        // self.0.drain().collect()
        replace(&mut self.0, vec![])
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

pub trait TRenderAlignmentCalc {
    fn calc_rotation(&self, g_rotation: &Rotation3, g_rotation_euler: (Number, Number, Number), g_velocity: &Vector3) -> Rotation3;
    fn calc_local(&self, g_velocity: &Vector3) -> Option<Matrix>;
}
impl TRenderAlignmentCalc for ERenderAlignment {
    fn calc_rotation(&self, g_rotation: &Rotation3, g_rotation_euler: (Number, Number, Number), g_velocity: &Vector3) -> Rotation3 {
        let mut m = Rotation3::identity();
        match self {
            ERenderAlignment::View => {
                let (_, _, z) =  g_rotation_euler;
                m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
            },
            ERenderAlignment::World => {
                // m = Rotation3::identity();
            },
            ERenderAlignment::Local => {
                m = g_rotation.clone();
            },
            ERenderAlignment::Facing => {
                let (_, _, z) =  g_rotation_euler;
                m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
            },
            ERenderAlignment::Velocity => {
                let vlen = CoordinateSytem3::length(g_velocity);
                let z_axis = if vlen > 0.00000001 {
                    g_velocity.normalize()
                } else {
                    Vector3::new(0., 0., 1.)
                };

                let mut y_axis = Vector3::new(0., 1., 0.);
                let mut x_axis = y_axis.cross(&z_axis);
                if CoordinateSytem3::length(&x_axis) > 0. {
                    x_axis.normalize_mut();
                    y_axis = z_axis.cross(&x_axis);
                } else {
                    y_axis = Vector3::new(1., 0., 0.);
                    x_axis = y_axis.cross(&z_axis);
                }
                m = CoordinateSytem3::rotation_matrix_from_axises(&x_axis, &y_axis, &z_axis);
            },
            ERenderAlignment::StretchedBillboard => {
                let vlen = CoordinateSytem3::length(g_velocity);
                let x_axis = if vlen > 0.00000001 {
                    g_velocity.normalize()
                } else {
                    Vector3::new(1., 0., 0.)
                };

                let mut y_axis = Vector3::new(0., 1., 0.);
                let mut z_axis = x_axis.cross(&y_axis);
                if CoordinateSytem3::length(&z_axis) > 0. {
                    z_axis.normalize_mut();
                    y_axis = z_axis.cross(&x_axis);
                } else {
                    y_axis = Vector3::new(0., 0., 1.);
                    z_axis = x_axis.cross(&y_axis);
                }
                m = CoordinateSytem3::rotation_matrix_from_axises(&x_axis, &y_axis, &z_axis);

            },
            ERenderAlignment::HorizontalBillboard => {
                let (_, _, z) =  g_rotation_euler;
                m = CoordinateSytem3::rotation_matrix_from_euler_angles((90_f32).to_radians(), 0., z);
            },
            ERenderAlignment::VerticalBillboard => {
                let (_, _, z) =  g_rotation_euler;
                m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
            },
        }
        m
    }
    fn calc_local(&self, g_velocity: &Vector3) -> Option<Matrix> {
        match self {
            ERenderAlignment::View => None,
            ERenderAlignment::World => None,
            ERenderAlignment::Local => None,
            ERenderAlignment::Facing => None,
            ERenderAlignment::Velocity => None,
            ERenderAlignment::StretchedBillboard => {
                let mut result = Matrix::identity();
                let vlen = CoordinateSytem3::length(g_velocity);
                let scaling = Vector3::new(vlen, 1., 1.);
                let translation = Vector3::new(-0.5 * vlen, 0., 0.);
                CoordinateSytem3::matrix4_compose_rotation(&scaling, &Rotation3::identity(), &translation, &mut result);
                Some(result)
            },
            ERenderAlignment::HorizontalBillboard => None,
            ERenderAlignment::VerticalBillboard => None,
        }
    }
}