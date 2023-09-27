
use pi_engine_shell::prelude::*;

use crate::pass::{PassRenderInfo, PassTag, EPassTag};

#[derive(Debug, Clone, Component)]
pub struct ScenePassRenderCfg {
    pub p01: PassRenderInfo,
    pub p02: PassRenderInfo,
    pub p03: PassRenderInfo,
    pub p04: PassRenderInfo,
    pub p05: PassRenderInfo,
    pub p06: PassRenderInfo,
    pub p07: PassRenderInfo,
    pub p08: PassRenderInfo,
}
impl Default for ScenePassRenderCfg {
    fn default() -> Self {
        Self {
            p01: PassRenderInfo::shadow(),
            p02: PassRenderInfo::opaque(),
            p03: PassRenderInfo::opaque(),
            p04: PassRenderInfo::opaque(),
            p05: PassRenderInfo::normal(),
            p06: PassRenderInfo::normal(),
            p07: PassRenderInfo::normal(),
            p08: PassRenderInfo::normal(),
        }
    }
}
impl ScenePassRenderCfg {
    pub fn query(&self, pass: PassTag) -> &PassRenderInfo {
        if pass == EPassTag::PASS_TAG_01 {
            &self.p01
        } else if pass == EPassTag::PASS_TAG_02 {
            &self.p02
        } else if pass == EPassTag::PASS_TAG_03 {
            &self.p03
        } else if pass == EPassTag::PASS_TAG_04 {
            &self.p04
        } else if pass == EPassTag::PASS_TAG_05 {
            &self.p05
        } else if pass == EPassTag::PASS_TAG_06 {
            &self.p06
        } else if pass == EPassTag::PASS_TAG_07 {
            &self.p07
        } else {
            &self.p08
        }
    }
    pub fn modify(&mut self, pass: PassTag, val: PassRenderInfo) {
        if pass == EPassTag::PASS_TAG_01 {
            self.p01 = val;
        } else if pass == EPassTag::PASS_TAG_02 {
            self.p02 = val;
        } else if pass == EPassTag::PASS_TAG_03 {
            self.p03 = val;
        } else if pass == EPassTag::PASS_TAG_04 {
            self.p04 = val;
        } else if pass == EPassTag::PASS_TAG_05 {
            self.p05 = val;
        } else if pass == EPassTag::PASS_TAG_06 {
            self.p06 = val;
        } else if pass == EPassTag::PASS_TAG_07 {
            self.p07 = val;
        } else {
            self.p08 = val;
        }
    }
}
