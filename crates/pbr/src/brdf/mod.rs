use pi_scene_shell::prelude::BindDefines;
use pi_node_materials::prelude::{TNodeMaterialBlock, BlockFloat};


pub struct NMBlockBRDF;
impl TNodeMaterialBlock for NMBlockBRDF {
    const KEY: &'static str = "NMBlockBRDF";
    const FS_DEFINED: &'static str = include_str!("./brdf.hlsl");
    const BIND_DEFINES: pi_scene_shell::prelude::BindDefine = BindDefines::ENVIRONMENT_BRDF_TEXTURE;
    fn depends() -> Vec<pi_atom::Atom> {
        vec![
            pi_atom::Atom::from(BlockFloat::KEY)
        ]
    }
}