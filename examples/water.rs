use pi_engine_shell::prelude::*;
use pi_node_materials::{prelude::{NodeMaterialBuilder, TNodeMaterialBlock, BlockUVOffsetSpeed, BlockMainTexture, BlockMainTextureUVOffsetSpeed, BlockOpacity, BlockEmissiveTexture}, NodeMaterialBlocks};
use pi_scene_context::prelude::ActionMaterial;

pub struct ShaderWater;
impl ShaderWater {
    pub const KEY: &'static str = "ShaderWater";

    pub fn meta(nodeblocks: &mut NodeMaterialBlocks) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.stage = wgpu::ShaderStages::VERTEX_FRAGMENT;
        nodemat.vs_define = String::from("
    layout(location = 0) out vec4 v_pos_SS;
    layout(location = 1) out vec2 vUV;
    layout(location = 2) out vec4 vColor;
"
        );
        nodemat.fs_define = String::from("
    layout(location = 0) out vec4 gl_FragColor;
    layout(location = 0) in vec4 v_pos_SS;
    layout(location = 1) in vec2 vUV;
    layout(location = 2) in vec4 vColor;
"
        );

        nodemat.vs = String::from("
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;

    vUV = A_UV;
    vColor = A_COLOR4;
    v_pos_SS = gl_Position;
");
        nodemat.fs = String::from("
        vec4 baseColor          = vColor;
        float alpha             = opacity() * baseColor.a;
    
        baseColor.rgb           *= mainColor();
        
        vec4 mainTextureColor   = mainTexture(vUV * 20., applyUVOffsetSpeed(uMainUVOS));
    
        vec2 screenUV           = v_pos_SS.xy / v_pos_SS.w * 0.5 + 0.5;
        vec4 emissiveTexture    = emissiveTexture(screenUV, vec2(0., 0.));
        float dDepth            = emissiveTexture.r + mainTextureColor.r * 0.01 * mainStrength();
        dDepth                  = dDepth - v_pos_SS.z;
        if (dDepth < 0.) {
            discard;
        }

        vec4 finalColor = vec4(baseColor.rgb, alpha);
        finalColor = mix(finalColor, vec4(0.1, 0.5, 0.6, 0.2), smoothstep(0., 0.5, dDepth));
    
        gl_FragColor = finalColor + mainTextureColor * 0.5;
");

        nodemat.include(&pi_atom::Atom::from(BlockUVOffsetSpeed::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockMainTexture::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockOpacity::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockMainTextureUVOffsetSpeed::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockEmissiveTexture::KEY), nodeblocks);

        nodemat.meta()
    }
}

pub fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderWater::KEY), ShaderWater::meta(&mut nodematblocks));
    log::warn!("PluginShaderWater Regist!!!");
}

pub struct PluginShaderWater;
impl Plugin for PluginShaderWater {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.after(pi_pbr::setup));
    }
}

pub fn main() {}