#![feature(box_into_inner)]


use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::{prelude::*, viewer::prelude::{ViewerGlobalPosition, ViewerViewMatrix}, light::PluginLighting};
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use pi_standard_material::shader::StandardShader;
use unlit_material::*;
use pi_winit::event::{Event, WindowEvent};
use pi_world::editor::EntityEditor;

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
        mut editor: EntityEditor,
        mut actions: pi_3d::ActionSets,
        defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut animegroupres: ResourceAnimationGroup,
        mut fps: ResMut<SingleFrameTimeCommand>,
        anime_assets: TypeAnimeAssetMgrs,
        mut anime_contexts: TypeAnimeContexts,
        mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    ) {

        let tes_size = 16;
        fps.frame_ms = 100;
        
        



        // Test Code
        let demopass = base::DemoScene::new(&mut editor, &mut actions, &mut animegroupres, 
            &mut assets.0, &assets.1, &assets.2, &assets.3,
            tes_size as f32, 0.7, (10., 10., -40.), false
        );
        let (scene, camera01) = (demopass.scene, demopass.camera);

        let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut editor, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
        actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));
    
        actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 * 0.7 )));
        actions.camera.target.push(OpsCameraTarget::ops(camera01, -1., -1., 4.));

        let cameraroot = editor.alloc_entity(); actions.transform.tree.push(OpsTransformNodeParent::ops(cameraroot, scene)); actions.transform.tree.push(OpsTransformNodeParent::ops(camera01, cameraroot));
        actions.transform.create.push(OpsTransformNode::ops(scene, cameraroot));

        let lightroot = editor.alloc_entity(); actions.transform.tree.push(OpsTransformNodeParent::ops(lightroot, scene));
        actions.transform.create.push(OpsTransformNode::ops(scene, lightroot));

        actions.scene.shadowmap.push(OpsSceneShadowMap::ops(scene, demopass.shadowtarget));
        // {
        //     let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, scene));
        //     actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(light, 0., 10., -10.));
        //     actions.mesh.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        //     actions.lighting.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
        //     actions.lighting.param.push(ELightModifyCommand::Directional(light, Vector3::new(-1., -1., 1.)));
        //     // actions.lighting.param.push(ELightModifyCommand::ShadowEnable(light, true));
        //     actions.lighting.param.push(ELightModifyCommand::ShadowFrustumSize(light, 40.0));
        //     actions.lighting.color.push(OpsLightColor::ops(light, 1. * 0.2, 0.0 * 0.2, 0.0 * 0.2));
    
        //     // actions.renderer.connect.push(OpsRendererConnect::ops(final_render.clear_entity, light, false));
        //     // actions.renderer.connect.push(OpsRendererConnect::ops(light, id_renderer, false));
    
        //     // actions.renderer.modify.push(OpsRendererCommand::DepthFormat(light, RenderDepthFormat(DepthStencilFormat::Depth32Float)));
        //     // actions.renderer.modify.push(OpsRendererCommand::ColorFormat(light, RenderColorFormat(ColorFormat::Rgba16Float)));
        //     actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(light, true));
        //     actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(light, true));
        //     actions.renderer.modify.push(OpsRendererCommand::DepthClear(light, RenderDepthClear(0.)));
        //     actions.renderer.modify.push(OpsRendererCommand::ColorClear(light, RenderColorClear(0, 0, 0, 0)));
        //     log::warn!("Light: {:?}", light);
        // }

        // {
        //     let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, scene));
        //     actions.mesh.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        //     actions.lighting.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
        //     actions.lighting.param.push(ELightModifyCommand::Directional(light, Vector3::new(-1., -0.2, 0.2)));
        //     actions.lighting.color.push(OpsLightColor::ops(light, 0.0 * 0.2, 0.8 * 0.2, 0.0 * 0.2));
        // }
        // {
        //     let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, scene));
        //     actions.mesh.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        //     actions.lighting.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
        //     actions.lighting.param.push(ELightModifyCommand::Directional(light, Vector3::new(1., -0.2, 0.2)));
        //     actions.lighting.color.push(OpsLightColor::ops(light, 0.0 * 0.2, 0.0 * 0.2, 0.8 * 0.2));
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
            [ tes_size as f32 * 0.25, 2., -tes_size as f32 * 0.25],
            [ tes_size as f32 * 0.25, 2.,  tes_size as f32 * 0.25],
            [-tes_size as f32 * 0.25, 2., -tes_size as f32 * 0.25],
            [-tes_size as f32 * 0.25, 2.,  tes_size as f32 * 0.25],
            [ tes_size as f32 * 0.75, 2., -tes_size as f32 * 0.75],
            [ tes_size as f32 * 0.75, 2.,  tes_size as f32 * 0.75],
            [-tes_size as f32 * 0.75, 2., -tes_size as f32 * 0.75],
            [-tes_size as f32 * 0.75, 2.,  tes_size as f32 * 0.75],
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
        actions.material.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY));
        idmat
    };

    let (vertices, indices) = (CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()));
    let state: MeshInstanceState = base::instance_attr(true, false, false);
    let source = base::DemoScene::mesh(&mut editor, scene, scene, &mut actions,  vertices, indices, state);

    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(0., -1., 0.)));
    actions.material.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_OPAQUE));
    actions.mesh.state.push(OpsMeshStateModify::ops(source, EMeshStateModify::CastShadow(true)));
    lights.iter().for_each(|light| {
        actions.mesh.forcelighting.push(OpsMeshForceLighting::ops(source, *light, EMeshForceLighting::ForcePointLighting(true)));
    });

        let cell_col = 4.;
        let cell_row = 4.;
        let half = tes_size as f32 * 0.5;
        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..tes_size {
                    let cube = editor.alloc_entity();
                    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    actions.transform.tree.push(OpsTransformNodeParent::ops(cube, source));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation((i as f32 - half) * 1., (k as f32 - half * 0.5) * 1., (j as f32 - half) * 1.)));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Scaling(0.25, 0.25, 0.25)));
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
            
        // 创建帧事件
    }


pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();

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