#![feature(box_into_inner)]


use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_scene_context::{prelude::*, light::PluginLighting};
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use pi_standard_material::shader::StandardShader;
use unlit_material::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;
#[path = "../light.rs"]
mod light;
#[path = "../shadow.rs"]
mod shadow;
#[path = "../pbr_material.rs"]
mod pbr_material;

#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginLighting);
    }
}

    fn setup(
        mut commands: Commands,
        mut actions: pi_3d::ActionSets,
        defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut animegroupres: ResourceAnimationGroup,
        mut fps: ResMut<SingleFrameTimeCommand>,
        anime_assets: TypeAnimeAssetMgrs,
        mut anime_contexts: TypeAnimeContexts,
        mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    ) {


        let tes_size = 6;
        fps.frame_ms = 100;

        let orthographic_camera = true;
        let camera_position = (10., 10., -10.);

        // Test Code
        let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
            &mut assets.0, &assets.1, &assets.2, &assets.3,
            tes_size as f32, 0.7, camera_position, orthographic_camera
        );
        let (scene, camera01) = (demopass.scene, demopass.camera);

        let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
        actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));
    
        actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32 * 2.));
        
        let cameraroot = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(cameraroot, scene)); actions.transform.tree.push(OpsTransformNodeParent::ops(camera01, cameraroot));
        actions.transform.create.push(OpsTransformNode::ops(scene, cameraroot));
        let lightroot = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(lightroot, scene));
        actions.transform.create.push(OpsTransformNode::ops(scene, lightroot));

        actions.scene.shadowmap.push(OpsSceneShadowMap::ops(scene, demopass.shadowtarget));
        {
            let light = light::DemoLight::directlight(&mut commands, scene, lightroot, &mut actions,);
            log::warn!("Light: {:?}", light);

            {
                let pass = DemoScene::PASS_SHADOW;
                let pre_renderer = None;
                let next_renderer = demopass.opaque_renderer;
                let rendertarget = demopass.shadowtarget;
                let shadow = shadow::DemoShadow::init(&mut commands, scene, light, pass, pre_renderer, next_renderer, rendertarget, &mut actions);
            }
        }
        {
            let position = (0., 0., 0.);
            let direction =  (-1., -0.2, 0.2);
            let color = (0.2 * 0.2, 0.8 * 0.2, 0.1 * 0.2);
            let light = light::DemoLight::directlight_custom(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
        }
        {
            let position = (0., 0., 0.);
            let direction =  (1., -0.5, 0.2);
            let color = (0.2 * 0.2, 0.4 * 0.2, 0.8 * 0.2);
            let light = light::DemoLight::directlight_custom(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
        }

    let lightingmat = {
        
        let idmat = commands.spawn_empty().id();
        actions.material.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY));
        // actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        //     slotname: Atom::from(BlockMainTexture::KEY_TEX),
        //     filter: true,
        //     sample: KeySampler::linear_repeat(),
        //     url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/fractal.png"),
        // }));
        idmat
    };

    let (vertices, indices) = (CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()));
    let state: MeshInstanceState = base::instance_attr(true, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    actions.material.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_OPAQUE));
    actions.mesh.shadow.push(OpsMeshShadow::CastShadow(source, true));
    
    let ins = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(ins, scene));
    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, ins));
    actions.transform.localscl.push(OpsTransformNodeLocalScaling::ops(ins, 100., 1., 100.));
    actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(ins, 0., -1., 0.));

        let cell_col = 4.;
        let cell_row = 4.;
        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(cube, scene));
                    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(cube, (i + 1) as f32 * 3. - (tes_size) as f32, 0., j as f32 * 3. - (tes_size) as f32));
                    actions.transform.localscl.push(OpsTransformNodeLocalScaling::ops(cube, 1.,  (f32::sin((i * j) as f32) * 0.5 + 0.5) * 6., 1.));
                }
            }
        }

        let id_group = commands.spawn_empty().id();
        // animegroupres.scene_ctxs.create_group(scene).unwrap();
        // animegroupres.global.record_group(source, id_group);
        actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
        // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
        {
            let key_curve0 = pi_atom::Atom::from((0).to_string());
            let key_curve0 = key_curve0.asset_u64();
            let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(0., 6.28, 0.)), 60 as FrameIndex, 30, EEasingMode::None);
            let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) { Some(curve) } else {
                match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                    Ok(value) => { Some(value) },
                    Err(_) => { None },
                }
            };
            if let Some(asset_curve) = asset_curve {
                let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), cameraroot, animation));
            }
        }
        {
            let key_curve0 = pi_atom::Atom::from((1).to_string());
            let key_curve0 = key_curve0.asset_u64();
            let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(0., 6.28 * 2., 0.)), 60 as FrameIndex, 30, EEasingMode::None);
            let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) { Some(curve) } else {
                match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                    Ok(value) => { Some(value) },
                    Err(_) => { None },
                }
            };
            if let Some(asset_curve) = asset_curve {
                let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), lightroot, animation));
            }
        }
        actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE));
}


pub fn main() {
    let mut app = base::test_plugins_with_gltf();

    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_systems(Startup, setup.after(base::setup_default_mat));
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    // app.run()
    loop { app.update(); }

}