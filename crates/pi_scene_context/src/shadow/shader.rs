use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use crate::materials::prelude::*;

use super::base::*;

pub struct ShaderShadowGenerator;
impl ShaderShadowGenerator {
    pub const KEY: &'static str = "ShadowGenerator";
    pub fn res() -> ShaderEffectMeta {
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
                vec![]
            ),
            String::from(""),
            EVerticeExtendCode::default(),
            BlockCodeAtom { 
                define: Atom::from("
layout(location = 0) out float vDepthMetricSM;
"
                ), 
                running: Atom::from("
vec3 position = A_POSITION;
mat4 finalWorld = PI_ObjectToWorld;

vec3 positionUpdated = position;
vec4 worldPos = finalWorld*vec4(positionUpdated, 1.0);
gl_Position = PI_MATRIX_VP*worldPos;
vDepthMetricSM = gl_Position.z;
"
                )
            },
            BlockCodeAtom { 
                define: Atom::from("
layout(location = 0) out vec4 gl_FragColor;
layout(location = 0) in float vDepthMetricSM;
"), 
                running: Atom::from("
if (vDepthMetricSM <= 0.00001) {
    discard;
}
float depthSM = vDepthMetricSM * uShadowDepthScale;
gl_FragColor = vec4(depthSM, 0.0, 0.0, 0.0);
"
                )
            },
            ShaderDefinesSet::default(),
        );

        result.binddefines = result.binddefines | BindDefines::MODEL_BIND | BindDefines::EFFECT_VALUE_BIND | BindDefines::SCENE_EFFECT | BindDefines::VIEWER;

        result
    }
}
