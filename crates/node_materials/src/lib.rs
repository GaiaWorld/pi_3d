
use base::{NodeMaterialBlockInfo, TNodeMaterialBlock};
use common::*;
use emissive::emissive_texture::BlockEmissiveTexture;
use fresnel::{opacity_fresnel::BlockOpacityFresnel, emissive_fresnel::BlockEmissiveFresnel};
use main_tex::BlockMainTexture;
use opacity::BlockOpacityTexture;
use pi_scene_shell::prelude::*;
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
mod shadowmapping;
mod default_shader;
pub mod prelude;
pub mod animation_sys;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct NodeMaterialBlocks(pub XHashMap<Atom, NodeMaterialBlockInfo>);
impl NodeMaterialBlocks {
    pub fn regist<T: TNodeMaterialBlock>(&mut self) {
        self.0.insert(Atom::from(T::KEY), T::info());
    }
}

pub struct PluginNodeMaterial;
impl Plugin for PluginNodeMaterial {
    fn build(&self, app: &mut App) {
        let mut blocks = NodeMaterialBlocks::default();

        blocks.regist::<BlockFloat>();
        blocks.regist::<BlockViewDirection>();
        blocks.regist::<BlockUVAtlas>();

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
