#![feature(box_into_inner)]



use base::DemoScene;
use pi_curves::curve::frame_curve::FrameCurve;
use pi_engine_shell::{prelude::*, frame_time::SingleFrameTimeCommand};

use pi_gltf2_load::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;

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
    let tes_size = 20;
    fps.frame_ms = 16;

    let demopass = DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        10., 0.7, (0., 0., -40.), false
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = base::instance_attr(true, true, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);
    actions.mesh.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));

    let idmat = defaultmat.0;
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));

    let root = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(root, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, root));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    
    let key_curve0 = pi_atom::Atom::from("test");
    let key_curve0 = key_curve0.asset_u64();
    let mut curve = FrameCurve::<LocalEulerAngles>::curve_frame_values(30);
    curve.curve_frame_values_frame(1, LocalEulerAngles(Vector3::new(0., 0., 0.)));
    curve.curve_frame_values_frame(120, LocalEulerAngles(Vector3::new(6.28, 6.28, 0.)));
    let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) {
        curve
    } else {
        match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
            Ok(value) => { value },
            Err(_) => { return; },
        }
    };
    let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
    actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), root, animation));
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE));

    let temproot = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(temproot, root));
    actions.transform.create.push(OpsTransformNode::ops(scene, temproot));
    // let cell_col = 4.;
    // let cell_row = 4.;
    let size = 4;
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                
                let ins: Entity = commands.spawn_empty().id();
                actions.instance.create.push(OpsInstanceMeshCreation::ops(source, ins));
                actions.transform.tree.push(OpsTransformNodeParent::ops(ins, temproot));
                
                let r = (i as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;
                let g = (j as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;
                let b = (k as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;
                let _a: f32 = (k as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;

                let x = (i as f32 - size as f32 / 2.) + 0.5;
                let y = (j as f32 - size as f32 / 2.) + 0.5;
                let z = (k as f32 - size as f32 / 2.) + 0.5;
                actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(ins, x, y, z));
                actions.transform.localscl.push(OpsTransformNodeLocalScaling::ops(ins, r, g, b));
                actions.instance.vec4s.push(OpsInstanceVec4::ops(ins, r, g, b, 1., Atom::from("InsColor4")));
            }
        }
    }
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

    // while !app.ready() {
    //     #[cfg(not(target_arch = "wasm32"))]
    //     bevy::tasks::tick_global_task_pools_on_main_thread();
    // }
    // app.finish();
    // app.cleanup();

    // app.run()
    loop { app.update(); }

}