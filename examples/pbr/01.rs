#![feature(box_into_inner)]


use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::{prelude::*, light::PluginLighting};
use pi_mesh_builder::{cube::*, ball::BallBuilder};
use pi_world::editor::EntityEditor;
use pi_winit::event::{Event, WindowEvent};

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

pub fn setup(
        mut editor: EntityEditor,
        mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
        mut fps: ResMut<SingleFrameTimeCommand>,
        anime_assets: TypeAnimeAssetMgrs,
        mut anime_contexts: TypeAnimeContexts,
        mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    ) {


        let tes_size = 10;
        fps.frame_ms = 100;
        
        
        let orthographic_camera = false;
        let camera_position = (0., 20., -20.);

        // Test Code
        let demopass = base::DemoScene::new(
            &mut editor, &mut actions, &mut animegroupres,
            &mut assets.0, &assets.1, &assets.2, &assets.3,
            tes_size as f32, 0.7, camera_position, orthographic_camera
        );
        let (scene, camera01) = (demopass.scene, demopass.camera);

        let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut editor, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
        actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));
    
        actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 * 2. )));

        actions.scene.brdf.push(OpsSceneBRDF::ops(scene, Atom::from("./assets/images/fractal.png"), false));
        actions.scene.env.push(OpsSceneEnvTexture::ops(scene, Some(Atom::from("./assets/images/01.env")), false));
        
        let cameraroot = editor.alloc_entity(); actions.transform.tree.push(OpsTransformNodeParent::ops(cameraroot, scene)); actions.transform.tree.push(OpsTransformNodeParent::ops(camera01, cameraroot));
        actions.transform.create.push(OpsTransformNode::ops(scene, cameraroot));
        // actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(cameraroot, 0., 0., 0.));
        let lightroot = editor.alloc_entity(); actions.transform.tree.push(OpsTransformNodeParent::ops(lightroot, scene));
        actions.transform.create.push(OpsTransformNode::ops(scene, lightroot));

        actions.scene.shadowmap.push(OpsSceneShadowMap::ops(scene, demopass.shadowtarget));
        {
            let light = light::DemoLight::directlight(&mut editor, scene, lightroot, &mut actions,);
            log::warn!("Light: {:?}", light);

            {
                let pass = DemoScene::PASS_SHADOW;
                let pre_renderer = None;
                let next_renderer = demopass.opaque_renderer;
                let rendertarget = demopass.shadowtarget;
                let shadow = shadow::DemoShadow::init(&mut editor, scene, light, pass, pre_renderer, next_renderer, rendertarget, &mut actions);
            }
        }
        // {
        //     let position = (0., 0., 0.);
        //     let direction =  (-1., -0.2, 0.2);
        //     let color = (0.2 * 0.2, 0.8 * 0.2, 0.1 * 0.2);
        //     let light = light::DemoLight::directlight_custom(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
        // }
        // {
        //     let position = (0., 0., 0.);
        //     let direction =  (1., -0.2, 0.2);
        //     let color = (0.2 * 0.2, 0.4 * 0.2, 0.8 * 0.2);
        //     let light = light::DemoLight::directlight_custom(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
        // }


        let light_colors = [
            [1.0, 0.1, 0.1],
            [0.1, 1.0, 0.1],
            [0.1, 0.1, 1.0],
            // [0.1, 0.8, 1.0],
            // [0.1, 1.0, 0.1],
            // [0.1, 0.1, 1.0],
            // [0.1, 0.8, 1.0],
            // [1.0, 0.1, 0.1],
        ];
        let light_position = [
            [ tes_size as f32 * 0.25, 0.2, -tes_size as f32 * 0.25],
            [ tes_size as f32 * 0.25, 0.2,  tes_size as f32 * 0.25],
            [-tes_size as f32 * 0.25, 0.2, -tes_size as f32 * 0.25],
            [-tes_size as f32 * 0.25, 0.2,  tes_size as f32 * 0.25],
            [ tes_size as f32 * 0.75, 0.2, -tes_size as f32 * 0.75],
            [ tes_size as f32 * 0.75, 0.2,  tes_size as f32 * 0.75],
            [-tes_size as f32 * 0.75, 0.2, -tes_size as f32 * 0.75],
            [-tes_size as f32 * 0.75, 0.2,  tes_size as f32 * 0.75],
        ];
        let mut lights: Vec<Entity> = vec![];
        for i in 0..light_colors.len() {
            let color = &light_colors[i]; let pos = &light_position[i];
            let position = (pos[0], pos[1], pos[2]);
            let direction =  (1., -0.2, 0.2);
            let color = (color[0], color[1], color[2]);
            let light = light::DemoLight::pointlight(&mut editor, scene, scene, &mut actions, position, color, 0xFFFFFFFF);
            lights.push(light);
        }


    let lightingmat = {
        
        let idmat = editor.alloc_entity();
        actions.material.create.push(OpsMaterialCreate::ops(idmat, pbr_material::ShaderPBR::KEY));
        // actions.material.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY, EPassTag::Opaque));
        actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
            slotname: Atom::from(BlockMainTexture::KEY_TEX),
            filter: true,
            sample: KeySampler::linear_repeat(),
            url: EKeyTexture::from("./assets/images/fractal.png"),
        }));
        idmat
    };
    
    {
        let vertices = CubeBuilder::attrs_meta();
        let indices = Some(CubeBuilder::indices_meta());
        let mut state: MeshInstanceState = MeshInstanceState::default();
        // state.state = InstanceState::INSTANCE_BASE | InstanceState::INSTANCE_CUSTOM_VEC4_A | InstanceState::INSTANCE_CUSTOM_VEC4_B;
        let cube = base::DemoScene::mesh(&mut editor, scene, scene, &mut actions,  vertices, indices, state);

        actions.material.usemat.push(OpsMaterialUse::Use(cube, lightingmat, DemoScene::PASS_OPAQUE));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Scaling(100., 1., 100.)));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation(0., -1., 0.)));
    }

    let (vertices, indices) = (BallBuilder::attrs_meta(), Some(BallBuilder::indices_meta()));
    let state: MeshInstanceState = MeshInstanceState {
        instance_matrix: true,
        instances: vec![
            CustomVertexAttribute::new(Atom::from("InsV2"), Atom::from("uMetallic = InsV2.x; uRoughness = InsV2.y;"), ECustomVertexType::Vec2, Some(Atom::from("uMetallic")))
        ],
        use_single_instancebuffer: false,
    };
    let source = base::DemoScene::mesh(&mut editor, scene, scene, &mut actions,  vertices, indices, state);

    actions.material.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_OPAQUE));
    actions.mesh.state.push(OpsMeshStateModify::ops(source, EMeshStateModify::CastShadow(true)));
    lights.iter().for_each(|light| {
        actions.mesh.forcelighting.push(OpsMeshForceLighting::ops(source, *light, EMeshForceLighting::ForcePointLighting(true)));
    });

        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = editor.alloc_entity(); actions.transform.tree.push(OpsTransformNodeParent::ops(cube, scene));
                    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation((i + 1) as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32)));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Scaling(1.,  1., 1.)));
                    actions.instance.attr.push(OpsInstanceAttr::ops(cube, EInstanceAttr::Vec2([(i as f32) / (tes_size as f32 - 1.), (j as f32) / (tes_size as f32 - 1.)]), Atom::from("InsV2")));
                }
            }
        }

        let id_group = editor.alloc_entity();
        // animegroupres.scene_ctxs.create_group(scene).unwrap();
        // animegroupres.global.record_group(source, id_group);
        actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
        // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
        {
            let key_curve0 = pi_atom::Atom::from((0).to_string());
            let key_curve0 = key_curve0.asset_u64();
            let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(0., 6.28, 0.)), 300 as FrameIndex, 30, EEasingMode::None);
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
            let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(0., 6.28 * 2., 0.)), 300 as FrameIndex, 30, EEasingMode::None);
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
    let  (mut app, window, event_loop) = base::test_plugins_with_gltf();
    app.add_plugins(
        pi_pbr::PluginPBR
    );
    app.add_plugins(
        pbr_material::PluginPBRMaterial
    );

    app.add_system(Update, pi_3d::sys_info_node);
    app.add_system(Update, pi_3d::sys_info_resource);
    app.add_system(Update, pi_3d::sys_info_draw);
    app.world.get_single_res_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    app.add_startup_system(Update, base::active_lighting_shadow);
    
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_window_id) => {
                app.run();
            }
            
            _ => (),
        }
    });

}