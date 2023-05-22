use std::{fmt::Debug};

use pi_atom::Atom;
use pi_bevy_render_plugin::NodeId;
use pi_engine_shell::prelude::*;
use pi_hash::{DefaultHasher, XHashMap};

use crate::{viewer::prelude::*};

use super::{graphic::{RenderNode, RendererGraphicDesc}, base::DrawList3D, render_object::RendererID};


#[derive(Debug, Clone, Default, Resource)]
pub struct RendererHasher(pub DefaultHasher);

#[derive(Debug, Clone, Copy, Component)]
pub struct RendererEnable(pub bool);

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderSize(pub(crate) u32, pub(crate) u32);
impl RenderSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self(width, height)
    }
    pub fn width(&self) -> u32 { self.0 }
    pub fn height(&self) -> u32 { self.1 }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderColorFormat(pub ColorFormat);
impl Default for RenderColorFormat {
    fn default() -> Self {
        Self(ColorFormat::Rgba8Unorm)
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderColorClear(pub u8, pub u8, pub u8, pub u8);
impl Default for RenderColorClear {
    fn default() -> Self {
        Self(0, 0, 0, 0)
    }
}
impl RenderColorClear {
    pub fn color(&self) -> wgpu::Color {
        wgpu::Color { r: self.0 as f64 / 255.0, g: self.1 as f64 / 255.0, b: self.2 as f64 / 255.0, a: self.3 as f64 / 255.0 }
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderDepthFormat(pub Option<DepthStencilFormat>);
impl Default for RenderDepthFormat {
    fn default() -> Self {
        Self(Some(DepthStencilFormat::Depth32Float))
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderDepthClear(pub f32);
impl Default for RenderDepthClear {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderStencilClear(pub u32);
impl Default for RenderStencilClear {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderAutoClearColor(pub bool);
impl Default for RenderAutoClearColor {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderAutoClearDepth(pub bool);
impl Default for RenderAutoClearDepth {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderAutoClearStencil(pub bool);
impl Default for RenderAutoClearStencil {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Debug, Clone, Copy, Component)]
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

#[derive(Debug, Clone, Default, Component)]
pub struct ViewerRenderersInfo {
    pub map: XHashMap<Atom, (RendererGraphicDesc, RendererID)>,
}

#[derive(Component)]
pub struct DirtyViewerRenderersInfo;
