use brdf::NMBlockBRDF;
use fresnel::NMBlockFresnel;
use lighting::{NMBlackSurfaceLighting, NMBlackPBRLighting};
use pi_engine_shell::prelude::*;
use pi_node_materials::NodeMaterialBlocks;
use principled_brdf::PrincipledBRDF;
use reflectivity::NMBlockReflectivity;

use crate::reflection::NMBlockReflection;

mod brdf;
mod reflectivity;
mod reflection;
mod principled_brdf;
mod lighting;
mod fresnel;
pub mod prelude;

pub fn setup(
    mut blocks: ResMut<NodeMaterialBlocks>,
) {
    blocks.regist::<NMBlockBRDF>();
    blocks.regist::<NMBlockFresnel>();
    blocks.regist::<NMBlockReflectivity>();
    blocks.regist::<NMBlockReflection>();
    blocks.regist::<NMBlackSurfaceLighting>();
    blocks.regist::<NMBlackPBRLighting>();
    blocks.regist::<PrincipledBRDF>();
    log::warn!("PluginPBR: PrincipledBRDF !!!!");
}

pub struct PluginPBR;
impl Plugin for PluginPBR {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}