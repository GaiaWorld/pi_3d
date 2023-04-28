use std::{ops::Range, mem::replace};

pub use bevy::{
    app::{ prelude::*, PluginGroupBuilder }, core::prelude::*, ecs::prelude::*, hierarchy::prelude::*, input::{prelude::*, InputPlugin},
    log::prelude::*, math::prelude::*, reflect::prelude::*, time::prelude::*,
    utils::prelude::*, window::{prelude::*},
    ecs::system::{CommandQueue, EntityCommands, SystemState, SystemParam}, prelude::{Deref, DerefMut},
    a11y::*,
    // winit::*,
};
pub use pi_atom::Atom;
pub use pi_bevy_winit_window::*;
pub use pi_bevy_ecs_extend::prelude::*;
pub use pi_bevy_asset::{AssetMgrConfigs, AssetCapacity, ShareAssetMgr};
pub use pi_bevy_render_plugin::{
    PiRenderDevice, PiRenderQueue, PiRenderGraph, PiRenderWindow, PiRenderOptions, PiSafeAtlasAllocator, PiScreenTexture,
    node::*, RenderContext, GraphError
};
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
        binds::{ scene::*, model::*, effect_value::*, effect_sampler2d::*, effect_texture2d::*, texture_sampler::* },
    },
    rhi::{
        asset::*,
        pipeline::*,
        device::{ RenderDevice },
        RenderQueue,
        shader::WriteBuffer,
        texture::*,
    },
    components::view::{
        target_alloc::*,
    }
};
pub use pi_assets::{asset::GarbageEmpty};

pub use crate::run_stage::ERunStageChap;
pub use crate::object::ObjectID;
pub use crate::engine_shell::EnginShell;
pub use crate::assets::{
    sync_load::*,
    image_texture_load::*
};
pub use crate::effect_sampler2d::*;
pub use crate::effect_texture2d::*;
pub use crate::frame_time::SingleFrameTimeCommand;
pub use crate::entity_ref::*;

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

#[derive(Debug, Component)]
pub struct IndicesBufferDesc {
    pub format: wgpu::IndexFormat,
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
pub struct EInstanceCodeComp(pub EInstanceCode);

#[derive(Deref, DerefMut, Component)]
pub struct VertexBufferLayoutsComp(pub VertexBufferLayouts);

////////////////////////////////////// Shader
// #[derive(Deref, DerefMut, Component)]
// pub struct KeyShaderMeta(pub Atom);
pub type KeyShaderMeta = Atom;

#[derive(Deref, DerefMut, Resource)]
pub struct ResBindsRecorder(pub BindsRecorder);

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
    pub fn push_some(&mut self, val: impl IntoIterator<Item = T>) {
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