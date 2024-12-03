use pi_atom::Atom;
use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};
use crate::materials::prelude::*;

use super::base::*;

pub struct ShaderShadowGenerator;
impl ShaderShadowGenerator {
    pub const KEY: &'static str = "ShadowGenerator";
    pub fn res(engineopt: &EngineCustomPlugins) -> ShaderEffectMeta {
        let mut result = ShaderEffectMeta::new(
            ShaderEffectValueUniformDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                // mat2_list: vec![],
                vec4_list: vec![],
                vec3_list: vec![],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_DEPTH_BIAS), 0., false),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_NORMAL_BIAS), 0., false),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_DEPTH_SCALE), 50., false),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_MINZ), 1., false),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_MAXZ), 1001., false),
                ],
                // int_list: vec![],
                uint_list: vec![],
            },
            vec![],
            Varyings(
                vec![
                    Varying { format: Atom::from("float"), name: Atom::from("vDepthMetricSM") }
                ]
            ),
            String::from(""),
            // EVerticeExtendCode::default(),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from("
    vec3 position = A_POSITION;
    vec3 normal = A_NORMAL;
    mat4 finalWorld = PI_ObjectToWorld;
    mat3 normWorldSM = mat3(finalWorld);

    vec3 positionUpdated = position;
    vec4 worldPos = finalWorld*vec4(positionUpdated, 1.0);

    vec3 vNormalW = normalize(normWorldSM*normal);
    vec3 worldLightDirSM = normalize(
        PI_MATRIX_P[3][3] * PI_MATRIX_P[2].xyz
        +
        (1.0 - PI_MATRIX_P[3][3]) * (PI_CAMERA_POSITION.xyz - worldPos.xyz)
    );
    float ndlSM = dot(vNormalW, worldLightDirSM);
    float sinNLSM = sqrt(1.0-ndlSM*ndlSM);
    float normalBiasSM = matParam.uShadowNormalBias*sinNLSM;
    worldPos.xyz -= vNormalW*normalBiasSM;

    gl_Position = PI_MATRIX_VP *worldPos;
    vDepthMetricSM = (gl_Position.z + matParam.uShadowMinZ) / matParam.uShadowMaxZ + matParam.uShadowDepthBias ;
"
                )
            },
            BlockCodeAtom { 
                define: Atom::from("
layout(location = 0) out vec4 gl_FragColor;
"), 
                running: Atom::from("
gl_FragColor = vec4(vDepthMetricSM, 0.0, 0.0, 0.0);
"
                )
            },
            ShaderDefinesSet::default(),
            engineopt
        );

        result.binddefines = result.binddefines | BindDefines::MODEL_BIND | BindDefines::EFFECT_VALUE_BIND | BindDefines::SCENE_EFFECT | BindDefines::VIEWER;

        result
    }
}
