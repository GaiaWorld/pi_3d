
// use pi_engine_shell::prelude::*;

// use crate::pass::{PassRenderInfo, PassTag};

// /// 每个场景一个配置,场景创建时唯一确定,不可更改
// #[derive(Debug, Clone, Component)]
// pub struct ScenePassRenderCfg {
//     pub p01: PassRenderInfo,
//     pub p02: PassRenderInfo,
//     pub p03: PassRenderInfo,
//     pub p04: PassRenderInfo,
//     pub p05: PassRenderInfo,
//     pub p06: PassRenderInfo,
//     pub p07: PassRenderInfo,
//     pub p08: PassRenderInfo,
//     pub p09: PassRenderInfo,
//     pub p10: PassRenderInfo,
//     pub p11: PassRenderInfo,
//     pub p12: PassRenderInfo,
//     pub pass_opaque: PassTag,
//     pub pass_shadow: PassTag,
//     pub pass_opaque_texture: PassTag,
//     pub pass_depth_texture: PassTag,
// }
// impl Default for ScenePassRenderCfg {
//     fn default() -> Self {
//         Self {
//             p01: PassRenderInfo::shadow(),
//             p02: PassRenderInfo::opaque(),
//             p03: PassRenderInfo::opaque(),
//             p04: PassRenderInfo::opaque(),
//             p05: PassRenderInfo::normal(),
//             p06: PassRenderInfo::normal(),
//             p07: PassRenderInfo::normal(),
//             p08: PassRenderInfo::normal(),
//             p09: PassRenderInfo::normal(),
//             p10: PassRenderInfo::normal(),
//             p11: PassRenderInfo::normal(),
//             p12: PassRenderInfo::normal(),
//             pass_opaque: PassTag::PASS_TAG_03,
//             pass_shadow: PassTag::PASS_TAG_01,
//             pass_opaque_texture: PassTag::PASS_TAG_12,
//             pass_depth_texture: PassTag::PASS_TAG_12,
//         }
//     }
// }
// impl ScenePassRenderCfg {
//     pub fn query(&self, pass: PassTag) -> &PassRenderInfo {
//         match pass {
//             PassTag::PASS_TAG_01 => { &self.p01 },
//             PassTag::PASS_TAG_02 => { &self.p02 },
//             PassTag::PASS_TAG_03 => { &self.p03 },
//             PassTag::PASS_TAG_04 => { &self.p04 },
//             PassTag::PASS_TAG_05 => { &self.p05 },
//             PassTag::PASS_TAG_06 => { &self.p06 },
//             PassTag::PASS_TAG_07 => { &self.p07 },
//             PassTag::PASS_TAG_08 => { &self.p08 },
//             PassTag::PASS_TAG_09 => { &self.p09 },
//             PassTag::PASS_TAG_10 => { &self.p10 },
//             PassTag::PASS_TAG_11 => { &self.p11 },
//             PassTag::PASS_TAG_12 => { &self.p12 },
//             _ => { &self.p12 },
//         }
//     }
//     pub fn modify(&mut self, pass: PassTag, val: PassRenderInfo) {
//         match pass {
//             PassTag::PASS_TAG_01 => { self.p01 = val; },
//             PassTag::PASS_TAG_02 => { self.p02 = val; },
//             PassTag::PASS_TAG_03 => { self.p03 = val; },
//             PassTag::PASS_TAG_04 => { self.p04 = val; },
//             PassTag::PASS_TAG_05 => { self.p05 = val; },
//             PassTag::PASS_TAG_06 => { self.p06 = val; },
//             PassTag::PASS_TAG_07 => { self.p07 = val; },
//             PassTag::PASS_TAG_08 => { self.p08 = val; },
//             PassTag::PASS_TAG_09 => { self.p09 = val; },
//             PassTag::PASS_TAG_10 => { self.p10 = val; },
//             PassTag::PASS_TAG_11 => { self.p11 = val; },
//             PassTag::PASS_TAG_12 => { self.p12 = val; },
//             _ => { self.p12 = val; },
//         }
//     }
// }
