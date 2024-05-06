use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::{NodeMaterialBuilder, TNodeMaterialBlock, BlockUVOffsetSpeed, BlockMainTexture, BlockMainTextureUVOffsetSpeed}, NodeMaterialBlocks};
use pi_scene_context::prelude::ActionMaterial;

pub struct ShaderPreDepth;
impl ShaderPreDepth {
    pub const KEY: &'static str = "ShaderPreDepth";

    pub fn meta(nodeblocks: &mut NodeMaterialBlocks) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.stage = wgpu::ShaderStages::VERTEX_FRAGMENT;
        nodemat.binddefines = BindDefines::MODEL_BIND | BindDefines::VIEWER;
        nodemat.fs_define = String::from("
    layout(location = 0) out vec4 gl_FragColor;
    layout(location = 0) in vec4 vWorldPos;
");
nodemat.vs_define = String::from("
layout(location = 0) out vec4 vWorldPos;
");

        nodemat.vs = String::from("
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;

    float depth = gl_Position.z;
    vWorldPos = vec4(depth, depth, depth, 1.);
        ");
        nodemat.fs = String::from("
    gl_FragColor = vWorldPos;
        ");

        nodemat.meta()
    }
}

pub fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderPreDepth::KEY), ShaderPreDepth::meta(&mut nodematblocks));
    log::warn!("PluginShaderPreDepth Regist!!!");
}

pub struct PluginShaderPreDepth;
impl Plugin for PluginShaderPreDepth {
    fn build(&self, app: &mut App) {
        app.add_system(Startup, setup.after(pi_pbr::setup));
    }
}

pub fn main() {}