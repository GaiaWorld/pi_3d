#![feature(box_into_inner)]

use base::DemoScene;
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
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
    mut geometryres: ResourceGeometry,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 5;
    fps.frame_ms = 4;

    

    let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01, id_renderer) = (demopass.scene, demopass.camera, demopass.transparent_renderer);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let source = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState::default()));
    actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    
    let bone0 = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone0, scene));
    let bone1 = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone1, scene));
    let key_curve0 = pi_atom::Atom::from((1).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), bone1, animation));
    }
    let bone2 = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone2, scene));
    let key_curve0 = pi_atom::Atom::from((2).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(-1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), bone2, animation));
    }
    let bone3 = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone3, scene));
    let key_curve0 = pi_atom::Atom::from((3).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., 1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), bone3, animation));
    }
    let bone4 = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone4, scene));
    let key_curve0 = pi_atom::Atom::from((4).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., -1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), bone4, animation));
    }
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE));

    actions.skin.bone_create.push(OpsBoneCreation::ops(bone0, scene, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone1, bone0, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone2, bone0, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone3, bone0, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone4, bone0, scene));
    // actions.transform.tree.push(OpsTransformNodeParent::ops(bone0, scene));
    // actions.transform.tree.push(OpsTransformNodeParent::ops(bone1, bone0));
    // actions.transform.tree.push(OpsTransformNodeParent::ops(bone2, bone0));
    // actions.transform.tree.push(OpsTransformNodeParent::ops(bone3, bone0));
    // actions.transform.tree.push(OpsTransformNodeParent::ops(bone4, bone0));

    let data: [u32; 24] = [
        0, 0, 0, 0, 
        0, 0, 0, 0, 
        1, 1, 1, 1, 
        2, 2, 2, 2, 
        3, 3, 3, 3, 
        4, 4, 4, 4, 
    ];
    // normals
    let jointkey = KeyVertexBuffer::from("TestJoint");
    geometryres.vb_wait.add(&jointkey, bytemuck::cast_slice(&data).iter().map(|v| *v).collect::<Vec<u8>>());

    let format = wgpu::VertexFormat::Uint32;
    let jointdesc = VertexBufferDesc::vertices(
        jointkey.clone(), VertexBufferDescRange::default(), 
        vec![ EVertexAttribute::Custom(CustomVertexAttribute::new(Atom::from("A_JOINT_INC1"), Atom::from(""), ECustomVertexType::Uint, None)) ]
    );
    
    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(jointdesc);
    actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY));
    actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from("_MainTex"),
        filter: true,
        sample: KeySampler::default(),
        url: EKeyTexture::from("assets/images/bubbles.png"),
    }));

    let skeleton = commands.spawn_empty().id();
    actions.skin.skin_create.push(OpsSkinCreation::ops(skeleton, ESkinBonesPerVertex::One, bone0, &vec![bone0, bone1, bone2, bone3, bone4], 1, None));
    actions.skin.skin_use.push(OpsSkinUse::ops(source, skeleton));

    actions.transform.localrot.push(OpsTransformNodeLocalEuler::ops(source, 1. as f32 * 0.2, 1. as f32 * 0.2, 1. as f32 * 0.2));
    actions.mesh.cullmode.push(OpsCullMode::ops(source, CullMode::Off));
}
pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}


pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;
    
    // app.run()
    loop { app.update(); }

}