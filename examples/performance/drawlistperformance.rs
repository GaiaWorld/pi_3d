#![feature(box_into_inner)]

use base::{DemoScene, DemoWindowEvent};
use bevy_ecs::system::adapter::unwrap;
use pi_animation::loop_mode::ELoopMode;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::{BlockMainTexture, BlockEmissiveTexture, BlockMainTextureUVOffsetSpeed};
use pi_scene_context::{prelude::*, light::PluginLighting};
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::BallBuilder};
use pi_trail_renderer::{OpsTrail, OpsTrailAgeControl};
use pi_winit::event::WindowEvent;
use rand::Rng;
use unlit_material::MainOpacityShader;

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
#[path = "../distortion_material.rs"]
mod distortion_material;

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
        mut animegroupres: ResourceAnimationGroup,
        mut fps: ResMut<SingleFrameTimeCommand>,
        anime_assets: TypeAnimeAssetMgrs,
        mut anime_contexts: TypeAnimeContexts,
        mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
        mut matmetas: ResMut<ShareAssetMgr<ShaderEffectMeta>>,
        demooption: Res<base::DemoOption>,
        mut events: ResMut<DemoWindowEvent>,
        engineopt: Res<EngineCustomPlugins>,
    ) {
        let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
            (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
        } else { return; };
        

    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(MainOpacityShader::KEY), MainOpacityShader::meta(&engineopt));

        events.viewer = Some(camera01);

        let tes_size = 20;
        fps.frame_ms = 5;

        let limit = assets.1.limits();
        log::warn!("{:?}", limit);

        actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 * 2. )));
        actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::Fov( 2.5 )));

        actions.scene.options.push(OpsSceneOption::brdf(scene, Atom::from("./assets/images/fractal.png"), false));
        actions.scene.options.push(OpsSceneOption::envtexture(scene, Some(Atom::from("./assets/images/01.env")), false));
        
        let cameraroot = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(cameraroot, scene)); actions.transform.tree.push(OpsTransformNodeParent::ops(camera01, cameraroot));
        actions.transform.create.push(OpsTransformNode::ops(scene, cameraroot));
        // actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(cameraroot, 0., 0., 0.));
        let lightroot = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(lightroot, scene));
        actions.transform.create.push(OpsTransformNode::ops(scene, lightroot));

        actions.scene.options.push(OpsSceneOption::shadowmap(scene, demopass.shadowtarget))
        .push(OpsSceneOption::ambientcolor(scene, 0.0, 0.0, 0.0))
        ;
        let shadow_renderer = {
            let light: Entity = light::DemoLight::directlight(&mut commands, scene, lightroot, &mut actions,);
            log::warn!("Light: {:?}", light);

            {
                let pass = DemoScene::PASS_SHADOW;
                let pre_renderer = None;
                let next_renderer = demopass.opaque_renderer;
                let rendertarget = demopass.shadowtarget;
                let shadow = shadow::DemoShadow::init(&mut commands, scene, light, pass, pre_renderer, next_renderer, rendertarget, &mut actions);
                shadow
            }
        };


        let light_colors: [[f32; 3]; 3] = [
            [1.0, 0.1, 0.1],
            [0.5, 1.0, 0.5],
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
            let light = light::DemoLight::pointlight(&mut commands, scene, scene, &mut actions, position, color, 0xFFFFFFFF);
            lights.push(light);
        }


    let lightingmat = {
        
        let idmat = commands.spawn_empty_id();
        actions.material.create.push(OpsMaterialCreate::ops(idmat, pbr_material::ShaderPBR::KEY));
        // actions.material.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY, EPassTag::Opaque));
        actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
            slotname: Atom::from(BlockMainTexture::KEY_TEX),
            sample: KeySampler::linear_repeat(),
            url: EKeyTexture::from("./assets/images/fractal.png"),
            ..Default::default()
        }));
        idmat
    };
    
    {
        let vertices = CubeBuilder::attrs_meta();
        let indices = CubeBuilder::indices_meta();
        let mut state: MeshInstanceState = MeshInstanceState::default();
        // state.state = InstanceState::INSTANCE_BASE | InstanceState::INSTANCE_CUSTOM_VEC4_A | InstanceState::INSTANCE_CUSTOM_VEC4_B;
        let cube = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

        log::error!("lightingmat Cube {:?}", cube);
        actions.material.usemat.push(OpsMaterialUse::Use(cube, lightingmat, DemoScene::PASS_OPAQUE));
        actions.mesh.state.push(OpsMeshStateModify::ops(cube, EMeshStateModify::CastShadow(true)));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Scaling(100., 1., 100.)));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation(0., -0.5, 0.)));
    }

    let (vertices, indices) = (BallBuilder::attrs_meta(), Some(BallBuilder::indices_meta()));
    let state: MeshInstanceState = MeshInstanceState {
        instance_matrix: true,
        instances: vec![
            CustomVertexAttribute::new(Atom::from("InsV2"), Atom::from(""), ECustomVertexType::Vec2, Some(Atom::from("uMetallicRoughness")))
        ],
        use_single_instancebuffer: false,
    };
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);
    
    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(unlit_material::PlanarShadow::KEY), unlit_material::PlanarShadow::meta(&engineopt));
    let planarmat =  {
        let idmat = commands.spawn_empty_id();
        actions.material.create.push(OpsMaterialCreate::ops(idmat, unlit_material::PlanarShadow::KEY));
        idmat
    };
    actions.material.usemat.push(OpsMaterialUse::Use(source, planarmat, DemoScene::PASS_SKY_WATER));
    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_TRANSPARENT, blend));
    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_SKY_WATER, blend));
    // actions.mesh.stencil_state.push(OpsStencilState::ops(source, DemoScene::PASS_TRANSPARENT, EStencilState::Write(1)));
    // actions.mesh.stencil_state.push(OpsStencilState::ops(source, DemoScene::PASS_TRANSPARENT, EStencilState::Front(StencilFaceState{
    //     compare: CompareFunction::NotEqual,
    //     fail_op: StencilOperation::Keep,
    //     depth_fail_op: StencilOperation::Keep,
    //     pass_op: StencilOperation::Keep,
    // })));
    actions.mesh.render_state.push(OpsRenderState::depth_state(source, DemoScene::PASS_SKY_WATER, EDepthState::Write(false)));

    log::error!("lightingmat Ball {:?}", source);
    actions.material.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_TRANSPARENT));
    actions.mesh.state.push(OpsMeshStateModify::ops(source, EMeshStateModify::CastShadow(true)));
    lights.iter().for_each(|light| {
        actions.mesh.forcelighting.push(OpsMeshForceLighting::ops(source, *light, EMeshForceLighting::ForcePointLighting(true)));
    });

        for i in 0..tes_size {
            for j in 0..tes_size*4 {
                for k in 0..1 {
                    let cube = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(cube, scene));
                    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation((i + 1) as f32 * 2. - (tes_size) as f32, 0.5, j as f32 * 2. - (tes_size) as f32)));
                    // actions.transform.localscl.push(OpsTransformNodeLocalScaling::ops(cube, 1.,  1., 1.));
                    actions.instance.attr.push(OpsInstanceAttr::ops(cube, EInstanceAttr::Vec2([(i as f32) / (tes_size as f32 - 1.), (j as f32) / (tes_size as f32 - 1.)]), Atom::from("InsV2")));
                    actions.transform.collider.push(OpsCollider::ops(cube, (-0.5, -0.5, -0.5), (0.5, 0.5, 0.5), 1. * (f32::sqrt(3.) / 3. - 1.)));
                    // actions.mesh.state.push(OpsMeshStateModify::ops(cube, EMeshStateModify::BoundingCullingMode(ECullingStrategy::None)));
                    actions.mesh.render_state.push(OpsRenderState::render_queue(cube, 0, j as i32));
                }
            }
        }

        let id_group = commands.spawn_empty_id();
        // animegroupres.scene_ctxs.create_group(scene).unwrap();
        // animegroupres.global.record_group(source, id_group);
        actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
        // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
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
                actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), lightroot, animation));
            }
        }
        actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE));

        let (targets, device, asset_samp, atlas_allocator) = (&mut assets.0, &assets.1, &assets.2, &assets.3);
        let opaquetarget = targets.create( KeySampler::linear_repeat(), ColorFormat::Rgba8Unorm, DepthStencilFormat::Depth32Float, 512, 512 ); 
        let (opaque_texture_renderer, opaque_texture_renderer_camera) = copy::PluginImageCopy::init(&mut commands, &mut actions, scene,
            demopass.skywater_renderer, demopass.transparent_renderer, demopass.opaque_target.clone(), Some(KeyCustomRenderTarget::Custom(opaquetarget.unwrap()))
        );

        actions.renderer.connect.push(OpsRendererConnect::ops(demopass.skywater_renderer, demopass.transparent_renderer, true));

        {
            let distortiommat = commands.spawn_empty_id();
            actions.material.create.push(OpsMaterialCreate::ops(distortiommat, distortion_material::ShaderDistortion::KEY));
            actions.material.valb.push(OpsUniformValB::texture(distortiommat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockMainTexture::KEY_TEX), url: EKeyTexture::from("./assets/images/eff_wm_trail_fml_001_89_clamp.png"), sample: KeySampler::linear_repeat(), ..Default::default() }));
            // actions.material.val.push(OpsUniformVal::vec2(distortiommat, Atom::from(BlockMainTextureUVOffsetSpeed::KEY_PARAM), 100., 100.));
            actions.material.valb.push(OpsUniformValB::texture_from_target(distortiommat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockEmissiveTexture::KEY_TEX), ..Default::default() }, opaquetarget.unwrap(), Atom::from(BlockEmissiveTexture::KEY_TILLOFF)));
            // actions.material.val.push(OpsUniformVal::vec3(distortiommat, Atom::from(BlockMainTexture::KEY_COLOR), 1., 0.5, 0.5));

            let node = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
            actions.transform.create.push(OpsTransformNode::ops(scene, node));

            // let key_group = pi_atom::Atom::from("key_group");
            let id_group = commands.spawn_empty_id();
            // animegroupres.scene_ctxs.create_group(scene).unwrap();
            // animegroupres.global.record_group(source, id_group);
            actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
            // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
            
            {
                let key_curve0 =  pi_atom::Atom::from("testcc"); 
                let key_curve0 = key_curve0.asset_u64();
                let curve = FrameCurve::<CameraFov>::curve_easing(CameraFov(0.1), CameraFov(2.5), (5. * 60.) as FrameIndex, 30, EEasingMode::None);
                
                let asset_curve = if let Some(curve) = anime_assets.camerafov.get(&key_curve0) { curve } else {
                    match anime_assets.camerafov.insert(key_curve0, TypeFrameCurve(curve)) {
                        Ok(value) => { value  },
                        Err(_) => { return; },
                    }
                };

                let animation = anime_contexts.camerafov.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), camera01, animation));
            }
            {
                let key_curve0 =  pi_atom::Atom::from("test2"); 
                let key_curve0 = key_curve0.asset_u64();
                let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(0., 3.1415926 * 4., 0.)), (5. * 60.) as FrameIndex, 30, EEasingMode::None);
                
                let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) { curve } else {
                    match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                        Ok(value) => { value  },
                        Err(_) => { return; },
                    }
                };

                let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node, animation));
            }

            let mut random = pi_wy_rng::WyRng::default();
            for idx in 0..0 {
                // let scalescalar = if idx % 2 == 0 { 1. } else { -1. };

                let source = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, node));
                // if idx == 0 {
                //     actions.mesh.create.push(OpsMeshCreation::ops(scene, source));
                //     actions.material.usemat.push(OpsMaterialUse::ops(source, idmat));
                //     let id_geo = commands.spawn_empty_id();
                //     let instancestate = 0;
                //     actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), CubeBuilder::indices_meta(), instancestate));
                // } else {
                    actions.transform.create.push(OpsTransformNode::ops(scene, source));
                // }
                let z = random.gen_range(-5.0..5.0);
                let y = random.gen_range(0.1..5.0);
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(random.gen_range(-5.0..5.0), y, z)));
                let scl = random.gen_range(2.0..3.0) - y * 0.2;
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Scaling(scl, scl, scl)));
                // actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Euler(3., 0., 0.)));

                let trail = commands.spawn_empty_id();
                actions.transform.tree.push(OpsTransformNodeParent::ops(trail, scene));
                actions.mesh.create.push(OpsMeshCreation::ops(scene, trail, MeshInstanceState::default()));
                actions.trail.create.push(OpsTrail::ops(scene, source, trail));
                actions.trail.age.push(OpsTrailAgeControl::ops(trail, 500));
                actions.material.usemat.push(OpsMaterialUse::ops(trail, distortiommat, DemoScene::PASS_TRANSPARENT));
                let mut blend = ModelBlend::default(); blend.combine();
                actions.mesh.render_state.push(OpsRenderState::blend(trail, DemoScene::PASS_TRANSPARENT, blend));
                actions.mesh.render_state.push(OpsRenderState::depth_state(trail, DemoScene::PASS_TRANSPARENT, EDepthState::Compare(CompareFunction::Always)));
            }
            
            let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 2.;param.loop_mode = ELoopMode::PositivePly(None);
            // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, param, 0., pi_animation::base::EFillMode::NONE));
        }
}

pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins_with_gltf();
    app
    .add_plugins(pi_pbr::PluginPBR)
    .add_plugins(pbr_material::PluginPBRMaterial)
    .add_plugins(distortion_material::PluginDistortionMaterial)
    ;

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: false,
        camera_size: 10.,
        camera_fov: 0.7,
        camera_position: (0., 4., -10.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, base::active_lighting_shadow);
    #[cfg(feature = "use_bevy")]
    app.add_startup_system(Update, base::active_lighting_shadow);
    
    crate::base::run_loop(app, window, event_loop)
    // app.run()
    // crate::base::run_loop(app, window, event_loop)

}
