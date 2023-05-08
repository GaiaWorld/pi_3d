use std::mem::replace;

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

#[derive(Debug, Clone, Component)]
pub struct ModelDepthStencil {
    pub(crate) write: bool,
    pub(crate) compare: wgpu::CompareFunction,
    pub(crate) bias: DepthBiasState,
    pub(crate) stencil: wgpu::StencilState,
}
impl ModelDepthStencil {
    pub fn new(
        write: bool,
        compare: wgpu::CompareFunction,
        bias: DepthBiasState,
        stencil: wgpu::StencilState,
    ) -> Self {
        Self { write, compare, bias, stencil }
    }
}
impl Default for ModelDepthStencil {
    fn default() -> Self {
        Self {
            write: true,
            compare: wgpu::CompareFunction::GreaterEqual,
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0,
                clamp: 0,
            },
            stencil: wgpu::StencilState {
                front: wgpu::StencilFaceState::IGNORE,
                back: wgpu::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }
    }
}


#[derive(Debug, Clone)]
pub enum ERenderDepthAndStencilCommand {
    Disable(),
    DepthStencil(ModelDepthStencil),
}

pub struct ActionRenderDepthAndStencil;
impl ActionRenderDepthAndStencil {
    pub fn modify(
        commands: &mut EntityCommands,
        val: ERenderDepthAndStencilCommand,
    ) {
        match val {
            ERenderDepthAndStencilCommand::Disable() => {
                commands.insert(ModelDepthStencil::default());
            },
            ERenderDepthAndStencilCommand::DepthStencil(value) => {
                commands.insert(value);
            },
        }
    }
}