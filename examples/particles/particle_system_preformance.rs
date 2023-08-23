
use default_render::{SingleIDBaseDefaultMaterial, shader::DefaultShader};
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime,};
use pi_gltf2_load::{TypeAnimeContexts, TypeAnimeAssetMgrs};
use pi_node_materials::{NodeMaterialBlocks, PluginNodeMaterial};
use pi_scene_context::{prelude::*, viewer::prelude::{ViewerGlobalPosition, ViewerViewMatrix}};
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::*, quad::PluginQuadBuilder};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader};
use pi_particle_system::{prelude::*, PluginParticleSystem};

fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut final_render: ResMut<WindowRenderer>,
    mut renderercmds: ActionSetRenderer,
    mut particlesys_cmds: ParticleSystemActionSet,
    mut animegroupcmd: ActionSetAnimationGroup,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
) {
    let tes_size = 50;

    let (scene, camera01) = base::DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 0., -50.), true);
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));


    let temp = 4;
    for i in 0..temp {
        for j in 0..temp {
            for k in 0..temp {
                let item = {
                    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
                    meshcmds.create.push(OpsMeshCreation::ops(scene, source));
                    let id_geo = commands.spawn_empty().id();
                    let mut attrs = CubeBuilder::attrs_meta();
                    // ParticleSystem Add
                    attrs.push(VertexBufferDesc::instance_world_matrix());
                    attrs.push(VertexBufferDesc::instance_color());
                    attrs.push(VertexBufferDesc::instance_tilloff());
                    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));
                    //
                    let syskey = String::from("Test");
                    let syscfg = demo_cfg(10000., 50.);
                    let calculator = commands.spawn_empty().id();
                    particlesys_cmds.calculator_cmds.push(OpsCPUParticleCalculator::ops(calculator, syscfg));
                    let particle_sys_calculator = ParticleSystemCalculatorID(calculator, 1024, particlesys_cmds.calculator_queue.queue());
                    let calculator = particlesys_cmds.calcultors.insert(syskey.asset_u64(), particle_sys_calculator).unwrap();
                    particlesys_cmds.particlesys_cmds.push(OpsCPUParticleSystem::ops(source, source, calculator, 100));
                    particlesys_cmds.particlesys_state_cmds.push(OpsCPUParticleSystemState::ops_start(source));
                    //
                    let idmat = commands.spawn_empty().id();
                    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
                    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Opaque));
                    source
                };

                transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(item, (i - temp / 2) as f32 * 10., (j - temp / 2) as f32 * 10., (k - temp / 2) as f32 * 10.));
                transformcmds.localrot.push(OpsTransformNodeLocalEuler::ops(item, (i - temp) as f32, (j - temp) as f32, (k - temp) as f32));
                transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(item, 0.2, 0.2, 0.2));
            }
        }
    }

    // matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from("_MainTex"),
    //     filter: true,
    //     sample: KeySampler::default(),
    //     url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    // }));

    
    let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(node, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, node, id_group));
    {
        let key_curve0 =  pi_atom::Atom::from("test2"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(3.1415926 * 2., 3.1415926 * 2., 3.1415926 * 2.)), (60.) as FrameIndex, 30, EEasingMode::None);
        
        let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) {
            curve
        } else {
            match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => {
                    value
                },
                Err(_) => {
                    return;
                },
            }
        };

        let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, node, id_group.clone(), animation);
    }

    let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 0.1;
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), param, 0., pi_animation::base::EFillMode::NONE);
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

fn demo_cfg(count: f32, speed: f32) -> IParticleSystemConfig {
    let mut cfg = IParticleSystemConfig::new();

    cfg.name = String::from("Test");
    cfg.duration = 1.0;
    cfg.looping = 1;
    cfg.max_particles = count;
    cfg.emission = (count, None);
    cfg.start_speed = OneParamInfo::TInterpolateConstant(speed);
    cfg.start_color = FourGradientInfo::TInterpolateRandom;
    cfg.color_over_lifetime = Some(FourGradientInfo::TInterpolateRandom);
    cfg.lifetime = OneParamInfo::TInterpolateConstant(1.);
    cfg.shape = IShape::ShapeCone(IShapeCone::default());

    cfg
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}



#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugins(PluginTest);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}