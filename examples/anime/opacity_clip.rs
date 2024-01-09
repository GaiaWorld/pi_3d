#![feature(box_into_inner)]


use base::DemoScene;
use pi_animation::loop_mode::ELoopMode;
use pi_atom::Atom;
use pi_curves::curve::frame_curve::FrameCurve;
use pi_engine_shell::prelude::*;
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
) {
    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(OpacityClipShader::KEY), OpacityClipShader::create(&nodematblocks));

    let tes_size = 5;
    fps.frame_ms = 50;


    let demopass = DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let root = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(root, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, root));

    let node = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, node));

    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = base::instance_attr(false, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);
    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.blend.push(OpsRenderBlend::ops(source, blend));

    // actions.transform.enable.push(OpsNodeEnable::ops(source, false));

    actions.transform.tree.push(OpsTransformNodeParent::ops(source, node));
    actions.transform.tree.push(OpsTransformNodeParent::ops(node, root));

    let idmat = commands.spawn_empty().id();
    actions.material.create.push(OpsMaterialCreate::ops(idmat, OpacityClipShader::KEY));
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/fractal.png"),
    }));
    actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_ui_ll_085.png"),
    }));
    actions.material.float.push(
        OpsUniformFloat::ops(
            idmat, 
            Atom::from(BlockCutoff::KEY_VALUE), 
            0.8
        )
    );
    actions.material.vec3.push(
        OpsUniformVec3::ops(
            idmat, 
            Atom::from(BlockEmissiveTexture::KEY_INFO), 
            1., 1., 1.,
        )
    );
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    {
        let key_curve0 = pi_atom::Atom::from("cutoff");
        let key_curve0 =key_curve0.asset_u64();
        let mut curve = FrameCurve::<AnimatorableFloat>::curve_frame_values(10000);
        curve.curve_frame_values_frame(0, AnimatorableFloat(0.));
        curve.curve_frame_values_frame(10000, AnimatorableFloat(1.));
        
        let asset_curve = if let Some(curve) = anime_assets.float.get(&key_curve0) {
            curve
        } else {
            match anime_assets.float.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value },
                Err(_) => { return; },
            }
        };
    
        let animation = anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        // actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group, idmat, animation));
        actions.anime_uniform.push(OpsTargetAnimationUniform::ops( idmat, Atom::from(BlockCutoff::KEY_VALUE), id_group.clone(), key_curve0));
    }
    // {
    //     let key_curve0 = pi_atom::Atom::from("tilloff");
    //     let key_curve0 =key_curve0.asset_u64();
    //     let mut curve = FrameCurve::<AnimatorableVec4>::curve_frame_values(10000);
    //     curve.curve_frame_values_frame(0, AnimatorableVec4::from([1., 1., 0., 0.].as_slice()));
    //     curve.curve_frame_values_frame(10000, AnimatorableVec4::from([1., 1., 1., 1.].as_slice()));
        
    //     let asset_curve = if let Some(curve) = anime_assets.vec4s.get(&key_curve0) {
    //         curve
    //     } else {
    //         match anime_assets.vec4s.insert(key_curve0, TypeFrameCurve(curve)) {
    //             Ok(value) => { value },
    //             Err(_) => { return; },
    //         }
    //     };
    
    //     let animation = anime_contexts.vec4s.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
    //     // actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group, idmat, animation));
    //     actions.anime_uniform.push(OpsTargetAnimationUniform::ops( idmat, Atom::from(BlockMainTexture::KEY_TILLOFF), id_group.clone(), key_curve0));
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
                Ok(value) => { value },
                Err(_e) => { return; },
            }
        };
    
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group, root, animation));
    }
    let mut parma = AnimationGroupParam::default();
    parma.loop_mode = ELoopMode::Not;
    parma.speed = 0.1;
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));

    // animegroupres.global.add_frame_event_listen(id_group);
    // animegroupres.global.add_frame_event(id_group, 0.5, 100);
    actions.anime.listens.push(OpsAddAnimationListen::Start(id_group));
    actions.anime.listens.push(OpsAddAnimationListen::End(id_group));
}

pub fn sys_anime_event(
    mut events: ResMut<GlobalAnimeEvents>,
) {
    let mut list: Vec<(Entity, Entity, u8, u32)> = replace(&mut events, vec![]);
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

pub fn main() {
    let mut app = base::test_plugins();
    app.add_plugins(PluginTest);
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    app.add_systems(Update, sys_anime_event.in_set(ERunStageChap::Anime));
    
    // app.run()
    loop { app.update(); }

}