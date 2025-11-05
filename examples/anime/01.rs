#![feature(box_into_inner)]


use pi_animation::loop_mode::ELoopMode;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::{prelude::*, frame_time::SingleFrameTimeCommand};

use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;

use crate::base::DemoScene;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    let tes_size: i32 = 10;
    fps.frame_ms = 16;

    actions.camera.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));
    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = CubeBuilder::attrs_meta();
    let indices = CubeBuilder::indices_meta();
    let state = base::instance_attr(true, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let idmat = defaultmat.0;
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty_id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    for i in 0..tes_size {
        for j in 0..tes_size {
            for _k in 0..1 {
                
                let cube: Entity = commands.spawn_empty_id();
                actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                actions.transform.tree.push(OpsTransformNodeParent::ops(cube, source));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32)));
                
                let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                let key_curve0 = key_curve0.asset_u64();
                let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(1., 1., 1.)), LocalScaling(Vector3::new(0., 2. * (1.1 + (i as f32).sin()), 0.)), (60. * (1.1 + ((i * j) as f32).cos())) as FrameIndex, 30, EEasingMode::None);
                
                let asset_curve = if let Some(curve) = anime_assets.scaling.get(&key_curve0) {
                    curve
                } else {
                    match anime_assets.scaling.insert(key_curve0, TypeFrameCurve(curve)) {
                        Ok(value) => { value  },
                        Err(_) => { break; },
                    }
                };

                let animation = anime_contexts.scaling.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), cube,  animation));
                // engine.create_target_animation(source, cube, &key_group, animation);
            }
        }
    }

    let mut param = AnimationGroupParam::default(); param.loop_mode = ELoopMode::Positive(Some(1));
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    // animegroupres.scene_ctxs.stop(scene, id_group.clone());
    // let mut param = AnimationGroupParam::default(); param.loop_mode = ELoopMode::Positive(Some(1));
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, param, 0., pi_animation::base::EFillMode::NONE));
    // actions.anime.pause.push(OpsAnimationGroupPause::ops(id_group));
    // let mut param = AnimationGroupParam::default(); param.loop_mode = ELoopMode::Positive(Some(1));
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}


use std::{hash::Hasher};

pub fn shader(
    _id_pass: Entity,
    meta: &Handle<ShaderEffectMeta>,
    key_meta: &Atom,
    vb: &VertexBufferLayoutsComp,
    bindgroups: &BindGroups3D,
    renderalignment: ERenderAlignmentForShader,
    assets: & ShareAssetMgr<Shader3D>,
    device: &RenderDevice,
    engineopt: &EngineCustomPlugins,
) -> Result<Handle<Shader3D>, Shader3D> {
    let key_attributes = &vb.1;

    let (set0, set1, set2, set3) = (&bindgroups.scene, &bindgroups.model, bindgroups.matvalues.as_ref(), bindgroups.textures.as_ref());

    let mut hash = pi_scene_shell::prelude::DefaultHasher::default();
    if let Some(set) = set0 { set.hash_for_shader(&mut hash); }
    if let Some(set) = set1 { set.hash_for_shader(&mut hash); }
    if let Some(set) = set2 { set.hash_for_shader(&mut hash); }
    if let Some(set) = set3 { set.hash_for_shader(&mut hash); }
    let key_shader = KeyShader3D {
        key_meta: key_meta.clone(),
        bind_defines: meta.binddefines,
        key_attributes: key_attributes.clone(),
        renderalignment: renderalignment,
        bindgroups_for_shader: hash.finish(),
    };

    if let Some(shader) = assets.get(&key_shader) {
        // log::debug!("SysPassShaderRequestByModel: 4");
        Ok(shader)
    } else {
        let mut setidx = 0;
        let mut vs_defined_snippets = vec![];
        let mut fs_defined_snippets = vec![];
        let mut vs_extend_varying = String::from("");
        let mut fs_extend_varying = String::from("");
        let mut vs_running_model_snippets = vec![];
        let mut vs_running_attribute_snippets = vec![];
        let vs_running_after_effect_snippets = vec![];
        let vs_running_before_effect_snippets = vec![];
        let mut fs_running_before_effect_snippets = vec![];
        let fs_running_after_effect_snippets = vec![];
    
        // log::error!("Shader: {:?}", key_meta);
        // log::error!("{:?}", key_attributes);
        // log::error!("{:?}", key_attributes.vs_define_code());
    
        vs_defined_snippets.push(key_attributes.vs_define_code());
        vs_extend_varying += &key_attributes.vs_varying_code(meta.varyings.0.len() as u32, meta);
        fs_extend_varying += &key_attributes.fs_varying_code(meta.varyings.0.len() as u32, meta);
    
        if let Some(set) = set0 {
            vs_defined_snippets.push(set.vs_define_code(setidx));
            fs_defined_snippets.push(set.fs_define_code(setidx));
            setidx += 1;
        }
    
        if let Some(set) = set1 {
            let skin = set.key().key.skin;
            vs_defined_snippets.push(set.vs_define_code(setidx));
            fs_defined_snippets.push(set.fs_define_code(setidx));
    
            vs_running_attribute_snippets.push(set.vs_running_model_snippet(meta));
            vs_running_model_snippets.push(skin.running_code());
            vs_running_model_snippets.push(renderalignment.running_code());
    
            vs_defined_snippets.push(renderalignment.define_code());
    
            setidx += 1;
        }
        vs_running_attribute_snippets.push(key_attributes.vs_running_code());
        fs_running_before_effect_snippets.push(key_attributes.fs_running_code(meta));
    
        if let Some(set) = set2 {
            vs_defined_snippets.push(set.vs_define_code(setidx, meta, engineopt));
            fs_defined_snippets.push(set.fs_define_code(setidx, meta, engineopt));
            setidx += 1;
        }
        
        if let Some(set) = set3 {
            vs_defined_snippets.push(set.vs_define_code(setidx, meta, engineopt));
            fs_defined_snippets.push(set.fs_define_code(setidx, meta, engineopt));
            setidx += 1;
        }

        // log::error!("Shader {:?}", key_meta);
        let shader = meta.build_2(
            &device,
            &key_meta,
            &vs_defined_snippets,
            &vs_extend_varying,
            &fs_extend_varying,
            &vs_running_attribute_snippets,
            &vs_running_model_snippets,
            &vs_running_before_effect_snippets, &vs_running_after_effect_snippets,
            &fs_defined_snippets,
            &fs_running_before_effect_snippets, &fs_running_after_effect_snippets,
            engineopt
        );

        assets.insert(key_shader, shader)
    }
}

pub fn main() {

    let (mut app, window, event_loop) = base::test_plugins();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_position: (0., 0., -40.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);
    
    app.add_plugins(PluginTest);

    app.add_systems(StageD3, pi_3d::sys_info_node);
    app.add_systems(StageD3, pi_3d::sys_info_resource);
    app.add_systems(StageD3, pi_3d::sys_info_draw);
    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    // app.run()
    crate::base::run_loop(app, window, event_loop)
}