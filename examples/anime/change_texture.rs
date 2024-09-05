#![feature(box_into_inner)]


use base::DemoScene;
use pi_animation::{loop_mode::ELoopMode, animation_group::AnimationGroupID};
use pi_atom::Atom;
use pi_curves::curve::frame_curve::FrameCurve;
use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

use std::mem::replace;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut matmetas: ResMut<ShareAssetMgr<ShaderEffectMeta>>,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    nodematblocks: Res<NodeMaterialBlocks>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    mut list: ResMut<ActionListTestData>,
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(OpacityClipShader::KEY), OpacityClipShader::create(&nodematblocks));

    let tes_size = 5;
    fps.frame_ms = 50;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let root = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(root, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, root));

    let node = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, node));

    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = base::instance_attr(false, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);
    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_TRANSPARENT, blend));

    actions.transform.tree.push(OpsTransformNodeParent::ops(source, node));
    actions.transform.tree.push(OpsTransformNodeParent::ops(node, root));

    let idmat = commands.spawn_empty_id();
    actions.material.create.push(OpsMaterialCreate::ops(idmat, OpacityClipShader::KEY));
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("assets/images/fractal.png"),
    }));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("assets/images/eff_ui_ll_085.png"),
    }));
    actions.material.val.push(OpsUniformVal::float(
            idmat, 
            Atom::from(BlockCutoff::KEY_VALUE), 
            0.5
        )
    );
    actions.material.val.push(OpsUniformVal::vec4(
            idmat, 
            Atom::from(BlockEmissiveTexture::KEY_INFO), 
            1., 1., 1., 1.
        )
    );
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty_id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    // {
    //     let key_curve0 = pi_atom::Atom::from("cutoff");
    //     let key_curve0 =key_curve0.asset_u64();
    //     let mut curve = FrameCurve::<Cutoff>::curve_frame_values(10000);
    //     curve.curve_frame_values_frame(0, Cutoff(0.));
    //     curve.curve_frame_values_frame(10000, Cutoff(0.5));
        
    //     let asset_curve = if let Some(curve) = anime_assets.alphacutoff.get(&key_curve0) {
    //         curve
    //     } else {
    //         match anime_assets.alphacutoff.insert(key_curve0, TypeFrameCurve(curve)) {
    //             Ok(value) => {
    //                 value
    //             },
    //             Err(_) => {
    //                 return;
    //             },
    //         }
    //     };
    
    //     let animation = anime_contexts.alphacutoff.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
    //     actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group, idmat, animation));
    // }
    {
        let key_curve0 = pi_atom::Atom::from("Pos");
        let key_curve0 = key_curve0.asset_u64();
        let mut curve = FrameCurve::<LocalPosition>::curve_frame_values(10000);
        curve.curve_frame_values_frame(0, LocalPosition(Vector3::new(0., 0., 0.)));
        curve.curve_frame_values_frame(10000, LocalPosition(Vector3::new(2., 0., 0.)));
        
        let asset_curve = if let Some(curve) = anime_assets.position.get(&key_curve0) {
            curve
        } else {
            match anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => {
                    value
                },
                Err(_e) => {
                    return;
                },
            }
        };
    
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group, root, animation));
    }
    let mut parma = AnimationGroupParam::default();
    parma.loop_mode = ELoopMode::Not;
    parma.speed = 1.;
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));

    // animegroupres.global.add_frame_event_listen(id_group);
    // animegroupres.global.add_frame_event(id_group, 0.5, 100);
    actions.anime.action.push(OpsAnimationGroupAction::listen_start(id_group));
    actions.anime.action.push(OpsAnimationGroupAction::listen_end(id_group));

    list.material = Some(idmat);
}

pub fn sys_anime_event(
    mut events: ResMut<GlobalAnimeEvents>,
    mut test: ResMut<ActionListTestData>,
    mut actions: pi_3d::ActionSets,
) {
    let mut list: Vec<(Entity, Entity, u8, u32)> = replace(&mut events, vec![]);
    list.drain(..).for_each(|item| {
        log::warn!("Event {:?}", item);
        if let Some(idmat) = test.material {
            if item.2 == 2 {
                actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
                    slotname: Atom::from(BlockMainTexture::KEY_TEX),
                    filter: true,
                    sample: KeySampler::linear_repeat(),
                    url: EKeyTexture::from("assets/images/eff_ui_ll_085.png"),
                }));
            // } else {
            //     test.change = true;
            }
        }
    });
}

#[derive(Resource)]
pub struct ActionListTestData {
    material: Option<Entity>,
    change: bool,
}

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData { material: None, change: false  });
    }
}

pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();
    
    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 5.,
        camera_position: (0., 0., -10.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    app.add_systems(Update, sys_anime_event.in_set(ERunStageChap::Anime));
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}