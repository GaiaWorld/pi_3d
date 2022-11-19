#![feature(box_into_inner)]

use pi_graph::NGraph;
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::{
    components::view::{
        render_window::{RenderWindow, RenderWindows},
        target_alloc::ShareTargetView,
    },
    init_render,
    rhi::{options::RenderOptions, device::RenderDevice, dyn_uniform_buffer::{Uniform, Bind}, RenderQueue}, graph::graph::RenderGraph, RenderStage,
};
use pi_scene_math::Vector3;
use winit::event_loop::EventLoop;
use std::{any::TypeId, sync::Arc, time::Instant, mem::replace, borrow::BorrowMut};
use wgpu::PresentMode;

use pi_async::{
    prelude::{Mutex, WorkerRuntime},
    rt::{AsyncRuntime, AsyncRuntimeBuilder},
};
use pi_ecs::{
    entity::Id,
    prelude::{
        ArchetypeId, IntoSystem, Query, Res, ResMut, Setup, SingleDispatcher, StageBuilder, System,
        World, Dispatcher, ExecNode,
    },
};
use pi_share::{Share, ShareMutex, ShareRwLock};

use self::{shell_node::ScreenClearNode, frame_time::PluginFrameTime};

pub mod shell_node;
pub mod frame_time;
pub mod plugin;
pub mod engine_shell;
pub mod object;
pub mod run_stage;
pub mod setup;
pub mod assets;
// pub mod image_texture_load;

pub struct DispatchEnd(pub ShareMutex<bool>);

impl Default for DispatchEnd {
    fn default() -> Self {
        Self(ShareMutex::new(true))
    }
}

pub struct PreFrameTime(pub Arc<Mutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Default for PreFrameTime {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Instant::now())))
    }
}
