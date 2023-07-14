
use pi_engine_shell::prelude::*;

pub use crate::animation::*;
// use crate::animation_sys::PluginMaterialAnime;
pub use crate::cutoff::*;
pub use crate::common::*;
pub use crate::math::*;
pub use crate::render::*;
pub use crate::lighting::*;
pub use crate::fresnel:: {
    fresnel::*,
    emissive_fresnel::*,
    opacity_fresnel::*,
};
pub use crate::base::*;
pub use crate::emissive::{
    emissive_texture::*,
    emissive_texture_uv_offset_speed::*,
};
pub use crate::main_tex::*;
pub use crate::opacity::*;
pub use crate::fog::*;
pub use crate::mix_texture::*;
pub use crate::mask_texture::*;
pub use crate::premultiply::*;
pub use crate::{PluginNodeMaterial, PluginGroupNodeMaterialAnime};

#[derive(Resource)]
pub struct AssetCfgMaterialAnime(pub AssetCapacity);
impl Default for AssetCfgMaterialAnime {
    fn default() -> Self {
        Self(AssetCapacity { flag: false, min: 1024, max: 10 * 1024, timeout: 10 * 1000 })
    }
}
impl AsRef<AssetCapacity> for AssetCfgMaterialAnime {
    fn as_ref(&self) -> &AssetCapacity {
        &self.0
    }
}

pub type PluginAnimeMainTexUScale       = PluginTypeAnime<MainTexUScale,        RecordMainTexUScale     >;
pub type PluginAnimeMainTexVScale       = PluginTypeAnime<MainTexVScale,        RecordMainTexVScale     >;
pub type PluginAnimeMainTexUOffset      = PluginTypeAnime<MainTexUOffset,       RecordMainTexUOffset    >;
pub type PluginAnimeMainTexVOffset      = PluginTypeAnime<MainTexVOffset,       RecordMainTexVOffset    >;
pub type PluginAnimeOpacityTexUScale    = PluginTypeAnime<OpacityTexUScale,     RecordOpacityTexUScale  >;
pub type PluginAnimeOpacityTexVScale    = PluginTypeAnime<OpacityTexVScale,     RecordOpacityTexVScale  >;
pub type PluginAnimeOpacityTexUOffset   = PluginTypeAnime<OpacityTexUOffset,    RecordOpacityTexUOffset >;
pub type PluginAnimeOpacityTexVOffset   = PluginTypeAnime<OpacityTexVOffset,    RecordOpacityTexVOffset >;
pub type PluginAnimeMaskTexUScale       = PluginTypeAnime<MaskTexUScale,        RecordMaskTexUScale     >;
pub type PluginAnimeMaskTexVScale       = PluginTypeAnime<MaskTexVScale,        RecordMaskTexVScale     >;
pub type PluginAnimeMaskTexUOffset      = PluginTypeAnime<MaskTexUOffset,       RecordMaskTexUOffset    >;
pub type PluginAnimeMaskTexVOffset      = PluginTypeAnime<MaskTexVOffset,       RecordMaskTexVOffset    >;
pub type PluginAnimeMainColor           = PluginTypeAnime<MainColor,            RecordMainColor         >;
pub type PluginAnimeAlpha               = PluginTypeAnime<Alpha,                RecordAlpha             >;
pub type PluginAnimeCutoff              = PluginTypeAnime<Cutoff,               RecordCutoff            >;
pub type PluginAnimeMaskCutoff          = PluginTypeAnime<MaskCutoff,           RecordMaskCutoff        >;
pub type PluginAnimeLightDiffuse        = PluginTypeAnime<LightDiffuse,         RecordLightDiffuse      >;

#[derive(SystemParam)]
pub struct ActionSetMaterialAnime<'w> {
    pub main_tex_uscale:        (ResMut<'w, TypeAnimeContext<MainTexUScale>>,       Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUScale>>>,      ResMut<'w, TypeAnimeContextCounter<MainTexUScale>>           ),
    pub main_tex_vscale:        (ResMut<'w, TypeAnimeContext<MainTexVScale>>,       Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVScale>>>,      ResMut<'w, TypeAnimeContextCounter<MainTexVScale>>           ),
    pub main_tex_uoffset:       (ResMut<'w, TypeAnimeContext<MainTexUOffset>>,      Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUOffset>>>,     ResMut<'w, TypeAnimeContextCounter<MainTexUOffset>>           ),
    pub main_tex_voffset:       (ResMut<'w, TypeAnimeContext<MainTexVOffset>>,      Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVOffset>>>,     ResMut<'w, TypeAnimeContextCounter<MainTexVOffset>>           ),
    pub opacity_tex_uscale:     (ResMut<'w, TypeAnimeContext<OpacityTexUScale>>,    Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUScale>>>,   ResMut<'w, TypeAnimeContextCounter<OpacityTexUScale>>           ),
    pub opacity_tex_vscale:     (ResMut<'w, TypeAnimeContext<OpacityTexVScale>>,    Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVScale>>>,   ResMut<'w, TypeAnimeContextCounter<OpacityTexVScale>>           ),
    pub opacity_tex_uoffset:    (ResMut<'w, TypeAnimeContext<OpacityTexUOffset>>,   Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUOffset>>>,  ResMut<'w, TypeAnimeContextCounter<OpacityTexUOffset>>           ),
    pub opacity_tex_voffset:    (ResMut<'w, TypeAnimeContext<OpacityTexVOffset>>,   Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVOffset>>>,  ResMut<'w, TypeAnimeContextCounter<OpacityTexVOffset>>           ),
    pub mask_tex_uscale:        (ResMut<'w, TypeAnimeContext<MaskTexUScale>>,       Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUScale>>>,      ResMut<'w, TypeAnimeContextCounter<MaskTexUScale>>           ),
    pub mask_tex_vscale:        (ResMut<'w, TypeAnimeContext<MaskTexVScale>>,       Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVScale>>>,      ResMut<'w, TypeAnimeContextCounter<MaskTexVScale>>           ),
    pub mask_tex_uoffset:       (ResMut<'w, TypeAnimeContext<MaskTexUOffset>>,      Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUOffset>>>,     ResMut<'w, TypeAnimeContextCounter<MaskTexUOffset>>           ),
    pub mask_tex_voffset:       (ResMut<'w, TypeAnimeContext<MaskTexVOffset>>,      Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVOffset>>>,     ResMut<'w, TypeAnimeContextCounter<MaskTexVOffset>>           ),
    pub main_color:             (ResMut<'w, TypeAnimeContext<MainColor>>,           Res<'w, ShareAssetMgr<TypeFrameCurve<MainColor>>>,          ResMut<'w, TypeAnimeContextCounter<MainColor>>           ),
    pub alpha:                  (ResMut<'w, TypeAnimeContext<Alpha>>,               Res<'w, ShareAssetMgr<TypeFrameCurve<Alpha>>>,              ResMut<'w, TypeAnimeContextCounter<Alpha>>           ),
    pub cutoff:                 (ResMut<'w, TypeAnimeContext<Cutoff>>,              Res<'w, ShareAssetMgr<TypeFrameCurve<Cutoff>>>,             ResMut<'w, TypeAnimeContextCounter<Cutoff>>           ),
    pub light_diffuse:          (ResMut<'w, TypeAnimeContext<LightDiffuse>>,        Res<'w, ShareAssetMgr<TypeFrameCurve<LightDiffuse>>>,       ResMut<'w, TypeAnimeContextCounter<LightDiffuse>>           ),
    pub mask_cutoff:            (ResMut<'w, TypeAnimeContext<MaskCutoff>>,          Res<'w, ShareAssetMgr<TypeFrameCurve<MaskCutoff>>>,         ResMut<'w, TypeAnimeContextCounter<MaskCutoff>>           ),
    // pub cell_id:            (ResMut<'w, TypeAnimeContext<MainTexUScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUScale>>>),
}