#![feature(box_into_inner)]

use std::{sync::Arc, };


use pi_share::ShareMutex;

mod effect_sampler2d;
mod effect_texture2d;

pub mod shell_node;
pub mod frame_time;
pub mod plugin;
pub mod engine_shell;
pub mod object;
pub mod run_stage;
pub mod setup;
pub mod assets;
pub mod prelude;
mod entity_ref;
mod animation;
mod interpolation;
mod extends;

pub struct DispatchEnd(pub ShareMutex<bool>);

impl Default for DispatchEnd {
    fn default() -> Self {
        Self(ShareMutex::new(true))
    }
}

pub struct PreFrameTime(pub Arc<ShareMutex< pi_time::Instant>>);
pub struct FrameStartTime(pub  pi_time::Instant);
impl Default for FrameStartTime {
    fn default() -> Self {
        Self( pi_time::Instant::now())
    }
}

impl Default for PreFrameTime {
    fn default() -> Self {
        Self(Arc::new(ShareMutex::new( pi_time::Instant::now())))
    }
}
