#![feature(box_into_inner)]


use pi_animation::loop_mode::ELoopMode;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::{prelude::*, frame_time::SingleFrameTimeCommand};

use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;

use crate::base::DemoScene;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 10;
    fps.frame_ms = 16;

    let demopass = DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        1., 0.7, (0., 10., -40.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));
    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = base::instance_attr(true, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let idmat = defaultmat.0;
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    for i in 0..tes_size {
        for j in 0..tes_size {
            for _k in 0..1 {
                
                let cube: Entity = commands.spawn_empty().id();
                actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                actions.transform.tree.push(OpsTransformNodeParent::ops(cube, source));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32)));
                
                let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                let key_curve0 = key_curve0.asset_u64();
                let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(1., 1., 1.)), LocalScaling(Vector3::new(0., 2. * (1.1 + (i as f32).sin()), 0.)), (60. * (1.1 + ((i * j) as f32).cos())) as FrameIndex, 30, EEasingMode::None);
                
                let asset_curve = if let Some(curve) = anime_assets.scaling.get(&key_curve0) {
                    curve
                } else {
                    match anime_assets.scaling.insert(key_curve0, TypeFrameCurve(curve)) {
                        Ok(value) => { value  },
                        Err(_) => { break; },
                    }
                };

                let animation = anime_contexts.scaling.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), cube,  animation));
                // engine.create_target_animation(source, cube, &key_group, animation);
            }
        }
    }

    let mut param = AnimationGroupParam::default(); param.loop_mode = ELoopMode::Positive(Some(1));
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    // animegroupres.scene_ctxs.stop(scene, id_group.clone());
    // let mut param = AnimationGroupParam::default(); param.loop_mode = ELoopMode::Positive(Some(1));
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, param, 0., pi_animation::base::EFillMode::NONE));
    // actions.anime.pause.push(OpsAnimationGroupPause::ops(id_group));
    // let mut param = AnimationGroupParam::default(); param.loop_mode = ELoopMode::Positive(Some(1));
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}


pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    

    // app.run()
    loop { app.update(); }

}