#![feature(box_into_inner)]

use base::DemoScene;
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeAssetMgrs, TypeAnimeContexts};
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
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut skincmds: ActionSetSkeleton,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut renderercmds: ActionSetRenderer,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 5;
    fps.frame_ms = 4;

    

    let demopass = base::DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut renderercmds, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01, id_renderer) = (demopass.scene, demopass.camera, demopass.transparent_renderer);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut matcmds, &mut meshcmds, &mut geometrycmd, &mut cameracmds, &mut transformcmds, &mut renderercmds, scene, demopass.transparent_renderer,demopass.transparent_target);
    renderercmds.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = 0;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false, ..Default::default() }));
    transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(source, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    
    let bone0 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone0, scene));
    let bone1 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone1, scene));
    let key_curve0 = pi_atom::Atom::from((1).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone1, id_group.clone(), animation);
    }
    let bone2 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone2, scene));
    let key_curve0 = pi_atom::Atom::from((2).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(-1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone2, id_group.clone(), animation);
    }
    let bone3 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone3, scene));
    let key_curve0 = pi_atom::Atom::from((3).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., 1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone3, id_group.clone(), animation);
    }
    let bone4 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone4, scene));
    let key_curve0 = pi_atom::Atom::from((4).to_string());
    let key_curve0 = key_curve0.asset_u64();
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., -1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone4, id_group.clone(), animation);
    }
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE);

    skincmds.bone_create.push(OpsBoneCreation::ops(bone0, scene, scene));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone1, bone0, scene));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone2, bone0, scene));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone3, bone0, scene));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone4, bone0, scene));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone0, scene));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone1, bone0));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone2, bone0));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone3, bone0));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone4, bone0));

    let data: [u16; 48] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1, 
        2, 2, 2, 2, 2, 2, 2, 2, 
        3, 3, 3, 3, 3, 3, 3, 3, 
        4, 4, 4, 4, 4, 4, 4, 4
    ];
    // normals
    let jointkey = KeyVertexBuffer::from("TestJoint");
    geometrycmd.vb_wait.add(&jointkey, bytemuck::cast_slice(&data).iter().map(|v| *v).collect::<Vec<u8>>());

    let format = wgpu::VertexFormat::Uint16x2;
    let jointdesc = VertexBufferDesc::vertices(jointkey.clone(), VertexBufferDescRange::default(), vec![VertexAttribute { kind: EVertexDataKind::MatricesIndices1, format }]);
    
    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(jointdesc);
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from("_MainTex"),
        filter: true,
        sample: KeySampler::default(),
        url: EKeyTexture::from("assets/images/bubbles.png"),
    }));

    let skeleton = commands.spawn_empty().id();
    skincmds.skin_create.push(OpsSkinCreation::ops(skeleton, ESkinBonesPerVertex::One, bone0, &vec![bone0, bone1, bone2, bone3, bone4], 1, None));
    skincmds.skin_use.push(OpsSkinUse::ops(source, skeleton));

    transformcmds.localrot.push(OpsTransformNodeLocalEuler::ops(source, 1. as f32 * 0.2, 1. as f32 * 0.2, 1. as f32 * 0.2));
    meshcmds.cullmode.push(OpsCullMode::ops(source, CullMode::Off));
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
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;
    
    // app.run()
    loop { app.update(); }

}