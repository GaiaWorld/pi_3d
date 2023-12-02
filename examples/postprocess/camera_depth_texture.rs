#![feature(box_into_inner)]


use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::{BlockMainTexture, BlockEmissiveTexture, BlockMainTextureUVOffsetSpeed};
use pi_scene_context::{prelude::*, light::PluginLighting};
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::BallBuilder};

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
#[path = "../predepth.rs"]
mod predepth;
#[path = "../water.rs"]
mod water;

#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginLighting);
    }
}

    fn setup(
        mut commands: Commands,
        mut shadowcmds: ActionSetShadow,
        mut geometrycmd: ActionSetGeometry,
        mut matcmds: ActionSetMaterial,
        mut animegroupcmd: ActionSetAnimationGroup,
        mut fps: ResMut<SingleFrameTimeCommand>,
        mut renderercmds: ActionSetRenderer,
        anime_assets: TypeAnimeAssetMgrs,
        mut anime_contexts: TypeAnimeContexts,
        mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
        mut cmds: (ActionSetScene, ActionSetCamera, ActionSetTransform, ActionSetLighting, ActionSetMesh, ActionSetInstanceMesh, ActionSetAbstructMesh)
    ) {

        let (mut scenecmds, mut cameracmds, mut transformcmds, mut lightingcmds, mut meshcmds, mut instancemeshcmds, mut abstructmeshcmds) = cmds;

        let tes_size = 10;
        fps.frame_ms = 100;
        
        
        let orthographic_camera = false;
        let camera_position = (0., 20., -20.);

        // Test Code
        let demopass = base::DemoScene::new(
            &mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut renderercmds,
            &mut assets.0, &assets.1, &assets.2, &assets.3,
            tes_size as f32, 0.7, camera_position, orthographic_camera
        );
        let (scene, camera01) = (demopass.scene, demopass.camera);

        let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut matcmds, &mut meshcmds, &mut geometrycmd, &mut cameracmds, &mut transformcmds, &mut renderercmds, scene, demopass.transparent_renderer,demopass.transparent_target);
        renderercmds.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));
    
        cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32 * 2.));

        scenecmds.brdf.push(OpsSceneBRDF::ops(scene, Atom::from("./assets/images/fractal.png"), false));
        scenecmds.env.push(OpsSceneEnvTexture::ops(scene, Some(Atom::from("./assets/images/01.env")), false));
        
        let cameraroot = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(cameraroot, scene)); transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, cameraroot));
        transformcmds.create.push(OpsTransformNode::ops(scene, cameraroot));
        // transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cameraroot, 0., 0., 0.));
        let lightroot = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(lightroot, scene));
        transformcmds.create.push(OpsTransformNode::ops(scene, lightroot));

        scenecmds.shadowmap.push(OpsSceneShadowMap::ops(scene, demopass.shadowtarget));
        let shadow_renderer = {
            let light = light::DemoLight::directlight(&mut commands, scene, lightroot, &mut transformcmds, &mut lightingcmds, &mut meshcmds.layermask);
            log::warn!("Light: {:?}", light);

            {
                let pass = DemoScene::PASS_SHADOW;
                let pre_renderer = None;
                let next_renderer = demopass.opaque_renderer;
                let rendertarget = demopass.shadowtarget;
                let shadow = shadow::DemoShadow::init(&mut commands, scene, light, pass, pre_renderer, next_renderer, rendertarget, &mut renderercmds, &mut shadowcmds);
                shadow
            }
        };


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
            let light = light::DemoLight::pointlight(&mut commands, scene, scene, &mut transformcmds, &mut lightingcmds, &mut meshcmds.layermask, position, color, 0xFFFFFFFF);
            lights.push(light);
        }


    let lightingmat = {
        
        let idmat = commands.spawn_empty().id();
        matcmds.create.push(OpsMaterialCreate::ops(idmat, pbr_material::ShaderPBR::KEY));
        // matcmds.create.push(OpsMaterialCreate::ops(idmat, StandardShader::KEY, EPassTag::Opaque));
        matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
            slotname: Atom::from(BlockMainTexture::KEY_TEX),
            filter: true,
            sample: KeySampler::linear_repeat(),
            url: EKeyTexture::from("./assets/images/fractal.png"),
        }));
        idmat
    };

    let predepthmat = {
        let idmat = commands.spawn_empty().id();
        matcmds.create.push(OpsMaterialCreate::ops(idmat, predepth::ShaderPreDepth::KEY));
        idmat
    };
    
    {
        let vertices = CubeBuilder::attrs_meta();
        let indices = Some(CubeBuilder::indices_meta());
        let mut state: MeshInstanceState = MeshInstanceState::default();
        // state.state = InstanceState::INSTANCE_BASE | InstanceState::INSTANCE_CUSTOM_VEC4_A | InstanceState::INSTANCE_CUSTOM_VEC4_B;
        let cube = base::DemoScene::mesh(&mut commands, scene, scene, &mut meshcmds, &mut geometrycmd, &mut transformcmds, vertices, indices, state);

        matcmds.usemat.push(OpsMaterialUse::Use(cube, lightingmat, DemoScene::PASS_OPAQUE));
        matcmds.usemat.push(OpsMaterialUse::Use(cube, predepthmat, DemoScene::PASS_PRE_DEPTH));

        meshcmds.shadow.push(OpsMeshShadow::CastShadow(cube, true));
        transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(cube, 100., 1., 100.));
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cube, 0., -1., 0.));
    }

    let (vertices, indices) = (BallBuilder::attrs_meta(), Some(BallBuilder::indices_meta()));
    let mut state: MeshInstanceState = MeshInstanceState::default();
    state.state = InstanceState::INSTANCE_BASE | InstanceState::INSTANCE_CUSTOM_VEC4_A | InstanceState::INSTANCE_CUSTOM_VEC4_B;
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut meshcmds, &mut geometrycmd, &mut transformcmds, vertices, indices, state);

    matcmds.usemat.push(OpsMaterialUse::Use(source, lightingmat, DemoScene::PASS_OPAQUE));
    matcmds.usemat.push(OpsMaterialUse::Use(source, predepthmat, DemoScene::PASS_PRE_DEPTH));

    meshcmds.shadow.push(OpsMeshShadow::CastShadow(source, true));
    lights.iter().for_each(|light| {
        abstructmeshcmds.force_point_light.push(OpsMeshForcePointLighting::ops(source, *light, true));
    });

        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(cube, scene));
                    instancemeshcmds.create.push(OpsInstanceMeshCreation::ops(source, cube));
                    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cube, (i + 1) as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32));
                    transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(cube, 1.,  1., 1.));
                    instancemeshcmds.floats.push(OpsInstanceFloat::ops(cube, (i as f32) / (tes_size as f32 - 1.), EInstanceFloatType::F00));
                    instancemeshcmds.floats.push(OpsInstanceFloat::ops(cube, (j as f32) / (tes_size as f32 - 1.), EInstanceFloatType::F01));
                }
            }
        }

        let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
        animegroupcmd.global.record_group(cameraroot, id_group);
        animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, cameraroot, id_group));
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
                animegroupcmd.scene_ctxs.add_target_anime(scene, cameraroot, id_group.clone(), animation);
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
                animegroupcmd.scene_ctxs.add_target_anime(scene, lightroot, id_group.clone(), animation);
            }
        }
        animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), AnimationGroupParam::default(), 0., pi_animation::base::EFillMode::NONE);

        {
            let (targets, device, asset_samp, atlas_allocator) = (&mut assets.0, &assets.1, &assets.2, &assets.3);
            let depthtarget = targets.create(device, KeySampler::linear_repeat(), asset_samp, atlas_allocator, ColorFormat::R16Float, DepthStencilFormat::Depth32Float, 256, 256 );

            let depth_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(depth_renderer, String::from("PreDepth") + depth_renderer.to_bits().to_string().as_str(), camera01, DemoScene::PASS_PRE_DEPTH, false));
            renderercmds.modify.push(OpsRendererCommand::AutoClearColor(depth_renderer, true));
            renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(depth_renderer, true));
            renderercmds.modify.push(OpsRendererCommand::AutoClearStencil(depth_renderer, true));
            renderercmds.modify.push(OpsRendererCommand::DepthClear(depth_renderer, RenderDepthClear(1.)));
            renderercmds.modify.push(OpsRendererCommand::ColorClear(depth_renderer, RenderColorClear(0, 0, 0, 0)));
            
            renderercmds.connect.push(OpsRendererConnect::ops(shadow_renderer, depth_renderer, false));
            renderercmds.connect.push(OpsRendererConnect::ops(depth_renderer, demopass.opaque_renderer, false));
            renderercmds.target.push(OpsRendererTarget::Custom(depth_renderer, KeyCustomRenderTarget::Custom(depthtarget.unwrap())));

            renderercmds.connect.push(OpsRendererConnect::ops(shadow_renderer, demopass.opaque_renderer, true));

            let vertices = CubeBuilder::attrs_meta();
            let indices = Some(CubeBuilder::indices_meta());
            let state = MeshInstanceState::default();
            let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut meshcmds, &mut geometrycmd, &mut transformcmds, vertices, indices, state);
            transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(source, 100., 0.5, 100.));
            meshcmds.shadow.push(OpsMeshShadow::CastShadow(source, false));
            let mut blend = ModelBlend::default(); blend.combine();
            meshcmds.blend.push(OpsRenderBlend::Blend(source, blend));

            let distortiommat = commands.spawn_empty().id();
            matcmds.create.push(OpsMaterialCreate::ops(distortiommat, water::ShaderWater::KEY));
            matcmds.usemat.push(OpsMaterialUse::Use(source, distortiommat, DemoScene::PASS_TRANSPARENT));
            matcmds.texture.push(OpsUniformTexture::ops(distortiommat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockMainTexture::KEY_TEX), url: EKeyTexture::image("./assets/images/eff_uv_lf_002.png"), sample: KeySampler::linear_repeat(), ..Default::default() }));
            matcmds.vec2.push(OpsUniformVec2::ops(distortiommat, Atom::from(BlockMainTextureUVOffsetSpeed::KEY_PARAM), 100., 100.));
            matcmds.texturefromtarget.push(OpsUniformTextureFromRenderTarget::ops(distortiommat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockEmissiveTexture::KEY_TEX), ..Default::default() }, depthtarget.unwrap(), Atom::from(BlockEmissiveTexture::KEY_TILLOFF)));
        }
}


pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    app.add_plugins(
        pi_pbr::PluginPBR
    );
    app.add_plugins(
        (pbr_material::PluginPBRMaterial, distortion_material::PluginDistortionMaterial, predepth::PluginShaderPreDepth, water::PluginShaderWater)
    );

    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_systems(Startup, setup.after(base::setup_default_mat));
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    // app.run()
    loop { app.update(); }

}
