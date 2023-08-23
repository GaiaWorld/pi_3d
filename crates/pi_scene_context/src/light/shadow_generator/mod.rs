use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use crate::materials::prelude::*;

use self::base::*;


pub mod base;
pub mod system;


// pub struct PluginShadowGenerator;
// impl Plugin for PluginShadowGenerator {
//     // fn init(
//     //     &mut self,
//     //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
//     //     stages: &mut pi_engine_shell::run_stage::RunStage,
//     // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
//     //     let entity = engine.new_object();
//     //     engine.regist_material_meta(KeyShaderMeta::from(ShaderShadowGenerator::KEY), ShaderShadowGenerator::res());

//     //     let world = engine.world_mut();

//     //     SysShadowParamUpdate::setup(world, stages.query_stage::<SysShadowParamUpdate>(ERunStageChap::Command));
//     //     SysShadowParamUpdateWhileMatCreate::setup(world, stages.query_stage::<SysShadowParamUpdateWhileMatCreate>(ERunStageChap::Command));
//     //     SysShadowGeneratorAppyWhileShadowModify::setup(world, stages.query_stage::<SysShadowGeneratorAppyWhileShadowModify>(ERunStageChap::Command));

//     //     Ok(())
//     // }

//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             (
//                 sys_shadow_param_update,
//                 sys_shadow_param_update_while_mat_create,
//                 sys_shadow_generator_apply_while_shadow_modify
//             ).chain()
//         );
//     }
// }

pub struct ShaderShadowGenerator;
impl ShaderShadowGenerator {
    pub const KEY: &'static str = "ShadowGenerator";
    pub fn res() -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            ShaderEffectValueUniformDesc {
                stage: wgpu::ShaderStages::VERTEX_FRAGMENT,
                mat4_list: vec![],
                // mat2_list: vec![],
                vec4_list: vec![],
                vec2_list: vec![],
                float_list: vec![
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_DEPTH_BIAS), 0.),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_NORMAL_BIAS), 0.),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_DEPTH_SCALE), 50.),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_MINZ), 1.),
                    UniformPropertyFloat(Atom::from(KEY_SHADOW_MAXZ), 1001.),
                ],
                // int_list: vec![],
                uint_list: vec![],
            },
            vec![],
            Varyings(
                vec![
                    Varying { 
                        format: Atom::from("float"),
                        name: Atom::from("vDepthMetricSM"),
                    },
                ]
            ),
            BlockCodeAtom { 
                define: Atom::from(""), 
                running: Atom::from("
                vec3 position = A_POSITION;
                mat4 finalWorld = PI_ObjectToWorld;

                    vec3 positionUpdated = position;
                    vec4 worldPos = finalWorld*vec4(positionUpdated, 1.0);
                    gl_Position = PI_MATRIX_VP*worldPos;
                    vDepthMetricSM = (gl_Position.z+uShadowMinZ)/uShadowMaxZ + uShadowDepthBias;
                ")
            },
            BlockCodeAtom { 
                define: Atom::from("
layout(location = 0) out vec4 gl_FragColor;
                "), 
                running: Atom::from("
                    float depthSM = vDepthMetricSM;
                    gl_FragColor = vec4(depthSM, 0.0, 0.0, 1.0);
                ")
            },
            ShaderDefinesSet::default()
        )
    }
}
