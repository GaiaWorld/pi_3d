use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::{BlockMainTexture, BlockMainTextureUVOffsetSpeed, BlockUVAtlas, BlockUVOffsetSpeed, NodeMaterialBuilder, TNodeMaterialBlock}, NodeMaterialBlocks};
use pi_scene_context::prelude::ActionMaterial;

pub struct ShaderPBR;
impl ShaderPBR {
    pub const KEY: &'static str = "ShaderPBR";

    pub fn meta(nodeblocks: &mut NodeMaterialBlocks) -> ShaderEffectMeta {

        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.stage = wgpu::ShaderStages::VERTEX_FRAGMENT;
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from("uMetallic"), 0.2, true));
        nodemat.values.float_list.push(UniformPropertyFloat(Atom::from("uRoughness"), 0.8, true));
        nodemat.material_instance_code = String::from("");
        // nodemat.check_instance = EVerticeExtendCode(EVerticeExtendCode::INSTANCE_CUSTOM_VEC4_A);
        nodemat.fs_define = String::from("
    layout(location = 0) out vec4 gl_FragColor;
    struct InputParam {
        float3  albedo;
        float   metallic;
        float   roughness;
        float3  emission;
        float   emissionStrength;
        float3  bumpTexture;
        float   bumpScale;
        float3  detailNormal;
        float   detailBumpLevel;
        float3  ambientOcc;
        float3  ambientColor;
        float4  lightingIntensities;
        float   IOR;
        float   F0;
        float4  metallicReflectanceColor;
        float   alpha;
        float   cutoff;
    };
#define P v_pos
#define N v_normal
");

        nodemat.vs = String::from("
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(A_POSITION, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;

    v_pos = worldPos;

    mat3 normalWorld = mat3(finalWorld);
    v_normal = normalize(normalWorld * A_NORMAL);

    v_color = A_COLOR4;
    v_uv = A_UV;
        ");
        nodemat.fs = String::from("

    float3 normal = normalize(v_normal);
    float3 geometricNormal = v_normal;
    
    float depth                 = 0.;
    float dither                = 0.;
    
	// float3 lightmapColor        = GetLightmapColor(GI_FRAGMENT_DATA(input));
	float3 lightmapColor        = vec3(0., 0., 0.);

    vec3 V                      = normalize(PI_CAMERA_POSITION.xyz - P.xyz); // WorldSpaceViewDir(P.xyz);
    float NdotV                 = saturate(abs(dot(normal, V)));

    vec4 baseColor              = v_color;
    float alpha                 = baseColor.a;

    vec4 mainTextureColor       = mainTexture(v_uv, applyUVOffsetSpeed(uMainUVOS));
    baseColor.rgb               *= mainTextureColor.rgb * mainStrength();
    alpha                       *= mainTextureColor.a;

    InputParam inputParam; 
    inputParam.albedo                   = baseColor.rgb;
    inputParam.metallic                 = uMetallic;
    inputParam.roughness                = uRoughness;
    inputParam.emission                 = vec3(0., 0., 0.);
    inputParam.emissionStrength         = 0.0;
    inputParam.bumpTexture              = vec3(0., 0., 1.);
    inputParam.bumpScale                = .0;
    inputParam.detailNormal             = vec3(0., 0., 1.);
    inputParam.detailBumpLevel          = 0.0;
    inputParam.ambientOcc               = vec3(1., 1., 1.);
    inputParam.ambientColor             = vec3(1., 1., 1.) * PI_Ambient.xyz;
    inputParam.lightingIntensities      = vec4(1., 1., 1., 1.);
    inputParam.IOR                      = 1.0;
    inputParam.F0                       = 1.0;
    inputParam.metallicReflectanceColor = vec4(1., 1., 1., 1.);
    inputParam.alpha                    = 1.;
    inputParam.cutoff                   = 0.;
    
    // #ifdef AMBIENT
    // float3 ambientOcclusionForDirectDiffuse = lerp(float3(1., 1., 1.), inputParam.ambientOcc, GetAmbientTextureImpactOnAnalyticalLights());
    // #else
    float3 ambientOcclusionForDirectDiffuse = inputParam.ambientOcc;
    // #endif
    
    float ambientMonochrome     = clamp(dot(inputParam.ambientOcc, float3(0.2126, 0.7152, 0.0722)), 0., 1.);
    float seo = 1.0;
// #ifdef _RADIANCE_OCCLUSION
    seo = environmentRadianceOcclusion(ambientMonochrome, NdotV);
// #endif
    float eho = 1.0;
// #ifdef _HORIZON_OCCLUSION
    eho = environmentHorizonOcclusion(-V, normal, geometricNormal);
// #endif

    PrincipledBRDFInput brdfInput;
    brdfInput.baseColor                 = inputParam.albedo;
    brdfInput.subsurface                = 0.0;
    brdfInput.subsurfaceRadius          = float3(1.0, 0.2, 0.1);
    brdfInput.subsurfaceColor           = float3(1., 1., 1.);
    brdfInput.subsurfaceIOR             = 1.4;
    brdfInput.subsurfaceAnisotropy      = 0.0;
    brdfInput.metallic                  = inputParam.metallic;
    brdfInput.specular                  = 0.5;
    brdfInput.specularTint              = float3(1., 1., 1.);
    brdfInput.roughness                 = inputParam.roughness;
    brdfInput.anisotropic               = 0.;
    brdfInput.anisotropicRotation       = 0.;
    brdfInput.sheen                     = 0.;
    brdfInput.sheenTint                 = float3(0.5, 0.5, 0.5);
    brdfInput.clearCoat                 = 0.;
    brdfInput.clearcoatRoughness        = 0.03;
    brdfInput.IOR                       = inputParam.IOR;
    brdfInput.transmission              = 0.;
    brdfInput.transmissionRoughness     = 0.;
    brdfInput.emission                  = inputParam.emission;
    brdfInput.emissionStrength          = inputParam.emissionStrength;
    brdfInput.alpha                     = inputParam.alpha;
    brdfInput.normal                    = normal;
    brdfInput.clearcoatNormal           = normal;
    brdfInput.tangent                   = normal;
    brdfInput.metallicReflectanceColor  = inputParam.metallicReflectanceColor;

    float lightmapUsed = 0.0;
    PrincipledBRDFOutput pbrOutput     = ApplyPrincipledBRDF(
        brdfInput,
        P,
        inputParam.ambientColor,
        lightmapColor,
        V,
        geometricNormal,
        // asuint(unity_RenderingLayer.x),
        lightmapUsed
    );
    
    float4 finalColor   = float4(
        pbrOutput.ambient               * inputParam.ambientOcc
        + pbrOutput.diffuse             * ambientOcclusionForDirectDiffuse * inputParam.lightingIntensities.x
        + pbrOutput.specular            * inputParam.lightingIntensities.x * inputParam.lightingIntensities.w
        + pbrOutput.irradianceColor     * inputParam.lightingIntensities.z * inputParam.ambientOcc
        + pbrOutput.radiance            * inputParam.lightingIntensities.z * seo * eho
        + pbrOutput.emission            * inputParam.lightingIntensities.y
        + pbrOutput.sheen               * 1.0
        + pbrOutput.sheenRadiance       * 1.0
        + pbrOutput.clearcoat           * 1.0
        + pbrOutput.clearcoatRadiance   * 1.0
        + pbrOutput.refraction          * 1.0
        ,
        pbrOutput.alpha
    );
    finalColor.rgb = max(finalColor.rgb, float3(0., 0., 0.));
    // finalColor.rgb = normal;
    // finalColor.rgb = pbrOutput.specular;
    // finalColor.rgb = pbrOutput.diffuse;
    // finalColor.rgb = vec3(NdotV);

    gl_FragColor = finalColor;
        ");

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from("vec3"),
                    name: Atom::from("v_normal"),
                },
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_pos"),
                },
                Varying {
                    format: Atom::from("vec2"),
                    name: Atom::from("v_uv"),
                },
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]
        );

        // nodemat.apply::<BlockUVOffsetSpeed>();
        // nodemat.apply::<BlockMainTexture>();
        // nodemat.apply::<BlockMainTextureUVOffsetSpeed>();
        // nodemat.apply::<BlockViewDirection>();
        // nodemat.apply::<BlockShadowMapping>();
        nodemat.include(&pi_atom::Atom::from(BlockUVOffsetSpeed::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockUVAtlas::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockMainTexture::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(BlockMainTextureUVOffsetSpeed::KEY), nodeblocks);
        nodemat.include(&pi_atom::Atom::from(pi_pbr::prelude::PrincipledBRDF::KEY), nodeblocks);

        nodemat.meta()
    }
}

pub fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderPBR::KEY), ShaderPBR::meta(&mut nodematblocks));
    // log::warn!("PluginPBRMaterial Regist!!!");
}

pub struct PluginPBRMaterial;
impl Plugin for PluginPBRMaterial {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.after(pi_pbr::setup));
    }
}

pub fn main() {}