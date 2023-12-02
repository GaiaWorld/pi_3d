use pi_node_materials::prelude::{TNodeMaterialBlock, BlockFloat, BlockViewDirection};
use pi_shadow_mapping::BlockShadowMapping;

use crate::{brdf::NMBlockBRDF, prelude::NMBlockFresnel};


pub struct NMBlackSurfaceLighting;
impl TNodeMaterialBlock for NMBlackSurfaceLighting {
    const KEY: &'static str = "NMBlackSurfaceLighting";
    const FS_DEFINED: &'static str = include_str!("./surface_lighting.hlsl");
    const BIND_DEFINES: pi_engine_shell::prelude::BindDefine = pi_engine_shell::prelude::BindDefines::LIGHTING;
    fn depends() -> Vec<pi_atom::Atom> {
        vec![
            pi_atom::Atom::from(BlockFloat::KEY),
        ]
    }
}

pub struct NMBlackPBRLighting;
impl TNodeMaterialBlock for NMBlackPBRLighting {
    const KEY: &'static str = "NMBlackPBRLighting";
    const FS_DEFINED: &'static str = include_str!("./lighting.hlsl");
    const BIND_DEFINES: pi_engine_shell::prelude::BindDefine = pi_engine_shell::prelude::BindDefines::LIGHTING;
    fn depends() -> Vec<pi_atom::Atom> {
        vec![
            pi_atom::Atom::from(BlockFloat::KEY),
            pi_atom::Atom::from(BlockShadowMapping::KEY),
            pi_atom::Atom::from(BlockViewDirection::KEY),
            pi_atom::Atom::from(NMBlockBRDF::KEY),
            pi_atom::Atom::from(NMBlockFresnel::KEY),
            pi_atom::Atom::from(NMBlackSurfaceLighting::KEY),
        ]
    }
}