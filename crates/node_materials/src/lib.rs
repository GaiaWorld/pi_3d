
use base::{NodeMaterialBlockInfo, TNodeMaterialBlock};
use common::*;
use emissive::emissive_texture::BlockEmissiveTexture;
use fresnel::{opacity_fresnel::BlockOpacityFresnel, emissive_fresnel::BlockEmissiveFresnel};
use main_tex::BlockMainTexture;
use opacity::BlockOpacityTexture;
use pi_hash::XHashMap;
use pi_engine_shell::prelude::*;
use prelude::*;
use premultiply::*;

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
mod premultiply;
mod command;
mod command_sys;
pub mod prelude;
pub mod animation_sys;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct NodeMaterialBlocks(pub XHashMap<Atom, NodeMaterialBlockInfo>);
impl NodeMaterialBlocks {
    pub fn regist<T: TNodeMaterialBlock>(&mut self) {
        self.0.insert(Atom::from(T::KEY), T::info());
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
// pub enum StageNodeMaterial {
//     InitAnimeAbout,
//     _InitAnimeAboutApply,
//     Command,
//     _CommandApply,
//     ApplyAnimeAbout,
// }

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

        blocks.regist::<BlockEmissiveTexture>();
        blocks.regist::<BlockEmissiveTextureUVOffsetSpeed>();
        
        blocks.regist::<BlockMixTexture>();
        blocks.regist::<BlockMixTextureUVOffsetSpeed>();

        blocks.regist::<BlockMaskTexture>();
        blocks.regist::<BlockMaskTextureUVOffsetSpeed>();

        blocks.regist::<BlockFresnel>();
        blocks.regist::<BlockEmissiveFresnel>();
        blocks.regist::<BlockOpacityFresnel>();

        blocks.regist::<BlockPremultiplyResult>();

        app.insert_resource(blocks);


    }
}

pub struct PluginGroupNodeMaterialAnime;
impl PluginGroup for PluginGroupNodeMaterialAnime {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
        // group
        //     .add(PluginAnimeMainTexUScale       ::new())
        //     .add(PluginAnimeMainTexVScale       ::new())
        //     .add(PluginAnimeMainTexUOffset      ::new())
        //     .add(PluginAnimeMainTexVOffset      ::new())
        //     .add(PluginAnimeOpacityTexUScale    ::new())
        //     .add(PluginAnimeOpacityTexVScale    ::new())
        //     .add(PluginAnimeOpacityTexUOffset   ::new())
        //     .add(PluginAnimeOpacityTexVOffset   ::new())
        //     .add(PluginAnimeMaskTexUScale       ::new())
        //     .add(PluginAnimeMaskTexVScale       ::new())
        //     .add(PluginAnimeMaskTexUOffset      ::new())
        //     .add(PluginAnimeMaskTexVOffset      ::new())
        //     .add(PluginAnimeMainColor           ::new())
        //     .add(PluginAnimeAlpha               ::new())
        //     .add(PluginAnimeCutoff              ::new())
        //     .add(PluginAnimeMaskCutoff          ::new())
        //     .add(PluginAnimeLightDiffuse        ::new())

    }
}