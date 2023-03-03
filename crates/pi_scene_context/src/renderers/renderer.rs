use std::{fmt::Debug};

use pi_atom::Atom;
use pi_engine_shell::object::ObjectID;
use pi_hash::DefaultHasher;
use pi_render::graph::{graph::RenderGraph, NodeId};

use crate::{viewer::command::Viewport};

use super::{graphic::RenderNode, base::DrawList3D};


#[derive(Debug, Default)]
pub struct RendererHasher(pub DefaultHasher);

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
            draws: DrawList3D { list: vec![], viewport: (0., 0., 10., 10., 0., 1.) },
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