use std::{fmt::Debug};

use pi_atom::Atom;
use pi_bevy_render_plugin::NodeId;
use pi_engine_shell::prelude::*;
use pi_hash::DefaultHasher;

use crate::{viewer::command::Viewport};

use super::{graphic::RenderNode, base::DrawList3D};


#[derive(Debug, Default, Resource)]
pub struct RendererHasher(pub DefaultHasher);

#[derive(Component)]
pub struct RendererEnable(pub bool);

#[derive(Debug, Component)]
pub struct RenderSize(pub(crate) u32, pub(crate) u32);
impl RenderSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self(width, height)
    }
    pub fn width(&self) -> u32 { self.0 }
    pub fn height(&self) -> u32 { self.1 }
}

#[derive(Component)]
pub struct RenderColorFormat(pub wgpu::TextureFormat);
impl Default for RenderColorFormat {
    fn default() -> Self {
        Self(wgpu::TextureFormat::Rgba8Unorm)
    }
}

#[derive(Component)]
pub struct RenderColorClear(pub wgpu::Color);
impl Default for RenderColorClear {
    fn default() -> Self {
        Self(wgpu::Color { r: 0., g: 0., b: 0., a: 0. })
    }
}

#[derive(Component)]
pub struct RenderDepthFormat(pub Option<wgpu::TextureFormat>);
impl Default for RenderDepthFormat {
    fn default() -> Self {
        Self(Some(wgpu::TextureFormat::Depth24PlusStencil8))
    }
}

#[derive(Component)]
pub struct RenderDepthClear(pub f32);
impl Default for RenderDepthClear {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component)]
pub struct RenderStencilClear(pub u32);
impl Default for RenderStencilClear {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Component)]
pub struct RenderAutoClearColor(pub bool);
impl Default for RenderAutoClearColor {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Component)]
pub struct RenderAutoClearDepth(pub bool);
impl Default for RenderAutoClearDepth {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Component)]
pub struct RenderAutoClearStencil(pub bool);
impl Default for RenderAutoClearStencil {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Component)]
pub struct RenderToFinalTarget(pub bool);
impl Default for RenderToFinalTarget {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component)]
pub struct Renderer {
    pub ready: bool,
    pub viewport: Viewport,
    pub draws: DrawList3D,
}
impl Renderer {
    pub fn new() -> Self {
        Self {
            viewport: Viewport::default(),
            draws: DrawList3D { list: vec![], viewport: (0., 0., 1., 1., 0., 1.) },
            ready: false,
        }
    }
    pub fn clear(&mut self) {
        self.draws.list.clear();
        self.ready = false;
    }

    pub fn reset(&mut self) {
        self.ready = true;
    }
}