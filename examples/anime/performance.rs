#![feature(box_into_inner)]



use base::DemoScene;
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

use std::{ mem::replace, ops::DerefMut};

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    let tes_size = 30;
    fps.frame_ms = 4;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = CubeBuilder::attrs_meta();
    let indices = CubeBuilder::indices_meta();
    let id_group = commands.spawn_empty_id();
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));

    let cell_col = 4.;
    let cell_row = 4.;
    for i in 0..tes_size {
        for j in 0..tes_size {
            for k in 0..1 {
                let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices.clone(), indices.clone(), base::instance_attr(false, false, false));
                let idmat = commands.spawn_empty_id();
                actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
                actions.material.create.push(OpsMaterialCreate::ops_with_matarray(idmat, UnlitShader::KEY));
                actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
                    slotname: Atom::from(BlockMainTexture::KEY_TEX),
                    sample: KeySampler::default(),
                    url: EKeyTexture::from("assets/images/bubbles.png"),
                    ..Default::default()
                }));
                actions.material.val.push(OpsUniformVal::vec4(
                        idmat, 
                        Atom::from(BlockEmissiveTexture::KEY_INFO), 
                        1., 0., 0., 1.
                    )
                );

                let cube: Entity = source;
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation(i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, 0.)));
                actions.instance.attr.push(OpsInstanceAttr::ops(cube, EInstanceAttr::Vec4([1.0 / cell_col, 1.0 / cell_row, (i % 4) as f32 / cell_col, (j % 4) as f32 / cell_row]), Atom::from("InsTilloff")));
                let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                let key_curve0 = key_curve0.asset_u64();
                let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(i as f32, j as f32, k as f32)), LocalEulerAngles(Vector3::new(10., 10., 10.)), 30, 30, EEasingMode::None);

                let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) {
                    curve
                } else {
                    match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                        Ok(value) => { value },
                        Err(_) => { break; },
                    }
                };

                let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group, cube, animation));
                // engine.create_target_animation(source, cube, &key_group, animation);
            }
        }
    }

    let parma = AnimationGroupParam::default();
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());

}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}

pub fn sys_anime_event(
    mut events: ResMut<GlobalAnimeEvents>,
) {
    let mut list: Vec<(Entity, Entity, u8, u32)> = replace(events.deref_mut(), vec![]);
    list.drain(..).for_each(|item| {
        log::warn!("Event {:?}", item);
    });
}


pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 100.,
        camera_position: (0., 0., -10.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    
        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    

    app.add_systems(StageD3, sys_anime_event.in_set(ERunStageChap::Modify));
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}