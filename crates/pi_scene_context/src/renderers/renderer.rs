use std::{hash::{Hash, Hasher}, fmt::Debug};

use pi_engine_shell::object::ObjectID;
use pi_hash::DefaultHasher;
use pi_render::graph::{graph::RenderGraph, NodeId};

use crate::{cameras::camera::CameraViewport, main_camera_render::graph::SingleMainCameraOpaqueRenderNode, viewer::command::Viewport};

use super::{render_object::{RenderObjectTransparentList, RenderObjectOpaqueList, RenderObjectBindGroup}};

pub type TPassTag = u16;

pub const PASS_TAG_01: TPassTag = 0b0000_0000_0000_0001;
pub const PASS_TAG_02: TPassTag = 0b0000_0000_0000_0010;
pub const PASS_TAG_03: TPassTag = 0b0000_0000_0000_0100;
pub const PASS_TAG_04: TPassTag = 0b0000_0000_0000_1000;
pub const PASS_TAG_05: TPassTag = 0b0000_0000_0001_0000;
pub const PASS_TAG_06: TPassTag = 0b0000_0000_0010_0000;
pub const PASS_TAG_07: TPassTag = 0b0000_0000_0100_0000;
pub const PASS_TAG_08: TPassTag = 0b0000_0000_1000_0000;
pub const PASS_TAG_09: TPassTag = 0b0000_0001_0000_0000;
pub const PASS_TAG_10: TPassTag = 0b0000_0010_0000_0000;
pub const PASS_TAG_11: TPassTag = 0b0000_0100_0000_0000;
pub const PASS_TAG_12: TPassTag = 0b0000_1000_0000_0000;
pub const PASS_TAG_13: TPassTag = 0b0001_0000_0000_0000;
pub const PASS_TAG_14: TPassTag = 0b0010_0000_0000_0000;
pub const PASS_TAG_15: TPassTag = 0b0100_0000_0000_0000;
pub const PASS_TAG_16: TPassTag = 0b1000_0000_0000_0000;

#[derive(Debug, Default)]
pub struct RendererHasher(pub DefaultHasher);

pub struct Renderer {
    pub ready: bool,
    pub viewport: Viewport,
    pub opaque_graphic: Option<NodeId>,
    pub skybox_graphic: Option<NodeId>,
    pub transparent_graphic: Option<NodeId>,
    pub opaque_draws: RenderObjectOpaqueList,
    pub skybox_draws: RenderObjectOpaqueList,
    pub transparent_draws: RenderObjectTransparentList,
}
impl Renderer {
    pub fn new(
        name: &'static str,
        object_id: ObjectID,
        rg: &mut RenderGraph,
    ) -> Self {
        let opaque_graphic = rg.add_node(name, SingleMainCameraOpaqueRenderNode::new(object_id)).unwrap();
        Self {
            viewport: Viewport::default(),
            opaque_graphic: Some(opaque_graphic),
            skybox_graphic: None,
            transparent_graphic: None,
            opaque_draws: RenderObjectOpaqueList::default(),
            skybox_draws: RenderObjectOpaqueList::default(),
            transparent_draws: RenderObjectTransparentList::default(),
            ready: false,
        }
    }
    pub fn clear(&mut self) {
        self.opaque_draws.bind_groups.clear();
        self.opaque_draws.draws.clear();

        self.skybox_draws.bind_groups.clear();
        self.skybox_draws.draws.clear();

        self.transparent_draws.bind_groups.clear();
        self.transparent_draws.draws.clear();
        self.ready = false;
    }

    pub fn remove(&mut self, obj: &ObjectID) {
        self.opaque_draws.remove(obj);
        self.skybox_draws.remove(obj);
        self.transparent_draws.remove(obj);
    }
    pub fn reset(&mut self, camera_bind_group: RenderObjectBindGroup) {
        self.opaque_draws.bind_groups.push(camera_bind_group.clone());

        self.skybox_draws.bind_groups.push(camera_bind_group.clone());

        self.transparent_draws.bind_groups.push(camera_bind_group);
        self.ready = true;
    }
}

#[derive(Default)]
pub struct RenderList {
    opaque: RenderObjectOpaqueList,
    skybox: RenderObjectOpaqueList,
    alphatest: RenderObjectOpaqueList,
    transparent: RenderObjectTransparentList,
}

// pub struct Renderer {
//     pass_list: [RenderList; PASS_TAG_16 as usize],
//     pub render_pass_tags: Vec<TPassTag>,
// }

// impl Default for Renderer {
//     fn default() -> Self {
//         Self {
//             pass_list: [
//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),


//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),

//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),

//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),
//                 RenderList::default(),
//             ],
//             render_pass_tags: vec![]
//         }
//     }
// }