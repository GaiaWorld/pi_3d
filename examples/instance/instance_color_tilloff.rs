#![feature(box_into_inner)]


use base::DemoScene;
use pi_atom::Atom;
use pi_curves::curve::curves::FrameCurve;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_mesh_builder::cube::*;
use rand::random;
use unlit_material::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn anim(
    mut actions: pi_3d::ActionSets,
    mut test: ResMut<ActionListTestData>,
) {
    if let Some(test) = test.as_mut() {
        let span = test.0;
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(span, ETransformSRT::Euler(0.2, test.1, 0.)));
        test.1 += 0.05;
    }
}

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut matmetas: ResMut<ShareAssetMgr<ShaderEffectMeta>>,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    demooption: Res<base::DemoOption>,
    engineopt: Res<EngineCustomPlugins>,
    
    mut test: ResMut<ActionListTestData>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(MainOpacityShader::KEY), MainOpacityShader::meta(&engineopt));

    let tes_size = 15;
    fps.frame_ms = 4;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = CubeBuilder::attrs_meta();
    let indices = CubeBuilder::indices_meta();
    let mut state = MeshInstanceState::default();
    state.instance_matrix = false;
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsPosition"),
            Atom::from("A_POSITION.xyz = A_POSITION.xyz * 0.75 + InsPosition + 10. * cos(InsColor.xyz + vec3(PI_Time.y, PI_Time.y, PI_Time.y));"),
            ECustomVertexType::Vec3, None
        )
    );
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsColor"),
            Atom::from("A_COLOR4 = InsColor * A_COLOR4;"),
            ECustomVertexType::Vec4, None
        )
    );
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let mut blend = ModelBlend::default(); //  blend.combine();
    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_OPAQUE, blend));

    let idmat = commands.spawn_empty_id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, DefaultShader::KEY, false));
    // actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from(BlockMainTexture::KEY_TEX),
    //     sample: KeySampler::linear_repeat(),
    //     url: EKeyTexture::from("./assets/images/fractal.png"),
    //     ..Default::default()
    // }));
    // actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
    //     sample: KeySampler::linear_repeat(),
    //     url: EKeyTexture::from("./assets/images/icon_city.png"),
    //     ..Default::default()
    // }));
    // actions.material.val.push(OpsUniformVal::vec4(
    //         idmat, 
    //         Atom::from(BlockEmissiveTexture::KEY_INFO), 
    //         1., 1., 1., 1.
    //     )
    // );
    
    let span = commands.spawn_empty_id();
    actions.transform.tree.push(OpsTransformNodeParent::ops(span, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, span));
    // actions.transform.localsrt.push(OpsTransformNodeLocal::ops(span, ETransformSRT::Scaling(0.2, 0.2, 0.2)));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(span, ETransformSRT::Translation(0., 0., 20.)));
    actions.transform.tree.push(OpsTransformNodeParent::ops(source, span));


    for i in -10..10 {
    for j in -10..10 {
    for k in -10..10 {
        let sprite = commands.spawn_empty_id();
        actions.transform.tree.push(OpsTransformNodeParent::ops(sprite, scene));
        // actions.transform.localsrt.push(OpsTransformNodeLocal::ops(sprite, ETransformSRT::Translation(i as f32, j as f32, k as f32)));
        actions.instance.create.push(OpsInstanceMeshCreation::ops(source, sprite));
        actions.instance.attr.push(OpsInstanceAttr::ops(sprite, EInstanceAttr::Vec4([random(), random(), 0., 1.]), Atom::from("InsColor")));
        actions.instance.attr.push(OpsInstanceAttr::ops(sprite, EInstanceAttr::Vec3([i as f32, j as f32, k as f32]), Atom::from("InsPosition")));
    }
    }
    }
   
    *test = Some((span, 0., 0., 0.));
}

pub type ActionListTestData = Option<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}


pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 5.,
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
    
    app.add_system(Update, anim);
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}