use std::mem::replace;

use pi_bevy_render_plugin::constant::render_state::{CompareFunction, StencilFaceState};
use pi_engine_shell::prelude::*;
use pi_render::renderer::pipeline::{DepthStencilState, DepthBiasState};


pub enum OpsDepthStencil {
    Write(Entity, bool),
    Compare(Entity, pi_bevy_render_plugin::constant::render_state::CompareFunction),
    StencilReadMask(Entity, u32),
    StencilWriteMask(Entity, u32),
}
impl OpsDepthStencil {
    pub fn ops_depth_write(entity: Entity, val: bool) -> Self {
        Self::Write(entity, val)
    }
    pub fn ops_depth_compare(entity: Entity, val: bool) -> Self {
        Self::Write(entity, val)
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct DepthWrite(pub bool);
impl Default for DepthWrite {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct DepthCompare(pub wgpu::CompareFunction);
impl Default for DepthCompare {
    fn default() -> Self {
        Self(wgpu::CompareFunction::GreaterEqual)
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct DepthBias(pub DepthBiasState);
impl Default for DepthBias {
    fn default() -> Self {
        Self(
            DepthBiasState {
                constant: 0,
                slope_scale: 0,
                clamp: 0,
            }
        )
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilFront(pub StencilFaceState);
impl Default for StencilFront {
    fn default() -> Self {
        Self(StencilFaceState::IGNORE)
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilBack(pub StencilFaceState);
impl Default for StencilBack {
    fn default() -> Self {
        Self(StencilFaceState::IGNORE)
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilRead(pub u32);
impl Default for StencilRead {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilWrite(pub u32);
impl Default for StencilWrite {
    fn default() -> Self {
        Self(0)
    }
}

pub fn depth_stencil_state(
    format: wgpu::TextureFormat,
    depth_write: &DepthWrite,
    compare: &DepthCompare,
    bias: &DepthBias,
    stencil_front: &StencilFront,
    stencil_back: &StencilBack,
    stencil_read: &StencilRead,
    stencil_write: &StencilWrite,
) -> DepthStencilState {
    DepthStencilState {
        format,
        depth_write_enabled: depth_write.0,
        depth_compare: compare.0,
        stencil: wgpu::StencilState {
            front: stencil_front.val(),
            back: stencil_back.val(),
            read_mask: stencil_read.0,
            write_mask: stencil_write.0,
        },
        bias: bias.0,
    }
}

// #[derive(Debug, Clone, Component)]
// pub struct ModelDepthStencil {
//     pub(crate) write: bool,
//     pub(crate) compare: wgpu::CompareFunction,
//     pub(crate) bias: DepthBiasState,
//     pub(crate) stencil: wgpu::StencilState,
// }
// impl ModelDepthStencil {
//     pub fn new(
//         write: bool,
//         compare: wgpu::CompareFunction,
//         bias: DepthBiasState,
//         stencil: wgpu::StencilState,
//     ) -> Self {
//         Self { write, compare, bias, stencil }
//     }
// }
// impl Default for ModelDepthStencil {
//     fn default() -> Self {
//         Self {
//             write: true,
//             compare: wgpu::CompareFunction::GreaterEqual,
//             bias: DepthBiasState {
//                 constant: 0,
//                 slope_scale: 0,
//                 clamp: 0,
//             },
//             stencil: wgpu::StencilState {
//                 front: wgpu::StencilFaceState::IGNORE,
//                 back: wgpu::StencilFaceState::IGNORE,
//                 read_mask: 0,
//                 write_mask: 0,
//             },
//         }
//     }
// }


// #[derive(Debug, Clone)]
// pub enum ERenderDepthAndStencilCommand {
//     Disable(),
//     DepthStencil(ModelDepthStencil),
// }

// pub struct ActionRenderDepthAndStencil;
// impl ActionRenderDepthAndStencil {
//     pub fn modify(
//         commands: &mut EntityCommands,
//         val: ERenderDepthAndStencilCommand,
//     ) {
//         match val {
//             ERenderDepthAndStencilCommand::Disable() => {
//                 commands.insert(ModelDepthStencil::default());
//             },
//             ERenderDepthAndStencilCommand::DepthStencil(value) => {
//                 commands.insert(value);
//             },
//         }
//     }
// }