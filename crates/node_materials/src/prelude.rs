
use pi_scene_shell::prelude::*;

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
pub use crate::shadowmapping::*;
pub use crate::default_shader::*;
pub use crate::{PluginNodeMaterial, NodeMaterialBlocks};

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
