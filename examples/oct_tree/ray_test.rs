#![feature(box_into_inner)]

use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{frame_time::SingleFrameTimeCommand, prelude::*};

use pi_mesh_builder::cube::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;

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
    mut list: ResMut<ActionListTestData>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>, TypeAnimeContexts, ),
) {
    let tes_size = 20;
    fps.frame_ms = 16;
    let mut anime_contexts = assets.4;

    

    let demopass = DemoScene::new(
        &mut commands,
        &mut actions,
        &mut animegroupres,
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        1.,
        0.7,
        (0., 10., -40.),
        false,
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);
    actions.camera
        .target
        .push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    let source = commands.spawn_empty().id();
    actions.transform
        .tree
        .push(OpsTransformNodeParent::ops(source, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(
        scene,
        source,
        MeshInstanceState {
            instance_matrix: true,
            ..Default::default()
        },
    ));
    // actions.mesh.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));

    let id_geo = commands.spawn_empty().id();
    let attrs = CubeBuilder::attrs_meta();
    actions.geometry.create.push(OpsGeomeryCreate::ops(
        source,
        id_geo,
        attrs,
        Some(CubeBuilder::indices_meta()),
    ));

    let idmat = defaultmat.0;
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));

    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    // let cell_col = 4.;
    // let cell_row = 4.;
    for i in 0..tes_size {
        for j in 0..tes_size {
            for _k in 0..1 {
                let cube: Entity = commands.spawn_empty().id();
                actions.instance
                    .create
                    .push(OpsInstanceMeshCreation::ops(source, cube));
                actions.transform
                    .tree
                    .push(OpsTransformNodeParent::ops(cube, source));

                actions.transform
                    .localpos
                    .push(OpsTransformNodeLocalPosition::ops(
                        cube,
                        i as f32 * 2. - (tes_size) as f32,
                        0.,
                        j as f32 * 2. - (tes_size) as f32,
                    ));

                let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                let key_curve0 = key_curve0.asset_u64();
                let curve = FrameCurve::<LocalScaling>::curve_easing(
                    LocalScaling(Vector3::new(1., 1., 1.)),
                    LocalScaling(Vector3::new(0., 2. * (1.1 + (i as f32).sin()), 0.)),
                    (60. * (1.1 + ((i * j) as f32).cos())) as FrameIndex,
                    30,
                    EEasingMode::None,
                );

                let asset_curve = if let Some(curve) = anime_assets.scaling.get(&key_curve0) {
                    curve
                } else {
                    match anime_assets
                        .scaling
                        .insert(key_curve0, TypeFrameCurve(curve))
                    {
                        Ok(value) => value,
                        Err(_) => {
                            break;
                        }
                    }
                };

                let animation = anime_contexts
                    .scaling
                    .ctx
                    .create_animation(0, AssetTypeFrameCurve::from(asset_curve));
                actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), source, animation));
                // engine.create_target_animation(source, cube, &key_group, animation);
            }
        }
    }

    let q = LocalRotationQuaternion::create(0., -0.9, 0., 0.1);
    // log::warn!("Q: {:?}", q.0 * 0.5);

    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE));

    list.0.push((scene, camera01, 0.5, 0.5));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

#[derive(Resource, Default)]
pub struct ActionListTestData(Vec<(ObjectID, Entity, f32, f32)>);

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginRayTest);

        app.insert_resource(ActionListTestData::default());
    }
}

pub fn sys_test(
    mut list: ResMut<ActionListTestData>,
    mut rays: ResMut<ActionListRayTest>,
    res: Res<RayTestID>,
) {
    list.0
        .iter_mut()
        .for_each(|item| rays.push(RayTest(item.0, item.1, item.2, item.3)));
    println!("res: {:?}", res.as_ref());
}
pub fn main() {
    let mut app = base::test_plugins();

    app.add_plugins(PluginTest);

    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    app.add_systems(Update, sys_test);
    app.world
        .get_resource_mut::<StateRecordCfg>()
        .unwrap()
        .write_state = false;

    // bevy_mod_debugdump::print_main_schedule(&mut app);

    // app.run()
    loop {
        app.update();
    }
}
