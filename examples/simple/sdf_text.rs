#![feature(box_into_inner)]


use base::DemoScene;
use pi_animation::loop_mode::ELoopMode;
use pi_atom::Atom;
use pi_bevy_render_plugin::ShareFontSheet;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_render::font::Font;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, quad::QuadBuilder};
use unlit_material::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;
#[path ="../sdf_material.rs"]
mod sdf_material;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut matmetas: ResMut<ShareAssetMgr<ShaderEffectMeta>>,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    demooption: Res<base::DemoOption>,
    engineopt: Res<EngineCustomPlugins>,
    mut sheet: ResMut<ShareFontSheet>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(MainOpacityShader::KEY), MainOpacityShader::meta(&engineopt));

    let tes_size = 4;
    fps.frame_ms = 4;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = QuadBuilder::attrs_meta();
    let indices = None;
    let mut state = MeshInstanceState::default();
    state.instance_matrix = true;
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsTilloff"),
            Atom::from("A_UV.y = 1. - A_UV.y;A_UV = A_UV.xy * InsTilloff.xy + InsTilloff.zw;\n"),
            ECustomVertexType::Vec4, None
        )
    );
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsColor"),
            Atom::from("A_COLOR4 = A_COLOR4 * InsColor;\n"),
            ECustomVertexType::Vec4, None
        )
    );
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsPosition"),
            Atom::from("A_POSITION.xy = (A_POSITION.xy + 0.5) * InsPosition.xy + InsPosition.zw;\n"),
            ECustomVertexType::Vec4, None
        )
    );
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsFontParam"),
            Atom::from("vFontParam = InsFontParam;\n"),
            ECustomVertexType::Vec4, None
        )
    );
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_TRANSPARENT, blend));
    actions.mesh.render_state.push(OpsRenderState::primitive_state(source, DemoScene::PASS_TRANSPARENT, EPrimitiveState::CCullMode(CullMode::Off)));

    let idmat = commands.spawn_empty_id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.create.push(OpsMaterialCreate::ops_with_matarray(idmat, sdf_material::ShaderSDFFont::KEY));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::Tex(Atom::from("_$text_sdf")),
        ..Default::default()
    }));
    actions.material.val.push(OpsUniformVal::vec3(
            idmat, 
            Atom::from(BlockMainTexture::KEY_COLOR), 
            1., 1., 1.,
        )
    );
    // actions.material.val.push(OpsUniformVal::vec4(
    //         idmat, 
    //         Atom::from(BlockMainTexture::KEY_TILLOFF), 
    //         0.2, 0.2, 0., 0.
    //     )
    // );

    let mut idx = -2;
    let mut scaleoffset = [0., 0., 0., 0.];
    let mut uvtilloff = [0., 0., 0., 0.];
    let mut fsheet =  sheet.borrow_mut();
    let f = fsheet.font_id(Font::new(Atom::from("hwkt"), 20, 400));

    let span = commands.spawn_empty_id();
    actions.transform.tree.push(OpsTransformNodeParent::ops(span, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, span));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(span, ETransformSRT::Scaling(0.2, 0.2, 0.2)));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(span, ETransformSRT::Translation(-2., 0., 0.)));

    let mut off = 0.;
    "[点击添加描述信息Aa,012".chars().for_each(|char|{
        let id = fsheet.glyph_id(f, char);
        fsheet.measure_width(f, char);

        let sprite = commands.spawn_empty_id();
        actions.transform.tree.push(OpsTransformNodeParent::ops(sprite, span));
        actions.instance.create.push(OpsInstanceMeshCreation::ops(source, sprite));

        let fontsize: f32 = 32.;
        ShareFontSheet::char_calc(&mut fsheet, f, char, &mut scaleoffset, &mut uvtilloff, 40., fontsize, 1., 32., 32., 1.);

        log::error!("{:?}", (scaleoffset));
        let xs = 2. * scaleoffset[0]/fontsize;
        let ys = 2. * scaleoffset[1]/fontsize;
        let xo = scaleoffset[2]/fontsize;
        let yo = 1.-scaleoffset[3]/fontsize-ys;
        off += xo;
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(sprite, ETransformSRT::Translation(off, 0., 0.)));
        off += xs;
        actions.instance.attr.push(OpsInstanceAttr::ops(sprite, EInstanceAttr::Vec4([xs, ys, xo, yo]), Atom::from("InsPosition")));
        actions.instance.attr.push(OpsInstanceAttr::ops(sprite, EInstanceAttr::Vec4([1.,1.,1.,1.]), Atom::from("InsColor")));
        actions.instance.attr.push(OpsInstanceAttr::ops(sprite, EInstanceAttr::Vec4([uvtilloff[0], uvtilloff[1], uvtilloff[2], uvtilloff[3]]), Atom::from("InsTilloff")));
        actions.instance.attr.push(OpsInstanceAttr::ops(sprite, EInstanceAttr::Vec4([0.50, 0.52, 0., 0.]), Atom::from("InsFontParam")));
        idx += 1;
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
    let (mut app, window, event_loop) = base::test_plugins();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_position: (0., 0., -40.),
        ..Default::default()
    });
    app.add_plugins(sdf_material::PluginShaderSDFFont);
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);

    
    // app.add_systems(StageD3, pi_3d::sys_info_node);
    // app.add_systems(StageD3, pi_3d::sys_info_resource);
    // app.add_systems(StageD3, pi_3d::sys_info_draw);
        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}