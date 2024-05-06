#![feature(box_into_inner)]


use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
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

        let tes_size = 10;
        fps.frame_ms = 20;
        
        


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
        actions.renderer.modify.push(OpsRendererCommand::DepthClear(demopass.opaque_renderer, RenderDepthClear(1.)));

        let cameraroot = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(cameraroot, scene)); actions.transform.tree.push(OpsTransformNodeParent::ops(camera01, cameraroot));
        actions.transform.create.push(OpsTransformNode::ops(scene, cameraroot));
        let lightroot = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(lightroot, scene));
        actions.transform.create.push(OpsTransformNode::ops(scene, lightroot));

        let mut lights: Vec<Entity> = vec![];

        actions.scene.shadowmap.push(OpsSceneShadowMap::ops(scene, demopass.shadowtarget));
        {
            let light = light::DemoLight::directlight(&mut commands, scene, lightroot, &mut actions,);

            let position = (0., 5., 0.);
            let direction =  (0., -5., 0.);
            let color = (1., 0.2, 0.2);
            let light = light::DemoLight::spotlight(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
            lights.push(light);

            {
                let pass = DemoScene::PASS_SHADOW;
                let pre_renderer = None;
                let next_renderer = demopass.opaque_renderer;
                let rendertarget = demopass.shadowtarget;
                let shadow = shadow::DemoShadow::init(&mut commands, scene, light, pass, pre_renderer, next_renderer, rendertarget, &mut actions);
                actions.shadow.param.push(OpsShadowGeneratorParam::ShadowMinz(shadow, 0.0));
            }
        }
        // {
        //     let position = (0., 0., 0.);
        //     let direction =  (-1., -0.2, 0.2);
        //     let color = (0.2 * 0.2, 0.8 * 0.2, 0.1 * 0.2);
        //     let light = light::DemoLight::directlight_custom(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
        //     lights.push(light);
        // }
        // {
        //     let position = (0., 0., 0.);
        //     let direction =  (1., -0.2, 0.2);
        //     let color = (0.2 * 0.2, 0.4 * 0.2, 0.8 * 0.2);
        //     let light = light::DemoLight::directlight_custom(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
        //     lights.push(light);
        // }

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
            [ tes_size as f32 * 0.25, 0., -tes_size as f32 * 0.25],
            [ tes_size as f32 * 0.25, 0.,  tes_size as f32 * 0.25],
            [-tes_size as f32 * 0.25, 0., -tes_size as f32 * 0.25],
            [-tes_size as f32 * 0.25, 0.,  tes_size as f32 * 0.25],
            [ tes_size as f32 * 0.75, 0., -tes_size as f32 * 0.75],
            [ tes_size as f32 * 0.75, 0.,  tes_size as f32 * 0.75],
            [-tes_size as f32 * 0.75, 0., -tes_size as f32 * 0.75],
            [-tes_size as f32 * 0.75, 0.,  tes_size as f32 * 0.75],
        ];
        for i in 0..light_colors.len() {
            let color = &light_colors[i]; let pos = &light_position[i];
            let position = (pos[0], pos[1], pos[2]);
            let direction =  (0. - pos[0], 0. - pos[1], 0. - pos[2]);
            let color = (color[0], color[1], color[2]);
            let light = light::DemoLight::spotlight(&mut commands, scene, scene, &mut actions, position, direction, color, 0xFFFFFFFF);
            lights.push(light);
        }


    let lightingmat = {
        let idmat = commands.spawn_empty().id();
        actions.material.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY));
        idmat
    };

    let (vertices, indices) = (CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()));
    let state: MeshInstanceState = base::instance_attr(true, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(0., -1., 0.)));
    actions.material.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_OPAQUE));
    actions.mesh.shadow.push(OpsMeshShadow::CastShadow(source, true));
    lights.iter().for_each(|light| {
        actions.abstructmesh.force_spot_light.push(OpsMeshForceSpotLighting::ops(source, *light, true));
    });
    
    
    let ins = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(ins, scene));
    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, ins));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(ins, ETransformSRT::Scaling(100., 1., 100.)));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(ins, ETransformSRT::Translation(0., -1., 0.)));

        let cell_col = 4.;
        let cell_row = 4.;
        let half = tes_size as f32 * 0.5;
        let ttt = 1; let tttf = 1.;
        for i in 0..(tes_size * ttt) {
            for j in 0..(tes_size * ttt) {
                for k in 0..(tes_size * ttt) {
                    let cube = commands.spawn_empty().id();
                    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    actions.transform.tree.push(OpsTransformNodeParent::ops(cube, source));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation((i as f32 - half) / tttf, (k as f32 - half * 0.5) / tttf + 5., (j as f32 - half) / tttf)));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Scaling(0.25, 0.25, 0.25)));
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
            
        // 创建帧事件
    }


pub fn main() {
    let mut app = base::test_plugins_with_gltf();

    app.add_system(Update, pi_3d::sys_info_node);
    app.add_system(Update, pi_3d::sys_info_resource);
    app.add_system(Update, pi_3d::sys_info_draw);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_system(Startup, setup.after(base::setup_default_mat));
    app.add_system(Startup, base::active_lighting_shadow);
    
    
    // app.run()
    loop { app.update(); }

}