use bevy::prelude::{DerefMut, Deref};
pub use bevy::{
    app::prelude::*, core::prelude::*, ecs::prelude::*, hierarchy::prelude::*, input::prelude::*,
    log::prelude::*, math::prelude::*, reflect::prelude::*, time::prelude::*,
    transform::prelude::*, utils::prelude::*, window::prelude::*, DefaultPlugins, MinimalPlugins, 
    ecs::system::CommandQueue,
};
pub use crate::run_stage::ERunStageChap;
pub use crate::object::ObjectID;
pub use crate::engine_shell::EnginShell;
pub use pi_bevy_asset::{AssetMgrConfigs, AssetCapacity, ShareAssetMgr};
pub use pi_bevy_render_plugin::{ PiRenderDevice, PiRenderQueue, PiRenderGraph, PiRenderWindow, PiRenderOptions };
pub use pi_render::renderer::{bind_buffer::{BindBufferAllocator}, vertex_buffer::{KeyVertexBuffer, VertexBufferAllocator, AssetVertexBuffer, VertexBufferLayouts, VertexBufferLayout}, };

#[derive(Resource, DerefMut, Deref)]
pub struct ResBindBufferAllocator(pub BindBufferAllocator);