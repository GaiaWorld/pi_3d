use std::{fmt::Debug};

use pi_atom::Atom;
use pi_engine_shell::object::ObjectID;
use pi_hash::DefaultHasher;
use pi_render::graph::{graph::RenderGraph, NodeId};

use crate::{viewer::command::Viewport};

use super::{graphic::RenderNode, base::DrawList3D};


#[derive(Debug, Default)]
pub struct RendererHasher(pub DefaultHasher);

#[derive(Debug)]
pub struct RenderSize(pub(crate) u32, pub(crate) u32);
impl RenderSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self(width, height)
    }
    pub fn width(&self) -> u32 { self.0 }
    pub fn height(&self) -> u32 { self.1 }
}

pub struct RenderColorFormat(pub wgpu::TextureFormat);
pub struct RenderColorClear(pub wgpu::Color);
pub struct RenderDepthFormat(pub Option<wgpu::TextureFormat>);
pub struct RenderDepthClear(pub f32);

pub struct Renderer {
    pub ready: bool,
    pub viewport: Viewport,
    pub opaque_graphic: Option<NodeId>,
    pub draws: DrawList3D,
}
impl Renderer {
    pub fn new(
        name: &Atom,
        object_id: ObjectID,
        rg: &mut RenderGraph,
    ) -> Self {
        let opaque_graphic = rg.add_node(String::from(name.as_str()), RenderNode::new(object_id)).unwrap();
        Self {
            viewport: Viewport::default(),
            opaque_graphic: Some(opaque_graphic),
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