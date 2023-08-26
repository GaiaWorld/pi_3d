#![feature(box_into_inner)]



use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::SingleFrameTimeCommand};

use pi_gltf2_load::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;

fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matuse: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    mut renderercmds: ActionSetRenderer,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
) {
    let tes_size = 20;
    fps.frame_ms = 30;
    final_render.cleardepth = 0.0;

    let (scene, camera01) = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, 4., 0.7, (0., 0., -10.), false);
    // cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));
    // cameracmds.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    let root = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(root, scene));
    transformcmds.create.push(OpsTransformNode::ops(scene, root));
    // transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(root, 0., 0., 0.));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, root));


    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = 0;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
    // meshcmds.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::VerticalBillboard));
    // meshcmds.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));
    
    let id_geo = commands.spawn_empty().id();
    let attrs = CubeBuilder::attrs_meta();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = defaultmat.0;
    matuse.usemat.push(OpsMaterialUse::ops(source, idmat));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(root, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, root, id_group));

    {
        let key_curve0 = pi_atom::Atom::from("test"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(1., 1., 1.)), LocalScaling(Vector3::new(0., 4., 2.)), (60.) as FrameIndex, 30, EEasingMode::None);
        
        let asset_curve = if let Some(curve) = anime_assets.scaling.get(&key_curve0) {
            curve
        } else {
            match anime_assets.scaling.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => {
                    value
                },
                Err(_) => {
                    return;
                },
            }
        };

        let animation = anime_contexts.scaling.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, source, id_group.clone(), animation);
    }
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
        animegroupcmd.scene_ctxs.add_target_anime(scene, root, id_group.clone(), animation);
    }

    // animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE);
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
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
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}