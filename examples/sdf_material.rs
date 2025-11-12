use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::{NodeMaterialBuilder, TNodeMaterialBlock, BlockUVOffsetSpeed, BlockMainTexture, BlockMainTextureUVOffsetSpeed, BlockOpacity, BlockEmissiveTexture}, NodeMaterialBlocks};
use pi_scene_context::prelude::ActionMaterial;

pub struct ShaderSDFFont;
impl ShaderSDFFont {
    pub const KEY: &'static str = "ShaderSDFFont";

    pub fn meta(nodeblocks: &mut NodeMaterialBlocks, engineopt: &EngineCustomPlugins) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.stage = wgpu::ShaderStages::bits( &wgpu::ShaderStages::VERTEX_FRAGMENT );
        nodemat.vs_define = String::from("
"
        );
        nodemat.varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("vUV") });
        nodemat.varyings.0.push(Varying { format: Atom::from("vec4"), name: Atom::from("vColor") });
        nodemat.varyings.0.push(Varying { format: Atom::from("vec4"), name: Atom::from("vFontParam") });

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

    vUV  = A_UV;// * matParam.uMainTilloff.xy + matParam.uMainTilloff.zw + applyUVOffsetSpeed(matParam.uMainUVOS);
    vColor = A_COLOR4;
    // v_pos_SS = gl_Position;
    // v_pos_SS.y = -v_pos_SS.y;
");
        nodemat.fs = String::from("
        vec4 baseColor          = vColor;
        float alpha             = opacity() * baseColor.a;
        // vec2 screenUV           = v_pos_SS.xy / v_pos_SS.w * 0.5 + 0.5;
    
        baseColor.rgb           *= mainColor();
        
        float SDF   = (mainTexture(vUV).r);
        SDF = smoothstep(vFontParam.x,vFontParam.y, SDF) * SDF;

        vec4 finalColor = vec4(baseColor.rgb * SDF, alpha * SDF);
    
        gl_FragColor = finalColor;
");

        nodemat.include(&pi_atom::Atom::from(BlockUVOffsetSpeed::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockMainTexture::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockOpacity::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockMainTextureUVOffsetSpeed::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockEmissiveTexture::KEY), nodeblocks);

        nodemat.meta(engineopt)
    }
}

pub fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
    engineopt: Res<EngineCustomPlugins>,
) {
    
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderSDFFont::KEY), ShaderSDFFont::meta(&mut nodematblocks, &engineopt));
}

pub struct PluginShaderSDFFont;
impl Plugin for PluginShaderSDFFont {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "use_bevy")]
        app.add_systems(Startup, setup.after(pi_pbr::setup));
        #[cfg(not(feature = "use_bevy"))]
        app.add_startup_system(Update, setup.after(pi_pbr::setup));
    }
}

pub fn main() {}