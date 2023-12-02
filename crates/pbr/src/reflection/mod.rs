use pi_node_materials::prelude::{TNodeMaterialBlock, BlockColorSpace};


pub struct NMBlockReflection;
impl TNodeMaterialBlock for NMBlockReflection {
    const KEY: &'static str = "NMBlockReflection";
    const FS_DEFINED: &'static str = include_str!("./reflection.hlsl");
    const SHADER_LANGUAGE_DEFINES: pi_node_materials::prelude::TShaderLanguageDefine = pi_node_materials::prelude::ShaderLanguageDefine::FLOATS_TO_VEC;
    const BIND_DEFINES: pi_engine_shell::prelude::BindDefine = pi_engine_shell::prelude::BindDefines::ENVIRONMENT_LIGHTING;
    fn depends() -> Vec<pi_engine_shell::prelude::Atom> {
        vec![
            pi_engine_shell::prelude::Atom::from(BlockColorSpace::KEY)
        ]
    }
}