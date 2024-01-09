#![feature(box_into_inner)]


use base::DemoScene;
use pi_animation::loop_mode::ELoopMode;
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

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
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(MainOpacityShader::KEY), MainOpacityShader::meta());

    let tes_size = 5;
    fps.frame_ms = 4;

    let demopass = DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = base::instance_attr(false, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.blend.push(OpsRenderBlend::ops(source, blend));

    let idmat = commands.spawn_empty().id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, MainOpacityShader::KEY));
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
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/icon_city.png"),
    }));
    actions.material.vec3.push(
        OpsUniformVec3::ops(
            idmat, 
            Atom::from(BlockMainTexture::KEY_COLOR), 
            1., 1., 0.,
        )
    );
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    {
        let key_curve0 = pi_atom::Atom::from("color");
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<AnimatorableVec3>::curve_easing(AnimatorableVec3(Vector3::new(0.5, 0.5, 0.5)), AnimatorableVec3(Vector3::new(1.0, 1., 1.)), 30, 30, EEasingMode::None);
        let asset_curve = match anime_assets.vec3s.insert(key_curve0, TypeFrameCurve(curve)) {
            Ok(value) => { value },
            Err(_) => { return; },
        };
        let animation = anime_contexts.vec3s.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        // actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), idmat, animation));
        actions.anime_uniform.push(OpsTargetAnimationUniform::ops( idmat, Atom::from(BlockMainTexture::KEY_COLOR), id_group.clone(), key_curve0));
    }
    // {
    //     let key_curve0 = pi_atom::Atom::from("mainuo");
    //     let key_curve0 = key_curve0.asset_u64();
    //     let curve = FrameCurve::<AnimatorableFloat>::curve_easing(AnimatorableFloat(0.), AnimatorableFloat(1.0), 30, 30, EEasingMode::None);
    //     let asset_curve = match anime_assets.float.insert(key_curve0, TypeFrameCurve(curve)) {
    //         Ok(value) => { value },
    //         Err(_) => { return; },
    //     };
    //     let animation = anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
    //     // actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), idmat, animation));
    //     actions.anime_uniform.push(OpsTargetAnimationUniform::ops(scene, idmat, Atom::from(BlockEmissiveTexture::KEY_INFO), id_group.clone(), animation));
    // }
    let mut parma = AnimationGroupParam::default();
    parma.loop_mode = ELoopMode::Positive(Some(5));
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
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