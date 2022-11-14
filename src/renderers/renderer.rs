use super::{render_object::{RenderObjectTransparentList, RenderObjectOpaqueList}, render_mode::ERenderMode};

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

#[derive(Default)]
pub struct RenderList {
    opaque: RenderObjectOpaqueList,
    skybox: RenderObjectOpaqueList,
    alphatest: RenderObjectOpaqueList,
    transparent: RenderObjectTransparentList,
}

pub struct Renderer {
    pass_list: [RenderList; PASS_TAG_16 as usize],
    pub render_pass_tags: Vec<TPassTag>,
}

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

impl Renderer {
    pub fn record(
        pass: TPassTag,
        mode: ERenderMode
    ) {

    }
}