use animation_sys::sys_material_anime_init;
use base::{NodeMaterialBlockInfo, TNodeMaterialBlock};
use common::*;
use emissive::{emissive_base::BlockEmissiveBase, emissive_texture::BlockEmissiveTexture};
use fresnel::{opacity_fresnel::BlockOpacityFresnel, emissive_fresnel::BlockEmissiveFresnel};
use main_tex::BlockMainTexture;
use opacity::BlockOpacityTexture;
use pi_hash::XHashMap;
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::sys_act_material_create;
use prelude::*;

mod cutoff;
mod common;
mod math;
mod render;
mod lighting;
mod fresnel;
mod base;
mod emissive;
mod main_tex;
mod mix_texture;
mod mask_texture;
mod opacity;
mod fog;
mod animation;
pub mod prelude;
pub mod animation_sys;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct NodeMaterialBlocks(pub XHashMap<&'static str, NodeMaterialBlockInfo>);
impl NodeMaterialBlocks {
    pub fn regist<T: TNodeMaterialBlock>(&mut self) {
        self.0.insert(T::KEY, T::info());
    }
}

pub struct PluginNodeMaterial;
impl Plugin for PluginNodeMaterial {
    fn build(&self, app: &mut App) {
        let mut blocks = NodeMaterialBlocks::default();

        blocks.regist::<BlockFloat>();
        blocks.regist::<BlockViewDirection>();

        blocks.regist::<BlockColorSpace>();
        blocks.regist::<BlockColorGray>();
        blocks.regist::<BlockColorHSV>();
        blocks.regist::<BlockTextureChannel>();
        blocks.regist::<BlockUVOffsetSpeed>();
        
        blocks.regist::<BlockFog>();
        
        blocks.regist::<BlockCutoff>();

        blocks.regist::<BlockMainTexture>();
        blocks.regist::<BlockMainTextureUVOffsetSpeed>();

        blocks.regist::<BlockOpacity>();
        blocks.regist::<BlockOpacityTexture>();
        blocks.regist::<BlockOpacityTextureUVOffsetSpeed>();
        
        blocks.regist::<BlockOpacity2Texture>();
        blocks.regist::<BlockOpacity2TextureUVOffsetSpeed>();

        blocks.regist::<BlockEmissiveBase>();
        blocks.regist::<BlockEmissiveTexture>();
        
        blocks.regist::<BlockMixTexture>();
        blocks.regist::<BlockMixTextureUVOffsetSpeed>();

        blocks.regist::<BlockMaskTexture>();
        blocks.regist::<BlockMaskTextureUVOffsetSpeed>();

        blocks.regist::<BlockFresnel>();
        blocks.regist::<BlockEmissiveFresnel>();
        blocks.regist::<BlockOpacityFresnel>();

        app.insert_resource(blocks);

        app.add_system(sys_material_anime_init.after(sys_act_material_create));
    }
}

pub struct PluginGroupNodeMaterialAnime;
impl PluginGroup for PluginGroupNodeMaterialAnime {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();

        group
            .add(PluginAnimeMainTexUScale       ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMainTexVScale       ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMainTexUOffset      ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMainTexVOffset      ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeOpacityTexUScale    ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeOpacityTexVScale    ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeOpacityTexUOffset   ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeOpacityTexVOffset   ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMaskTexUScale       ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMaskTexVScale       ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMaskTexUOffset      ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMaskTexVOffset      ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMainColor           ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeAlpha               ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeCutoff              ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeMaskCutoff          ::new(false, 1 * 1024 * 1024, 1000))
            .add(PluginAnimeLightDiffuse        ::new(false, 1 * 1024 * 1024, 1000))

    }
}