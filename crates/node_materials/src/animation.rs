use pi_scene_math::*;
use pi_engine_shell::prelude::*;

use pi_scene_context::prelude::*;

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

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMainTexUScale(pub MainTexUScale);
impl TAnimatableCompRecord<MainTexUScale> for RecordMainTexUScale{
    fn comp(&self) -> MainTexUScale {
        self.0.clone()
    }
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
impl TAssetCapacity for MainTexUScale {
    const ASSET_TYPE: &'static str = "AnimeMainTexUScale";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MainTexUScale {}


#[derive(Clone, Copy, Component, Default)]
pub struct RecordMainTexVScale(pub MainTexVScale);
impl TAnimatableCompRecord<MainTexVScale> for RecordMainTexVScale{
    fn comp(&self) -> MainTexVScale {
        self.0.clone()
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
impl TAssetCapacity for MainTexVScale {
    const ASSET_TYPE: &'static str = "AnimeMainTexVScale";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MainTexVScale {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMainTexUOffset(pub MainTexUOffset);
impl TAnimatableCompRecord<MainTexUOffset> for RecordMainTexUOffset{
    fn comp(&self) -> MainTexUOffset {
        self.0.clone()
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
impl TAssetCapacity for MainTexUOffset {
    const ASSET_TYPE: &'static str = "AnimeMainTexUOffset";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MainTexUOffset {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMainTexVOffset(pub MainTexVOffset);
impl TAnimatableCompRecord<MainTexVOffset> for RecordMainTexVOffset {
    fn comp(&self) -> MainTexVOffset {
        self.0.clone()
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
impl TAssetCapacity for MainTexVOffset {
    const ASSET_TYPE: &'static str = "AnimeMainTexVOffset";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MainTexVOffset {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordOpacityTexUScale(pub OpacityTexUScale);
impl TAnimatableCompRecord<OpacityTexUScale> for RecordOpacityTexUScale {
    fn comp(&self) -> OpacityTexUScale {
        self.0.clone()
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
impl TAssetCapacity for OpacityTexUScale {
    const ASSET_TYPE: &'static str = "AnimeOpacityTexUScale";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for OpacityTexUScale {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordOpacityTexVScale(pub OpacityTexVScale);
impl TAnimatableCompRecord<OpacityTexVScale> for RecordOpacityTexVScale {
    fn comp(&self) -> OpacityTexVScale {
        self.0.clone()
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
impl TAssetCapacity for OpacityTexVScale {
    const ASSET_TYPE: &'static str = "AnimeOpacityTexVScale";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for OpacityTexVScale {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordOpacityTexUOffset(pub OpacityTexUOffset);
impl TAnimatableCompRecord<OpacityTexUOffset> for RecordOpacityTexUOffset {
    fn comp(&self) -> OpacityTexUOffset {
        self.0.clone()
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
impl TAssetCapacity for OpacityTexUOffset {
    const ASSET_TYPE: &'static str = "AnimeOpacityTexUOffset";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for OpacityTexUOffset {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordOpacityTexVOffset(pub OpacityTexVOffset);
impl TAnimatableCompRecord<OpacityTexVOffset> for RecordOpacityTexVOffset {
    fn comp(&self) -> OpacityTexVOffset {
        self.0.clone()
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
impl TAssetCapacity for OpacityTexVOffset {
    const ASSET_TYPE: &'static str = "AnimeOpacityTexVOffset";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for OpacityTexVOffset {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMaskTexUScale(pub MaskTexUScale);
impl TAnimatableCompRecord<MaskTexUScale> for RecordMaskTexUScale {
    fn comp(&self) -> MaskTexUScale {
        self.0.clone()
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
impl TAssetCapacity for MaskTexUScale {
    const ASSET_TYPE: &'static str = "AnimeMaskTexUScale";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MaskTexUScale {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMaskTexVScale(pub MaskTexVScale);
impl TAnimatableCompRecord<MaskTexVScale> for RecordMaskTexVScale {
    fn comp(&self) -> MaskTexVScale {
        self.0.clone()
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
impl TAssetCapacity for MaskTexVScale {
    const ASSET_TYPE: &'static str = "AnimeMaskTexVScale";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MaskTexVScale {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMaskTexUOffset(pub MaskTexUOffset);
impl TAnimatableCompRecord<MaskTexUOffset> for RecordMaskTexUOffset {
    fn comp(&self) -> MaskTexUOffset {
        self.0.clone()
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
impl TAssetCapacity for MaskTexUOffset {
    const ASSET_TYPE: &'static str = "AnimeMaskTexUOffset";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MaskTexUOffset {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMaskTexVOffset(pub MaskTexVOffset);
impl TAnimatableCompRecord<MaskTexVOffset> for RecordMaskTexVOffset {
    fn comp(&self) -> MaskTexVOffset {
        self.0.clone()
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
impl TAssetCapacity for MaskTexVOffset {
    const ASSET_TYPE: &'static str = "AnimeMaskTexVOffset";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MaskTexVOffset {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMainColor(pub MainColor);
impl TAnimatableCompRecord<MainColor> for RecordMainColor {
    fn comp(&self) -> MainColor {
        self.0.clone()
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
impl TAssetCapacity for MainColor {
    const ASSET_TYPE: &'static str = "AnimeMainColor";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MainColor {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordAlpha(pub Alpha);
impl TAnimatableCompRecord<Alpha> for RecordAlpha {
    fn comp(&self) -> Alpha {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Alpha(pub f32);
impl Default for Alpha { fn default() -> Self { Self(1.) } }
impl TMaterialAnimeValue for Alpha {
    fn apply(&self, slots: &MaterialAnimeSlots, bind: &BindEffectValues) {
        // log::warn!("Material Anime Apply Slot {}", slots.alpha);
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
impl TAssetCapacity for Alpha {
    const ASSET_TYPE: &'static str = "AnimeAlpha";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for Alpha {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordLightDiffuse(pub LightDiffuse);
impl TAnimatableCompRecord<LightDiffuse> for RecordLightDiffuse {
    fn comp(&self) -> LightDiffuse {
        self.0.clone()
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
impl TAssetCapacity for LightDiffuse {
    const ASSET_TYPE: &'static str = "AnimeLightDiffuse";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LightDiffuse {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordCutoff(pub Cutoff);
impl TAnimatableCompRecord<Cutoff> for RecordCutoff {
    fn comp(&self) -> Cutoff {
        self.0.clone()
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
impl TAssetCapacity for Cutoff {
    const ASSET_TYPE: &'static str = "AnimeCutoff";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for Cutoff {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordMaskCutoff(pub MaskCutoff);
impl TAnimatableCompRecord<MaskCutoff> for RecordMaskCutoff {
    fn comp(&self) -> MaskCutoff {
        self.0.clone()
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
impl TAssetCapacity for MaskCutoff {
    const ASSET_TYPE: &'static str = "AnimeMaskCutoff";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for MaskCutoff {}

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
    pub fn _init(cmds: &mut EntityCommands) {
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
