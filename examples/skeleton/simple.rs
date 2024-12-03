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
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    let tes_size = 5;
    fps.frame_ms = 4;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let source = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState::default()));
    actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty_id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    
    let node0 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node0, scene));
    let node1 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node1, node0));
    let key_curve0 = pi_atom::Atom::from((1).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node1, animation));
    }
    let node2 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node2, node0));
    let key_curve0 = pi_atom::Atom::from((2).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(-1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node2, animation));
    }
    let node3 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node3, node0));
    let key_curve0 = pi_atom::Atom::from((3).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., 1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node3, animation));
    }
    let node4 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node4, node0));
    let key_curve0 = pi_atom::Atom::from((4).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., -1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node4, animation));
    }
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE));
    
    actions.transform.create.push(OpsTransformNode::ops(scene, node0));
    actions.transform.create.push(OpsTransformNode::ops(scene, node1));
    actions.transform.create.push(OpsTransformNode::ops(scene, node2));
    actions.transform.create.push(OpsTransformNode::ops(scene, node3));
    actions.transform.create.push(OpsTransformNode::ops(scene, node4));

    let bone0 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone0, scene));
    let bone1 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone1, bone0));
    let bone2 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone2, bone0));
    let bone3 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone3, bone0));
    let bone4 = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(bone4, bone0));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone0, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone1, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone2, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone3, scene));
    actions.skin.bone_create.push(OpsBoneCreation::ops(bone4, scene));
    actions.skin.skin_use.push(OpsSkinUse::bone_link(bone0, node0));
    actions.skin.skin_use.push(OpsSkinUse::bone_link(bone1, node1));
    actions.skin.skin_use.push(OpsSkinUse::bone_link(bone2, node2));
    actions.skin.skin_use.push(OpsSkinUse::bone_link(bone3, node3));
    actions.skin.skin_use.push(OpsSkinUse::bone_link(bone4, node4));

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
    
    let id_geo = commands.spawn_empty_id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(jointdesc);
    actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty_id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from("_MainTex"),
        sample: KeySampler::default(),
        url: EKeyTexture::from("assets/images/bubbles.png"),
        ..Default::default()
    }));

    let skeleton = commands.spawn_empty_id();
    actions.skin.skin_create.push(OpsSkinCreation::ops(skeleton, ESkinBonesPerVertex::One, bone0, &vec![bone0, bone1, bone2, bone3, bone4], 1, None));
    actions.skin.skin_use.push(OpsSkinUse::ops(source, skeleton));

    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Euler(1. as f32 * 0.2, 1. as f32 * 0.2, 1. as f32 * 0.2)));
    actions.mesh.render_state.push(OpsRenderState::primitive_state(source, DemoScene::PASS_TRANSPARENT, EPrimitiveState::CCullMode( CullMode::Off )) );
}
pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}


pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins_with_gltf();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 6.,
        camera_fov: 0.7,
        camera_position: (0., 0., -10.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    
    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}