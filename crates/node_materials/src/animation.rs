use pi_scene_math::*;
use pi_engine_shell::prelude::*;

use pi_scene_context::{prelude::*};

use crate::{
    main_tex::*,
    opacity::*,
    mask_texture::*,
    cutoff::BlockCutoff
};

#[derive(Debug, Component)]
pub struct MaterialAnimeSlots {
    pub main_tex_tilloff: u8,
    pub opacity_tex_tilloff: u8,
    pub mask_tex_tilloff: u8,
    pub main_color: u8,
    pub alpha: u8,
    pub light_diffuse: u8,
    pub cutoff: u8,
    pub mask_cutoff: u8,
    pub cell_id: u8,
}
impl Default for MaterialAnimeSlots {
    fn default() -> Self {
        Self {
            main_tex_tilloff:       u8::MAX,
            opacity_tex_tilloff:    u8::MAX,
            mask_tex_tilloff:       u8::MAX,
            main_color:             u8::MAX,
            alpha:                  u8::MAX,
            light_diffuse:          u8::MAX,
            cutoff:                 u8::MAX,
            mask_cutoff:            u8::MAX,
            cell_id:                u8::MAX,
        }
    }
}
impl MaterialAnimeSlots {
    pub fn new(meta: &BindEffectValues) -> Self {
        let mut result = Self::default();
        if let Some(slot) = meta.slot(&Atom::from(BlockMainTexture::KEY_TILLOFF)) {
            result.main_tex_tilloff = slot as u8;
        }
        
        if let Some(slot) = meta.slot(&Atom::from(BlockOpacityTexture::KEY_TILLOFF)) {
            result.opacity_tex_tilloff = slot as u8;
        }

        if let Some(slot) = meta.slot(&Atom::from(BlockMaskTexture::KEY_TILLOFF)) {
            result.mask_tex_tilloff = slot as u8;
        }

        if let Some(slot) = meta.slot(&Atom::from(BlockMainTexture::KEY_COLOR)) {
            result.main_color = slot as u8;
        }

        if let Some(slot) = meta.slot(&Atom::from(BlockOpacity::KEY_ALPHA)) {
            result.alpha = slot as u8;
        }
        
        if let Some(slot) = meta.slot(&Atom::from(BlockCutoff::KEY_VALUE)) {
            result.cutoff = slot as u8;
        }

        result
    }
}

pub trait TMaterialAnimeValue {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues);
}

#[derive(Debug, Clone, Copy, Component)]
pub struct MainTexUScale(pub f32);
impl Default for MainTexUScale { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for MainTexUScale {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.main_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.main_tex_tilloff as usize, 0, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MainTexUScale {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct MainTexVScale(pub f32);
impl Default for MainTexVScale { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for MainTexVScale {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.main_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.main_tex_tilloff as usize, 1, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MainTexVScale {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct MainTexUOffset(pub f32);
impl TMaterialAnimeValue for MainTexUOffset {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.main_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.main_tex_tilloff as usize, 2, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MainTexUOffset {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct MainTexVOffset(pub f32);
impl TMaterialAnimeValue for MainTexVOffset {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.main_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.main_tex_tilloff as usize, 3, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MainTexVOffset {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct OpacityTexUScale(pub f32);
impl Default for OpacityTexUScale { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for OpacityTexUScale {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.opacity_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.opacity_tex_tilloff as usize, 0, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for OpacityTexUScale {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct OpacityTexVScale(pub f32);
impl Default for OpacityTexVScale { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for OpacityTexVScale {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.opacity_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.opacity_tex_tilloff as usize, 1, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for OpacityTexVScale {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct OpacityTexUOffset(pub f32);
impl TMaterialAnimeValue for OpacityTexUOffset {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.opacity_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.opacity_tex_tilloff as usize, 2, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for OpacityTexUOffset {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct OpacityTexVOffset(pub f32);
impl TMaterialAnimeValue for OpacityTexVOffset {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.opacity_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.opacity_tex_tilloff as usize, 3, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for OpacityTexVOffset {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct MaskTexUScale(pub f32);
impl Default for MaskTexUScale { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for MaskTexUScale {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.mask_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.mask_tex_tilloff as usize, 0, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MaskTexUScale {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct MaskTexVScale(pub f32);
impl Default for MaskTexVScale { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for MaskTexVScale {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.mask_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.mask_tex_tilloff as usize, 1, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MaskTexVScale {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct MaskTexUOffset(pub f32);
impl TMaterialAnimeValue for MaskTexUOffset {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.mask_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.mask_tex_tilloff as usize, 2, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MaskTexUOffset {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct MaskTexVOffset(pub f32);
impl TMaterialAnimeValue for MaskTexVOffset {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.mask_tex_tilloff < u8::MAX {
            bind.vec4_one(slots.mask_tex_tilloff as usize, 3, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MaskTexVOffset {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct MainColor(pub Vector3);
impl Default for MainColor { fn default() -> Self { Self(Vector3::new(1., 1., 1.)) } }
impl TMaterialAnimeValue for MainColor {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.main_color < u8::MAX {
            bind.vec4_one(slots.main_color as usize, 0, self.0.x);
            bind.vec4_one(slots.main_color as usize, 1, self.0.y);
            bind.vec4_one(slots.main_color as usize,2, self.0.z);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MainColor {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Vector3::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Alpha(pub f32);
impl Default for Alpha { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for Alpha {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.alpha < u8::MAX {
            bind.float(slots.alpha as usize, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for Alpha {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct LightDiffuse(pub Vector3);
impl Default for LightDiffuse { fn default() -> Self { Self(Vector3::new(1., 1., 1.)) } }
impl TMaterialAnimeValue for LightDiffuse {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.light_diffuse < u8::MAX {
            bind.vec4_one(slots.light_diffuse as usize, 0, self.0.x);
            bind.vec4_one(slots.light_diffuse as usize, 1, self.0.y);
            bind.vec4_one(slots.light_diffuse as usize, 2, self.0.z);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for LightDiffuse {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Vector3::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Cutoff(pub f32);
impl TMaterialAnimeValue for Cutoff {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.cutoff < u8::MAX {
            bind.float(slots.cutoff as usize, self.0);
            // log::warn!("Anime: Cutoff {:?}", self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for Cutoff {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct MaskCutoff(pub f32);
impl TMaterialAnimeValue for MaskCutoff {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        if slots.mask_cutoff < u8::MAX {
            bind.float(slots.mask_cutoff as usize, self.0);
        }
    }
}
impl pi_curves::curve::frame::FrameDataValue for MaskCutoff {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct CellId(pub f32);

#[derive(Debug)]
pub struct Bundle (
    MainTexUScale,
    MainTexVScale,
    MainTexUOffset,
    MainTexVOffset,
    OpacityTexUScale,
    OpacityTexVScale,
    OpacityTexUOffset,
    OpacityTexVOffset,
    MaskTexUScale,
    MaskTexVScale,
    MaskTexUOffset,
    MaskTexVOffset,
    MainColor,
    Alpha,
    LightDiffuse,
    Cutoff,
    MaskCutoff,
    CellId,
);
impl Bundle {
    pub fn init(cmds: &mut EntityCommands) {
        cmds
            .insert(MainTexUScale::default())
            .insert(MainTexVScale::default())
            .insert(MainTexUOffset::default())
            .insert(MainTexVOffset::default())
            .insert(OpacityTexUScale::default())
            .insert(OpacityTexVScale::default())
            .insert(OpacityTexUOffset::default())
            .insert(OpacityTexVOffset::default())
            .insert(MaskTexUScale::default())
            .insert(MaskTexVScale::default())
            .insert(MaskTexUOffset::default())
            .insert(MaskTexVOffset::default())
            .insert(MainColor::default())
            .insert(Alpha::default())
            .insert(LightDiffuse::default())
            .insert(Cutoff::default())
            .insert(MaskCutoff::default())
            .insert(CellId::default())
            ;
    }
}
