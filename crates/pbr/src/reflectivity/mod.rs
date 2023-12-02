use pi_node_materials::prelude::TNodeMaterialBlock;


pub struct NMBlockReflectivity;
impl TNodeMaterialBlock for NMBlockReflectivity {
    const KEY: &'static str = "NMBlockReflectivity";
    const FS_DEFINED: &'static str = include_str!("./reflectivity.hlsl");
    const SHADER_LANGUAGE_DEFINES: pi_node_materials::prelude::TShaderLanguageDefine = pi_node_materials::prelude::ShaderLanguageDefine::FLOATS_TO_VEC;
}