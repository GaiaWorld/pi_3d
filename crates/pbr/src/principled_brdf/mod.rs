use pi_node_materials::prelude::{TNodeMaterialBlock, BlockFloat};
use pi_shadow_mapping::BlockShadowMapping;

use crate::{lighting::NMBlackPBRLighting, reflectivity::NMBlockReflectivity, brdf::NMBlockBRDF, reflection::NMBlockReflection};


pub struct PrincipledBRDF;
impl TNodeMaterialBlock for PrincipledBRDF {
    const KEY: &'static str = "PrincipledBRDF";
    const FS_DEFINED: &'static str = include_str!("./principled_brdf.hlsl");
    fn depends() -> Vec<pi_atom::Atom> {
        vec![
            pi_atom::Atom::from(BlockFloat::KEY),
            pi_atom::Atom::from(BlockShadowMapping::KEY),
            pi_atom::Atom::from(NMBlockBRDF::KEY),
            pi_atom::Atom::from(NMBlackPBRLighting::KEY),
            pi_atom::Atom::from(NMBlockReflectivity::KEY),
            pi_atom::Atom::from(NMBlockReflection::KEY),
        ]
    }
    const SHADER_LANGUAGE_DEFINES: pi_node_materials::prelude::TShaderLanguageDefine = pi_node_materials::prelude::ShaderLanguageDefine::FLOATS_TO_VEC;
}