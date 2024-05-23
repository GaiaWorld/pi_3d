use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::{NodeMaterialBuilder, TNodeMaterialBlock, BlockUVOffsetSpeed, BlockMainTexture, BlockMainTextureUVOffsetSpeed, BlockOpacity, BlockEmissiveTexture}, NodeMaterialBlocks};
use pi_scene_context::prelude::ActionMaterial;

pub struct ShaderWater;
impl ShaderWater {
    pub const KEY: &'static str = "ShaderWater";

    pub fn meta(nodeblocks: &mut NodeMaterialBlocks) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.stage = wgpu::ShaderStages::VERTEX_FRAGMENT;
        nodemat.vs_define = String::from("
"
        );
        nodemat.varyings.0.push(Varying { format: Atom::from("vec4"), name: Atom::from("v_pos_SS") });
        nodemat.varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("vUV") });
        nodemat.varyings.0.push(Varying { format: Atom::from("vec4"), name: Atom::from("vColor") });

        nodemat.fs_define = String::from("
    layout(location = 0) out vec4 gl_FragColor;
    float Random1DTo1D(float value,float a,float b){
        //make value more random by making it bigger
        float random = fract(sin(value+b)*a);
        return random;
    }
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
        vec2 screenUV           = v_pos_SS.xy / v_pos_SS.w * 0.5 + 0.5;
    
        baseColor.rgb           *= mainColor();
        
        vec4 mainTextureColor   = mainTexture(vUV * 20., applyUVOffsetSpeed(uMainUVOS) + vec2(Random1DTo1D(screenUV.x * 200., PI_Time.y, .762), Random1DTo1D(screenUV.y * 200., PI_Time.y, .762)));
    
        vec4 emissiveTexture    = emissiveTexture(screenUV, vec2(0., 0.));
        float dDepth            = emissiveTexture.r + mainTextureColor.r * 0.01 * mainStrength();
        dDepth                  = dDepth - (v_pos_SS.z);
        if (dDepth < 0.) {
            discard;
        }
        // dDepth *= 0.02;
        // dDepth += (sin(v_pos_SS.x * 10. + PI_Time.y) * sin(v_pos_SS.y * 10. + PI_Time.y) + 0.5) * 0.5;

        vec4 finalColor = vec4(baseColor.rgb, alpha);
        finalColor = mix(finalColor, vec4(0.1, 0.5, 0.6, 0.2), smoothstep(0., 0.15, dDepth));
        // finalColor = vec4(dDepth, 0., 0., 1.);
    
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
        // app.add_system(Startup, setup.after(pi_pbr::setup));
        app.add_startup_system(Update, setup);
        app.add_startup_system(Update, pi_pbr::setup);
    }
}

pub fn main() {}