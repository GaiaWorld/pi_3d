#![feature(box_into_inner)]

use std::sync::Arc;


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
mod static_string;
mod entity_ref;
mod animation;
mod interpolation;
mod extends;
mod pass;
mod log;
mod error;
mod bind_defines;
mod lighting_shadow;
mod forward_rendering;
mod bind_groups;
mod binds;
mod shader;
mod pipeline;
mod custom_rendertarget;
mod batch;
mod vertex_buffer_loader;
mod vertices;
mod ecs;

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

pub fn vec_u8_to_f32_16(val: &Vec<u8>) -> [f32;16] {
    if val.len() >= 64 {
        let mut temp: [u8;64] = [0;64];
        for i in 0..64 {
            temp[i] = val[i];
        }
        unsafe {
            std::mem::transmute::<[u8;64], [f32;16]>(temp)
        }
    } else {
        [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.]
    }
}

pub fn vec_u8_to_f32_4(val: &Vec<u8>) -> [f32;4] {
    if val.len() >= 16 {
        let mut temp: [u8;16] = [0;16];
        for i in 0..16 {
            temp[i] = val[i];
        }
        unsafe {
            std::mem::transmute::<[u8;16], [f32;4]>(temp)
        }
    } else {
        [1., 0., 0., 0.]
    }
}

pub fn vec_u8_to_f32_2(val: &Vec<u8>) -> [f32;2] {
    if val.len() >= 8 {
        let mut temp: [u8;8] = [0;8];
        for i in 0..8 {
            temp[i] = val[i];
        }
        unsafe {
            std::mem::transmute::<[u8;8], [f32;2]>(temp)
        }
    } else {
        [0., 0.]
    }
}

pub fn vec_u8_to_f32(val: &Vec<u8>) -> f32 {
    if val.len() >= 4 {
        let mut temp: [u8;4] = [0;4];
        for i in 0..4 {
            temp[i] = val[i];
        }
        unsafe {
            std::mem::transmute::<[u8;4], f32>(temp)
        }
    } else {
        0.
    }
}

pub fn vec_u8_to_i32(val: &Vec<u8>) -> i32 {
    if val.len() >= 4 {
        let mut temp: [u8;4] = [0;4];
        for i in 0..4 {
            temp[i] = val[i];
        }
        unsafe {
            std::mem::transmute::<[u8;4], i32>(temp)
        }
    } else {
        0
    }
}

pub fn vec_u8_to_u32(val: &Vec<u8>) -> u32 {
    if val.len() >= 4 {
        let mut temp: [u8;4] = [0;4];
        for i in 0..4 {
            temp[i] = val[i];
        }
        unsafe {
            std::mem::transmute::<[u8;4], u32>(temp)
        }
    } else {
        0
    }
}