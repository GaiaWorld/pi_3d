use pi_node_materials::prelude::{TNodeMaterialBlock, BlockFloat};


pub struct NMBlockFresnel;
impl TNodeMaterialBlock for NMBlockFresnel {
    const KEY: &'static str = "NMBlockFresnel";
    const FS_DEFINED: &'static str = include_str!("./fresnel.hlsl");
    fn depends() -> Vec<pi_atom::Atom> {
        vec![
            pi_atom::Atom::from(BlockFloat::KEY)
        ]
    }
}