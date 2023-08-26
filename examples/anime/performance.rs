#![feature(box_into_inner)]



use base::DemoScene;
use pi_animation::animation_group::AnimationGroupID;
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

use std::{ mem::replace, ops::DerefMut};


fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut instancemeshcmds: ActionSetInstanceMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    mut renderercmds: ActionSetRenderer,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
) {
    let tes_size = 100;
    fps.frame_ms = 4;

    let (scene, camera01) = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 0., -10.), true);
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let source = commands.spawn_empty().id();
    let instancestate = InstanceState::INSTANCE_BASE | InstanceState::INSTANCE_TILL_OFF_1;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
    
    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Opaque));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::default(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    }));
    matcmds.vec4.push(
        OpsUniformVec4::ops(
            idmat, 
            Atom::from(BlockEmissiveTexture::KEY_INFO), 
            1., 0., 0., 1.
        )
    );
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(source, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    let cell_col = 4.;
    let cell_row = 4.;
    for i in 0..tes_size {
        for j in 0..tes_size {
            for k in 0..1 {
                
                let cube: Entity = commands.spawn_empty().id();
                instancemeshcmds.create.push(OpsInstanceMeshCreation::ops(source, cube));

                transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cube, i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, k as f32 * 2. - (tes_size) as f32));

                instancemeshcmds.tilloff.push(OpsInstanceTillOff::ops(cube, 1.0 / cell_col, 1.0 / cell_row, (i % 4) as f32 / cell_col, (j % 4) as f32 / cell_row));
                
                let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                let key_curve0 = key_curve0.asset_u64();
                let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(i as f32, j as f32, k as f32)), LocalEulerAngles(Vector3::new(10., 10., 10.)), 30, 30, EEasingMode::None);
                
                let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) {
                    curve
                } else {
                    match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                        Ok(value) => {
                            value
                        },
                        Err(_) => {
                            break;
                        },
                    }
                };

                let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                animegroupcmd.scene_ctxs.add_target_anime(scene, cube, id_group, animation);
                // engine.create_target_animation(source, cube, &key_group, animation);
            }
        }
    }

    let parma = AnimationGroupParam::default();
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), parma, 0., pi_animation::base::EFillMode::NONE);
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
    let mut list: Vec<(Entity, AnimationGroupID, u8, u32)> = replace(events.deref_mut(), vec![]);
    list.drain(..).for_each(|item| {
        log::warn!("Event {:?}", item);
    });
}


#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);

    app.add_systems(Update, sys_anime_event.in_set(ERunStageChap::Anime));
    
    app.run()

}