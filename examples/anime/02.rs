#![feature(box_into_inner)]



use base::DemoScene;
use pi_curves::curve::frame_curve::FrameCurve;
use pi_engine_shell::{prelude::*, frame_time::SingleFrameTimeCommand};

use pi_gltf2_load::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;


fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut instancemeshcmds: ActionSetInstanceMesh,
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
    fps.frame_ms = 16;

    let (scene, camera01) = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, 10., 0.7, (0., 10., -40.), false);
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    meshcmds.create.push(OpsMeshCreation::ops(scene, source));
    meshcmds.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));
    
    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(VertexBufferDesc::instance_world_matrix());
    attrs.push(VertexBufferDesc::instance_color());
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = defaultmat.0;
    matuse.usemat.push(OpsMaterialUse::ops(source, idmat));

    let root = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(root, scene));
    transformcmds.create.push(OpsTransformNode::ops(scene, root));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(source, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    
    let key_curve0 = pi_atom::Atom::from("test");
    let key_curve0 = key_curve0.asset_u64();
    let mut curve = FrameCurve::<LocalEulerAngles>::curve_frame_values(30);
    curve.curve_frame_values_frame(1, LocalEulerAngles(Vector3::new(0., 0., 0.)));
    curve.curve_frame_values_frame(120, LocalEulerAngles(Vector3::new(6.28, 6.28, 0.)));
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
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE);

    let temproot = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(temproot, root));
    transformcmds.create.push(OpsTransformNode::ops(scene, temproot));
    // let cell_col = 4.;
    // let cell_row = 4.;
    let size = 4;
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                
                let ins: Entity = commands.spawn_empty().id();
                instancemeshcmds.create.push(OpsInstanceMeshCreation::ops(source, ins));
                transformcmds.tree.push(OpsTransformNodeParent::ops(ins, temproot));
                
                let r = (i as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;
                let g = (j as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;
                let b = (k as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;
                let _a: f32 = (k as f32 - size as f32 / 2.).cos().cos() * 0.2 + 0.4;

                let x = (i as f32 - size as f32 / 2.) + 0.5;
                let y = (j as f32 - size as f32 / 2.) + 0.5;
                let z = (k as f32 - size as f32 / 2.) + 0.5;
                transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(ins, x, y, z));
                transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(ins, r, g, b));
                instancemeshcmds.color.push(OpsInstanceColor::ops(ins, r, g, b));
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


#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);

    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}