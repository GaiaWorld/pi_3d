
use pi_engine_shell::prelude::*;

pub use crate::animation::*;
use crate::animation_sys::PluginMaterialAnime;
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

pub type PluginAnimeMainTexUScale       = PluginMaterialAnime<MainTexUScale, AssetCfgMaterialAnime>;
pub type PluginAnimeMainTexVScale       = PluginMaterialAnime<MainTexVScale, AssetCfgMaterialAnime>;
pub type PluginAnimeMainTexUOffset      = PluginMaterialAnime<MainTexUOffset, AssetCfgMaterialAnime>;
pub type PluginAnimeMainTexVOffset      = PluginMaterialAnime<MainTexVOffset, AssetCfgMaterialAnime>;

pub type PluginAnimeOpacityTexUScale    = PluginMaterialAnime<OpacityTexUScale, AssetCfgMaterialAnime>;
pub type PluginAnimeOpacityTexVScale    = PluginMaterialAnime<OpacityTexVScale, AssetCfgMaterialAnime>;
pub type PluginAnimeOpacityTexUOffset   = PluginMaterialAnime<OpacityTexUOffset, AssetCfgMaterialAnime>;
pub type PluginAnimeOpacityTexVOffset   = PluginMaterialAnime<OpacityTexVOffset, AssetCfgMaterialAnime>;

pub type PluginAnimeMaskTexUScale       = PluginMaterialAnime<MaskTexUScale, AssetCfgMaterialAnime>;
pub type PluginAnimeMaskTexVScale       = PluginMaterialAnime<MaskTexVScale, AssetCfgMaterialAnime>;
pub type PluginAnimeMaskTexUOffset      = PluginMaterialAnime<MaskTexUOffset, AssetCfgMaterialAnime>;
pub type PluginAnimeMaskTexVOffset      = PluginMaterialAnime<MaskTexVOffset, AssetCfgMaterialAnime>;

pub type PluginAnimeMainColor           = PluginMaterialAnime<MainColor, AssetCfgMaterialAnime>;
pub type PluginAnimeAlpha               = PluginMaterialAnime<Alpha, AssetCfgMaterialAnime>;
pub type PluginAnimeCutoff              = PluginMaterialAnime<Cutoff, AssetCfgMaterialAnime>;
pub type PluginAnimeMaskCutoff          = PluginMaterialAnime<MaskCutoff, AssetCfgMaterialAnime>;
pub type PluginAnimeLightDiffuse        = PluginMaterialAnime<LightDiffuse, AssetCfgMaterialAnime>;

#[derive(SystemParam)]
pub struct ActionSetMaterialAnime<'w> {
    pub main_tex_uscale:    (ResMut<'w, TypeAnimeContext<MainTexUScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUScale>>>),
    pub main_tex_vscale:    (ResMut<'w, TypeAnimeContext<MainTexVScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVScale>>>),
    pub main_tex_uoffset:   (ResMut<'w, TypeAnimeContext<MainTexUOffset>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUOffset>>>),
    pub main_tex_voffset:   (ResMut<'w, TypeAnimeContext<MainTexVOffset>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVOffset>>>),

    pub opacity_tex_uscale:     (ResMut<'w, TypeAnimeContext<OpacityTexUScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUScale>>>),
    pub opacity_tex_vscale:     (ResMut<'w, TypeAnimeContext<OpacityTexVScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVScale>>>),
    pub opacity_tex_uoffset:    (ResMut<'w, TypeAnimeContext<OpacityTexUOffset>>, Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUOffset>>>),
    pub opacity_tex_voffset:    (ResMut<'w, TypeAnimeContext<OpacityTexVOffset>>, Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVOffset>>>),

    pub mask_tex_uscale:    (ResMut<'w, TypeAnimeContext<MaskTexUScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUScale>>>),
    pub mask_tex_vscale:    (ResMut<'w, TypeAnimeContext<MaskTexVScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVScale>>>),
    pub mask_tex_uoffset:   (ResMut<'w, TypeAnimeContext<MaskTexUOffset>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUOffset>>>),
    pub mask_tex_voffset:   (ResMut<'w, TypeAnimeContext<MaskTexVOffset>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVOffset>>>),

    pub main_color:         (ResMut<'w, TypeAnimeContext<MainColor>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MainColor>>>),
    pub alpha:              (ResMut<'w, TypeAnimeContext<Alpha>>, Res<'w, ShareAssetMgr<TypeFrameCurve<Alpha>>>),
    pub cutoff:             (ResMut<'w, TypeAnimeContext<Cutoff>>, Res<'w, ShareAssetMgr<TypeFrameCurve<Cutoff>>>),
    pub light_diffuse:      (ResMut<'w, TypeAnimeContext<LightDiffuse>>, Res<'w, ShareAssetMgr<TypeFrameCurve<LightDiffuse>>>),
    pub mask_cutoff:        (ResMut<'w, TypeAnimeContext<MaskCutoff>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MaskCutoff>>>),
    // pub cell_id:            (ResMut<'w, TypeAnimeContext<MainTexUScale>>, Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUScale>>>),
}