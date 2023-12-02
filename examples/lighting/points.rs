#![feature(box_into_inner)]


use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::{prelude::*, viewer::prelude::{ViewerGlobalPosition, ViewerViewMatrix}, light::PluginLighting};
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
        // mut scenecmds: ActionSetScene,
        // mut cameracmds: ActionSetCamera,
        // mut transformcmds: ActionSetTransform,
        // mut lightingcmds: ActionSetLighting,
        // mut meshcmds: ActionSetMesh,
        // mut abstructmeshcmds: ActionSetAbstructMesh,
        mut geometrycmd: ActionSetGeometry,
        mut matcmds: ActionSetMaterial,
        defaultmat: Res<SingleIDBaseDefaultMaterial>,
        mut animegroupcmd: ActionSetAnimationGroup,
        mut fps: ResMut<SingleFrameTimeCommand>,
        mut renderercmds: ActionSetRenderer,
        anime_assets: TypeAnimeAssetMgrs,
        mut anime_contexts: TypeAnimeContexts,
        mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
        mut cmds: (ActionSetScene, ActionSetCamera, ActionSetTransform, ActionSetLighting, ActionSetMesh, ActionSetInstanceMesh, ActionSetAbstructMesh)
    ) {

        let tes_size = 16;
        fps.frame_ms = 100;
        
        

        let (mut scenecmds, mut cameracmds, mut transformcmds, mut lightingcmds, mut meshcmds, mut instancemeshcmds, mut abstructmeshcmds) = cmds;


        // Test Code
        let demopass = base::DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut renderercmds, 
            &mut assets.0, &assets.1, &assets.2, &assets.3,
            tes_size as f32, 0.7, (10., 10., -40.), false
        );
        let (scene, camera01) = (demopass.scene, demopass.camera);

        let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut matcmds, &mut meshcmds, &mut geometrycmd, &mut cameracmds, &mut transformcmds, &mut renderercmds, scene, demopass.transparent_renderer,demopass.transparent_target);
        renderercmds.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));
    
        cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32 * 0.7));
        cameracmds.target.push(OpsCameraTarget::ops(camera01, -1., -1., 4.));

        let cameraroot = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(cameraroot, scene)); transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, cameraroot));
        transformcmds.create.push(OpsTransformNode::ops(scene, cameraroot));

        let lightroot = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(lightroot, scene));
        transformcmds.create.push(OpsTransformNode::ops(scene, lightroot));

        scenecmds.shadowmap.push(OpsSceneShadowMap::ops(scene, demopass.shadowtarget));
        // {
        //     let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, scene));
        //     transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(light, 0., 10., -10.));
        //     meshcmds.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        //     lightingcmds.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
        //     lightingcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(-1., -1., 1.)));
        //     // lightingcmds.param.push(ELightModifyCommand::ShadowEnable(light, true));
        //     lightingcmds.param.push(ELightModifyCommand::ShadowFrustumSize(light, 40.0));
        //     lightingcmds.color.push(OpsLightColor::ops(light, 1. * 0.2, 0.0 * 0.2, 0.0 * 0.2));
    
        //     // renderercmds.connect.push(OpsRendererConnect::ops(final_render.clear_entity, light, false));
        //     // renderercmds.connect.push(OpsRendererConnect::ops(light, id_renderer, false));
    
        //     // renderercmds.modify.push(OpsRendererCommand::DepthFormat(light, RenderDepthFormat(DepthStencilFormat::Depth32Float)));
        //     // renderercmds.modify.push(OpsRendererCommand::ColorFormat(light, RenderColorFormat(ColorFormat::Rgba16Float)));
        //     renderercmds.modify.push(OpsRendererCommand::AutoClearColor(light, true));
        //     renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(light, true));
        //     renderercmds.modify.push(OpsRendererCommand::DepthClear(light, RenderDepthClear(0.)));
        //     renderercmds.modify.push(OpsRendererCommand::ColorClear(light, RenderColorClear(0, 0, 0, 0)));
        //     log::warn!("Light: {:?}", light);
        // }

        // {
        //     let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, scene));
        //     meshcmds.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        //     lightingcmds.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
        //     lightingcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(-1., -0.2, 0.2)));
        //     lightingcmds.color.push(OpsLightColor::ops(light, 0.0 * 0.2, 0.8 * 0.2, 0.0 * 0.2));
        // }
        // {
        //     let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, scene));
        //     meshcmds.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        //     lightingcmds.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
        //     lightingcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(1., -0.2, 0.2)));
        //     lightingcmds.color.push(OpsLightColor::ops(light, 0.0 * 0.2, 0.0 * 0.2, 0.8 * 0.2));
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
            let light = light::DemoLight::pointlight(&mut commands, scene, scene, &mut transformcmds, &mut lightingcmds, &mut meshcmds.layermask, position, color, 0xFFFFFFFF);
            lights.push(light);
        }

    let lightingmat = {
        let idmat = commands.spawn_empty().id();
        matcmds.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY));
        idmat
    };

    let (vertices, indices) = (CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()));
    let mut state: MeshInstanceState = MeshInstanceState::default();
    state.state = InstanceState::INSTANCE_BASE;
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut meshcmds, &mut geometrycmd, &mut transformcmds, vertices, indices, state);

    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(source, 0., -1., 0.));
    matcmds.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_OPAQUE));
    meshcmds.shadow.push(OpsMeshShadow::CastShadow(source, true));
    lights.iter().for_each(|light| {
        abstructmeshcmds.force_point_light.push(OpsMeshForcePointLighting::ops(source, *light, true));
    });

        let cell_col = 4.;
        let cell_row = 4.;
        let half = tes_size as f32 * 0.5;
        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..tes_size {
                    let cube = commands.spawn_empty().id();
                    instancemeshcmds.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    transformcmds.tree.push(OpsTransformNodeParent::ops(cube, source));
                    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cube, (i as f32 - half) * 1., (k as f32 - half * 0.5) * 1., (j as f32 - half) * 1.));
                    transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(cube, 0.25, 0.25, 0.25));
                }
            }
        }

        let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
        animegroupcmd.global.record_group(cameraroot, id_group);
        animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, cameraroot, id_group));
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
                animegroupcmd.scene_ctxs.add_target_anime(scene, cameraroot, id_group.clone(), animation);
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
                animegroupcmd.scene_ctxs.add_target_anime(scene, lightroot, id_group.clone(), animation);
            }
        }
        animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE);
            
        // 创建帧事件
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