#![feature(box_into_inner)]


use base::DemoScene;
use pi_animation::loop_mode::ELoopMode;
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

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
    mut renderercmds: ActionSetRenderer,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
) {
    ActionMaterial::regist_material_meta(&matcmds.metas, KeyShaderMeta::from(MainOpacityShader::KEY), MainOpacityShader::meta());

    let tes_size = 5;
    fps.frame_ms = 4;

    let (scene, camera01) = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 0., -10.), true);
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = 0;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
    let mut blend = ModelBlend::default(); blend.combine();
    meshcmds.blend.push(OpsRenderBlend::ops(source, blend));
    
    let id_geo = commands.spawn_empty().id();
    let attrs = CubeBuilder::attrs_meta();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    matcmds.create.push(OpsMaterialCreate::ops(idmat, MainOpacityShader::KEY, EPassTag::Transparent));
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
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/icon_city.png"),
    }));
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
        let key_curve0 = pi_atom::Atom::from("color");
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<MainColor>::curve_easing(MainColor(Vector3::new(0.5, 0.5, 0.5)), MainColor(Vector3::new(1.0, 1., 1.)), 30, 30, EEasingMode::None);
        let asset_curve = match anime_assets.maincolor_curves.insert(key_curve0, TypeFrameCurve(curve)) {
            Ok(value) => { value },
            Err(_) => { return; },
        };
        let animation = anime_contexts.maincolor.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, idmat, id_group.clone(), animation);
    }
    {
        let key_curve0 = pi_atom::Atom::from("mainuo");
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<OpacityTexUOffset>::curve_easing(OpacityTexUOffset(0.), OpacityTexUOffset(1.0), 30, 30, EEasingMode::None);
        let asset_curve = match anime_assets.opacityuoff_curves.insert(key_curve0, TypeFrameCurve(curve)) {
            Ok(value) => { value },
            Err(_) => { return; },
        };
        let animation = anime_contexts.opacitytex_uoffset.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, idmat, id_group.clone(), animation);
    }
    let mut parma = AnimationGroupParam::default();
    parma.loop_mode = ELoopMode::Positive(Some(5));
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), parma, 0., pi_animation::base::EFillMode::NONE);
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
    
    // app.run()
    loop { app.update(); }

}