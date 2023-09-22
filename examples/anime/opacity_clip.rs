#![feature(box_into_inner)]


use base::DemoScene;
use pi_animation::{loop_mode::ELoopMode, animation_group::AnimationGroupID};
use pi_atom::Atom;
use pi_curves::curve::frame_curve::FrameCurve;
use pi_engine_shell::prelude::*;
use pi_gltf2_load::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

use std::mem::replace;

fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    nodematblocks: Res<NodeMaterialBlocks>,
    mut renderercmds: ActionSetRenderer,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
) {
    ActionMaterial::regist_material_meta(&matcmds.metas, KeyShaderMeta::from(OpacityClipShader::KEY), OpacityClipShader::create(&nodematblocks));

    let tes_size = 5;
    fps.frame_ms = 50;


    let (scene, camera01) = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 0., -10.), true);
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let root = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(root, scene));
    transformcmds.create.push(OpsTransformNode::ops(scene, root));

    let node = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(node, scene));
    transformcmds.create.push(OpsTransformNode::ops(scene, node));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = 0;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
    let mut blend = ModelBlend::default(); blend.combine();
    meshcmds.blend.push(OpsRenderBlend::ops(source, blend));

    transformcmds.tree.push(OpsTransformNodeParent::ops(source, node));
    transformcmds.tree.push(OpsTransformNodeParent::ops(node, root));
    
    let id_geo = commands.spawn_empty().id();
    let attrs = CubeBuilder::attrs_meta();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    matcmds.create.push(OpsMaterialCreate::ops(idmat, OpacityClipShader::KEY, EPassTag::Transparent));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/fractal.png"),
    }));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_ui_ll_085.png"),
    }));
    matcmds.float.push(
        OpsUniformFloat::ops(
            idmat, 
            Atom::from(BlockCutoff::KEY_VALUE), 
            0.5
        )
    );
    matcmds.vec4.push(
        OpsUniformVec4::ops(
            idmat, 
            Atom::from(BlockEmissiveTexture::KEY_INFO), 
            1., 1., 1., 1.
        )
    );
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(source, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    {
        let key_curve0 = pi_atom::Atom::from("cutoff");
        let key_curve0 =key_curve0.asset_u64();
        let mut curve = FrameCurve::<Cutoff>::curve_frame_values(10000);
        curve.curve_frame_values_frame(0, Cutoff(0.));
        curve.curve_frame_values_frame(10000, Cutoff(1.));
        
        let asset_curve = if let Some(curve) = anime_assets.alphacutoff.get(&key_curve0) {
            curve
        } else {
            match anime_assets.alphacutoff.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => {
                    value
                },
                Err(_) => {
                    return;
                },
            }
        };
    
        let animation = anime_contexts.alphacutoff.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, idmat, id_group, animation);
    }
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
        animegroupcmd.scene_ctxs.add_target_anime(scene, root, id_group, animation);
    }
    let mut parma = AnimationGroupParam::default();
    parma.loop_mode = ELoopMode::Not;
    parma.speed = 0.1;
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), parma, 0., pi_animation::base::EFillMode::NONE);

    // animegroupcmd.global.add_frame_event_listen(id_group);
    // animegroupcmd.global.add_frame_event(id_group, 0.5, 100);
    animegroupcmd.global.add_start_listen(id_group);
    animegroupcmd.global.add_end_listen(id_group);
}

pub fn sys_anime_event(
    mut events: ResMut<GlobalAnimeEvents>,
) {
    let mut list: Vec<(Entity, AnimationGroupID, u8, u32)> = replace(&mut events, vec![]);
    list.drain(..).for_each(|item| {
        log::warn!("Event {:?}", item);
    });
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
    app.add_systems(Update, sys_anime_event.in_set(ERunStageChap::Anime));
    
    // app.run()
    loop { app.update(); }

}