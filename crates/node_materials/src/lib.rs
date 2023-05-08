use base::{NodeMaterialBlockInfo, TNodeMaterialBlock};
use common::*;
use emissive::{emissive_base::BlockEmissiveBase, emissive_texture::BlockEmissiveTexture};
use fresnel::{opacity_fresnel::BlockOpacityFresnel, emissive_fresnel::BlockEmissiveFresnel};
use main_tex::BlockMainTexture;
use opacity::BlockOpacityTexture;
use pi_hash::XHashMap;
use pi_engine_shell::prelude::*;
use prelude::*;

pub mod common;
pub mod math;
pub mod render;
pub mod lighting;
pub mod fresnel;
pub mod base;
pub mod emissive;
pub mod main_tex;
pub mod mix_texture;
pub mod mask_texture;
pub mod opacity;
pub mod fog;
pub mod prelude;

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

        blocks.regist::<BlockMainTexture>();
        blocks.regist::<BlockMainTextureUVOffsetSpeed>();

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
    }
}