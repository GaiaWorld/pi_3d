use pi_engine_shell::prelude::*;

use crate::animation::*;


pub struct OpsMainTexTilloff(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsMainTexTilloff {
    pub fn ops(mat: Entity, uscale: f32, vscale: f32, uoffset: f32, voffset: f32) -> Self {
        Self(mat, uscale, vscale, uoffset, voffset)
    }
}
pub type ActionListMainTexTilloff = ActionList<OpsMainTexTilloff>;

pub struct OpsMaskTexTilloff(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsMaskTexTilloff {
    pub fn ops(mat: Entity, uscale: f32, vscale: f32, uoffset: f32, voffset: f32) -> Self {
        Self(mat, uscale, vscale, uoffset, voffset)
    }
}
pub type ActionListMaskTexTilloff = ActionList<OpsMaskTexTilloff>;

pub struct OpsOpacityTexTilloff(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsOpacityTexTilloff {
    pub fn ops(mat: Entity, uscale: f32, vscale: f32, uoffset: f32, voffset: f32) -> Self {
        Self(mat, uscale, vscale, uoffset, voffset)
    }
}
pub type ActionListOpacityTexTilloff = ActionList<OpsOpacityTexTilloff>;

pub struct OpsEmissiveTexTilloff(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsEmissiveTexTilloff {
    pub fn ops(mat: Entity, uscale: f32, vscale: f32, uoffset: f32, voffset: f32) -> Self {
        Self(mat, uscale, vscale, uoffset, voffset)
    }
}
pub type ActionListEmissiveTexTilloff = ActionList<OpsEmissiveTexTilloff>;

pub struct OpsAlpha(pub(crate) Entity, pub(crate) f32);
impl OpsAlpha {
    pub fn ops(mat: Entity, val: f32) -> Self {
        Self(mat, val)
    }
}
pub type ActionListAlpha = ActionList<OpsAlpha>;

pub struct OpsAlphaCutoff(pub(crate) Entity, pub(crate) f32);
impl OpsAlphaCutoff {
    pub fn ops(mat: Entity, val: f32) -> Self {
        Self(mat, val)
    }
}
pub type ActionListAlphaCutoff = ActionList<OpsAlphaCutoff>;

pub struct OpsMaskCutoff(pub(crate) Entity, pub(crate) f32);
impl OpsMaskCutoff {
    pub fn ops(mat: Entity, val: f32) -> Self {
        Self(mat, val)
    }
}
pub type ActionListMaskCutoff = ActionList<OpsMaskCutoff>;

pub struct OpsEmissiveColor(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsEmissiveColor {
    pub fn ops(mat: Entity, r: f32, g: f32, b: f32) -> Self {
        Self(mat, r, g, b)
    }
}
pub type ActionListEmissiveColor = ActionList<OpsEmissiveColor>;

pub struct OpsMainColor(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsMainColor {
    pub fn ops(mat: Entity, r: f32, g: f32, b: f32) -> Self {
        Self(mat, r, g, b)
    }
}
pub type ActionListMainColor = ActionList<OpsMainColor>;

pub struct OpsLightDiffuse(pub(crate) Entity, pub(crate) f32, pub(crate) f32, pub(crate) f32);
impl OpsLightDiffuse {
    pub fn ops(mat: Entity, r: f32, g: f32, b: f32) -> Self {
        Self(mat, r, g, b)
    }
}
pub type ActionListLightDiffuse = ActionList<OpsLightDiffuse>;

pub struct BundleNodeMaterialAnimatableComp (
    MaterialAnimeSlots,
    Alpha,
    Cutoff,
    MaskCutoff,
    MainColor,
    MainTexUOffset,
    MainTexUScale,
    MainTexVScale,
    MaskTexUOffset,
    MaskTexUScale,
    MaskTexVOffset,
    MaskTexVScale,
    OpacityTexUOffset,
    OpacityTexUScale,
    OpacityTexVOffset,
    OpacityTexVScale,
    RecordAlpha,
    RecordCutoff,
    RecordMaskCutoff,
    RecordMainColor,
    RecordMainTexUOffset,
    RecordMainTexUScale,
    RecordMainTexVScale,
    RecordMaskTexUOffset,
    RecordMaskTexUScale,
    RecordMaskTexVOffset,
    RecordMaskTexVScale,
    RecordOpacityTexUOffset,
    RecordOpacityTexUScale,
    RecordOpacityTexVOffset,
    RecordOpacityTexVScale,
);