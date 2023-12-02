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
use pi_scene_math::{Vector3, Matrix, Rotation3, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolRotation, TToolVector3}, Number, Isometry3};
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
pub use crate::pass::*;
pub use crate::log::*;
pub use crate::error::*;
pub use crate::lighting_shadow::*;
pub use crate::bind_defines::*;
pub use crate::forward_rendering::*;
pub use crate::{shader::*, bind_groups::*, binds::*};
pub use crate::pipeline::*;
pub use crate::custom_rendertarget::*;

#[derive(Resource)]
pub struct EngineInstant(pub pi_time::Instant);

///
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Component)]
pub struct EffectTextureSamplersComp(pub Option<EffectTextureSamplers>);

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
impl Default for EVerticeExtendCodeComp {
    fn default() -> Self {
        Self(EVerticeExtendCode::default())
    }
}

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


#[derive(Resource, Default)]
pub struct Performance {
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
}

pub trait TRenderAlignmentCalc {
    fn calc_rotation(&self, g_rotation: &Rotation3, g_velocity: &Vector3) -> Rotation3;
    fn calc_local(&self, g_velocity: &Vector3, length_scale: Number, length_modify: Number) -> Option<Matrix>;
    fn calc_matrix(&self, g_positon: &Vector3, g_scale: &Vector3, g_rotation: &Rotation3, g_velocity: &Vector3, l_positon: &Vector3, l_scale: &Vector3, l_rotation: &Rotation3, l_euler: &Vector3) -> Matrix;
}
impl TRenderAlignmentCalc for ERenderAlignment {
    fn calc_rotation(&self, g_rotation: &Rotation3, g_velocity: &Vector3) -> Rotation3 {
        let mut m = Rotation3::identity();
        match self {
            ERenderAlignment::View => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
            },
            ERenderAlignment::World => {
                // m = Rotation3::identity();
            },
            ERenderAlignment::Local => {
                m = g_rotation.clone();
            },
            ERenderAlignment::Facing => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
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
                m = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::z_axis(), &z_axis).to_rotation_matrix();

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
            },
            ERenderAlignment::StretchedBillboard => {
                let vlen = CoordinateSytem3::length(g_velocity);
                let x_axis = if vlen > f32::EPSILON {
                    g_velocity.scale(-1.0 / vlen)
                } else {
                    Vector3::new(1., 0., 0.)
                };
                m = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();

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

            },
            ERenderAlignment::HorizontalBillboard => {
                // let (_, _, z) =  g_rotation_euler;
                m = CoordinateSytem3::rotation_matrix_from_euler_angles((-90_f32).to_radians(), 0., 0.);
            },
            ERenderAlignment::VerticalBillboard => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
            },
        }
        m
    }
    fn calc_local(&self, _g_velocity: &Vector3, length_scale: Number, length_modify: Number) -> Option<Matrix> {
        match self {
            ERenderAlignment::View => None,
            ERenderAlignment::World => None,
            ERenderAlignment::Local => None,
            ERenderAlignment::Facing => None,
            ERenderAlignment::Velocity => None,
            ERenderAlignment::StretchedBillboard => {
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
                CoordinateSytem3::matrix4_compose_rotation(&scaling, &Rotation3::identity(), &translation, &mut temp);
                // let mut temp = Matrix::identity();
                // temp.append_translation_mut(&translation);
                // Some(result * temp)
                Some(result * temp)
                // Some(result)
            },
            ERenderAlignment::HorizontalBillboard => None,
            ERenderAlignment::VerticalBillboard => None,
        }
    }

    fn calc_matrix(&self, g_positon: &Vector3, g_scale: &Vector3, g_rotation: &Rotation3, g_velocity: &Vector3, l_positon: &Vector3, l_scale: &Vector3, l_rotation: &Rotation3, l_euler: &Vector3) -> Matrix {
        let mut matrix = Matrix::identity();
        match self {
            ERenderAlignment::View => {
                let g_rotation = Rotation3::identity();
                CoordinateSytem3::matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
                let mut l_matrix = Matrix::identity();
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &l_rotation, l_positon, &mut l_matrix);

                matrix = matrix * l_matrix;
                // matrix = matrix * v_rotation;
            },
            ERenderAlignment::World => {
                let g_rotation = Rotation3::identity();
                CoordinateSytem3::matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
                let mut l_matrix = Matrix::identity();
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &l_rotation, l_positon, &mut l_matrix);

                matrix = matrix * l_matrix;
            },
            ERenderAlignment::Local => {
                CoordinateSytem3::matrix4_compose_rotation(g_scale, g_rotation, g_positon, &mut matrix);
                let mut l_matrix = Matrix::identity();
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &l_rotation, l_positon, &mut l_matrix);

                matrix = matrix * l_matrix;
            },
            ERenderAlignment::Facing => {
                let g_rotation = Rotation3::identity();
                CoordinateSytem3::matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
                let mut l_matrix = Matrix::identity();
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &l_rotation, l_positon, &mut l_matrix);

                matrix = matrix * l_matrix;

                // let mut look_target = g_positon - v_position;
                // CoordinateSytem3::transform_normal(&look_target.clone(), &matrix, &mut look_target);
                // let mut lookat = Isometry3::identity();
                // let cood = CoordinateSytem3::left();
                // CoordinateSytem3::lookat(&cood, &Vector3::zeros(), &look_target, &Vector3::new(0., 1., 0.), &mut lookat);
                // matrix = matrix * &lookat.to_matrix();
            },
            ERenderAlignment::Velocity => {
                let g_rotation = Rotation3::identity();
                CoordinateSytem3::matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
                let mut l_matrix = Matrix::identity();
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &l_rotation, l_positon, &mut l_matrix);
                
                matrix = matrix * l_matrix;

                let mut lookat = Isometry3::identity();
                let mut look_target = g_velocity.clone();
                if look_target.magnitude_squared() < 0.000001 {
                    // matrix = matrix * &lookat.to_matrix();
                } else {
                    let cood = CoordinateSytem3::left();
                    CoordinateSytem3::transform_normal(&look_target.clone(), &matrix, &mut look_target);
                    CoordinateSytem3::lookat(&cood, &Vector3::zeros(), g_velocity, &Vector3::new(0., 1., 0.), &mut lookat);
                    matrix = matrix * &lookat.to_matrix();
                }
            },
            ERenderAlignment::StretchedBillboard => {
                let g_rotation = Rotation3::identity();
                CoordinateSytem3::matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
                
                let mut l_matrix = Matrix::identity();
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &g_rotation, l_positon, &mut l_matrix);
                matrix = matrix * l_matrix;
                
                // let vlen = CoordinateSytem3::length(g_velocity);
                // let x_axis = if vlen > f32::EPSILON {
                //     g_velocity.scale(-1.0 / vlen)
                // } else {
                //     Vector3::new(1., 0., 0.)
                // };
                // let d_rotation = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();
                // matrix = matrix * &d_rotation.to_homogeneous();

                // let mut look_target = g_positon - v_position;
                // let mut lookat = Isometry3::identity();
                // if look_target.magnitude_squared() < 0.000001 {
                //     // matrix = matrix * &lookat.to_matrix();
                // } else {
                //     let cood = CoordinateSytem3::left();
                //     CoordinateSytem3::lookat(&cood, &Vector3::zeros(), &look_target, &Vector3::new(0., 1., 0.), &mut lookat);
                //     matrix = matrix * &lookat.to_matrix();
                // }
            },
            ERenderAlignment::HorizontalBillboard => {
                let g_rotation = Rotation3::identity();
                CoordinateSytem3::matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
                let mut l_matrix = Matrix::identity();
                let l_rotation = CoordinateSytem3::rotation_matrix_from_euler_angles((-90_f32).to_radians(), 0., l_euler.z);
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &l_rotation, l_positon, &mut l_matrix);

                matrix = matrix * l_matrix;
            },
            ERenderAlignment::VerticalBillboard => {
                let g_rotation = Rotation3::identity();
                CoordinateSytem3::matrix4_compose_rotation(g_scale, &g_rotation, g_positon, &mut matrix);
                let mut l_matrix = Matrix::identity();
                let l_rotation = CoordinateSytem3::rotation_matrix_from_euler_angles(0., l_euler.y, l_euler.z);
                CoordinateSytem3::matrix4_compose_rotation(l_scale, &l_rotation, l_positon, &mut l_matrix);

                matrix = matrix * l_matrix;
            },
        }

        matrix
    }
}