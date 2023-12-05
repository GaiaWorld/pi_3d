#![feature(box_into_inner)]


use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::{prelude::*, viewer::prelude::{ViewerGlobalPosition, ViewerViewMatrix}, light::PluginLighting};
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use standard_material::shader::StandardShader;
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

        let tes_size = 16;
        fps.frame_ms = 100;
        
        



        // Test Code
        let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
            &mut assets.0, &assets.1, &assets.2, &assets.3,
            tes_size as f32, 0.7, (10., 10., -40.), false
        );
        let (scene, camera01) = (demopass.scene, demopass.camera);

        let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
        actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));
    
        actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32 * 0.7));
        actions.camera.target.push(OpsCameraTarget::ops(camera01, -1., -1., 4.));

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
            let direction =  (1., -0.2, 0.2);
            let color = (0.2 * 0.2, 0.4 * 0.2, 0.8 * 0.2);
            let light = light::DemoLight::directlight_custom(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
        }

        let light_colors = [
            [1.0, 0.1, 0.1],
            [0.1, 1.0, 0.1],
            // [0.1, 0.1, 1.0],
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
            let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, lightroot));
            actions.mesh.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
            actions.lighting.create.push(OpsLightCreate::ops(scene, light, ELightType::Point, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
            let color = &light_colors[i]; let pos = &light_position[i];
            actions.lighting.color.push(OpsLightColor::ops(light, color[0], color[1], color[2]));
            actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(light, pos[0], pos[1], pos[2]));
            lights.push(light);
        }

    let attrs = CubeBuilder::attrs_meta();

    let lightingmat = {
        let idmat = commands.spawn_empty().id();
        actions.material.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY));
        idmat
    };

    let source = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = InstanceState::INSTANCE_BASE;
    actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false, ..Default::default() }));
    let id_geo = commands.spawn_empty().id();
    actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs.clone(), Some(CubeBuilder::indices_meta())));
    actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(source, 0., -1., 0.));
    actions.material.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_OPAQUE));
    actions.mesh.depth_compare.push(OpsDepthCompare::ops(source, CompareFunction::Greater));
    lights.iter().for_each(|light| {
        actions.abstructmesh.force_spot_light.push(OpsMeshForceSpotLighting::ops(source, *light, true));
    });

        let cell_col = 4.;
        let cell_row = 4.;
        let half = tes_size as f32 * 0.5;
        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..tes_size {
                    let cube = commands.spawn_empty().id();
                    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    actions.transform.tree.push(OpsTransformNodeParent::ops(cube, source));
                    actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(cube, (i as f32 - half) * 1., (k as f32 - half * 0.5) * 1., (j as f32 - half) * 1.));
                    actions.transform.localscl.push(OpsTransformNodeLocalScaling::ops(cube, 0.25, 0.25, 0.25));
                }
            }
        }

        let id_group = animegroupres.scene_ctxs.create_group(scene).unwrap();
        animegroupres.global.record_group(cameraroot, id_group);
        actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, cameraroot, id_group));
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
                animegroupres.scene_ctxs.add_target_anime(scene, cameraroot, id_group.clone(), animation);
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
                animegroupres.scene_ctxs.add_target_anime(scene, lightroot, id_group.clone(), animation);
            }
        }
        animegroupres.scene_ctxs.start_with_progress(scene, id_group.clone(), AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE);
            
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