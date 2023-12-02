use pi_engine_shell::prelude::*;
use pi_node_materials::{prelude::{NodeMaterialBuilder, TNodeMaterialBlock, BlockUVOffsetSpeed, BlockMainTexture, BlockMainTextureUVOffsetSpeed, BlockEmissiveTexture, BlockOpacity}, NodeMaterialBlocks};
use pi_scene_context::prelude::ActionMaterial;

pub struct ShaderDistortion;
impl ShaderDistortion {
    pub const KEY: &'static str = "ShaderDistortion";

    pub fn meta(nodeblocks: &mut NodeMaterialBlocks) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.stage = wgpu::ShaderStages::VERTEX_FRAGMENT;
        nodemat.effect_varying_while_instance = String::from("
        ");
        nodemat.check_instance = EVerticeExtendCode(EVerticeExtendCode::NONE);
        nodemat.vs_define = String::from("
layout(location = 0) out vec4 v_color;
layout(location = 1) out vec2 v_uv;
layout(location = 2) out vec4 v_pos_SS;
#define P v_pos
#define N v_normal
");
        nodemat.fs_define = String::from("
layout(location = 0) in vec4 v_color;
layout(location = 1) in vec2 v_uv;
layout(location = 2) in vec4 v_pos_SS;
layout(location = 0) out vec4 gl_FragColor;
#define P v_pos
#define N v_normal
");

        nodemat.vs = String::from("
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;

    v_color = A_COLOR4;
    v_uv = A_UV;
    v_pos_SS = gl_Position;
        ");
        nodemat.fs = String::from("
    vec4 baseColor          = v_color;
    float alpha             = opacity() * baseColor.a;

    baseColor.rgb           *= mainColor();
    
    vec4 mainTextureColor   = mainTexture(v_uv, applyUVOffsetSpeed(uMainUVOS));

    vec2 screenUV           = v_pos_SS.xy / v_pos_SS.w * 0.5 + 0.5;
    vec4 emissiveTexture    = emissiveTexture(screenUV + (mainTextureColor.rg - 0.5) * 0.01 * mainStrength(), vec2(0., 0.));
    baseColor.rgb           *= emissiveTexture.rgb * emissiveStrength();
    alpha                   *= emissiveTexture.a;

    gl_FragColor = vec4(baseColor.rgb, alpha);
        ");

        // nodemat.apply::<BlockUVOffsetSpeed>();
        // nodemat.apply::<BlockMainTexture>();
        // nodemat.apply::<BlockMainTextureUVOffsetSpeed>();
        // nodemat.apply::<BlockViewDirection>();
        // nodemat.apply::<BlockShadowMapping>();
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
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderDistortion::KEY), ShaderDistortion::meta(&mut nodematblocks));
    log::warn!("PluginPBRMaterial Regist!!!");
}

pub struct PluginDistortionMaterial;
impl Plugin for PluginDistortionMaterial {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.after(pi_pbr::setup));
    }
}

pub fn main() {}