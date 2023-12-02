use pi_node_materials::prelude::{TNodeMaterialBlock, BlockViewDirection};


pub struct BlockStandardLighting;
impl TNodeMaterialBlock for BlockStandardLighting {
    const KEY: &'static str = "StandardLighting";

    const FS_DEFINED: &'static str = include_str!("./lighting.glsl");

    const VS_DEFINED: &'static str = "";

    const BIND_DEFINES: pi_engine_shell::prelude::BindDefine = pi_engine_shell::prelude::BindDefines::LIGHTING;

    fn depends() -> Vec<pi_atom::Atom> {
        vec![
            pi_atom::Atom::from(BlockViewDirection::KEY)
        ]
    }
}