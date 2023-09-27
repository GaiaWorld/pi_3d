#![feature(box_into_inner)]


use pi_engine_shell::prelude::*;
use pi_scene_context::{prelude::*, viewer::prelude::{ViewerGlobalPosition, ViewerViewMatrix}, light::PluginLighting};
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;


#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginLighting);
    }
}

    fn setup(
        mut commands: Commands,
        mut scenecmds: ActionSetScene,
        mut cameracmds: ActionSetCamera,
        mut transformcmds: ActionSetTransform,
        mut lightingcmds: ActionSetLighting,
        mut meshcmds: ActionSetMesh,
        mut geometrycmd: ActionSetGeometry,
        mut matcmds: ActionSetMaterial,
        defaultmat: Res<SingleIDBaseDefaultMaterial>,
        mut animegroupcmd: ActionSetAnimationGroup,
        mut fps: ResMut<SingleFrameTimeCommand>,
        mut final_render: ResMut<WindowRenderer>,
        mut renderercmds: ActionSetRenderer,
    ) {

        let tes_size = 12;
        fps.frame_ms = 100;

        // Test Code
        let (scene, camera01, id_renderer) = base::DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 10., -40.), true);
        cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));
        cameracmds.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

        let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, scene));
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(light, 0., 10., -10.));
        meshcmds.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        lightingcmds.create.push(OpsLightCreate::ops(scene, light));
        lightingcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(0., -1., 1.)));
        lightingcmds.param.push(ELightModifyCommand::ShadowEnable(light, true));
        lightingcmds.param.push(ELightModifyCommand::ShadowFrustumSize(light, 40.0));

        renderercmds.connect.push(OpsRendererConnect::ops(final_render.clear_entity, light));
        renderercmds.connect.push(OpsRendererConnect::ops(light, id_renderer));
        renderercmds.modify.push(OpsRendererCommand::DepthFormat(light, RenderDepthFormat(DepthStencilFormat::Depth32Float)));
        renderercmds.modify.push(OpsRendererCommand::ColorFormat(light, RenderColorFormat(ColorFormat::Rgba16Float)));
        renderercmds.modify.push(OpsRendererCommand::AutoClearColor(light, true));
        renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(light, true));
        renderercmds.modify.push(OpsRendererCommand::DepthClear(light, RenderDepthClear(0.)));
        renderercmds.modify.push(OpsRendererCommand::ColorClear(light, RenderColorClear(0, 0, 0, 0)));

        log::warn!("Light: {:?}", light);

        // let camera01 = engine.create_free_camera(scene01);
        // engine.free_camera_mode(camera01, EFreeCameraMode::Perspective);
        // engine.active_camera(camera01, true);
        // engine.layer_mask(camera01, LayerMask::default());
        // engine.transform_position(camera01, Vector3::new(0., 10., -40.));
        // // engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("TestLight")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque]) });
        // // engine.transform_parent(camera01, root);
        // engine.camera_target(camera01, Vector3::new(0., -1., 4.));

        // let source = engine.create_mesh(scene01);
        // let mut attrs = CubeBuilder::attrs_meta();
        // attrs.push(VertexBufferDesc::instance_world_matrix());
        // engine.use_geometry(source, attrs, Some(CubeBuilder::indices_meta()));
        // // engine.use_default_material(source);
        // engine.layer_mask(source, LayerMask::default());

        // let key_group = pi_atom::Atom::from("key_group");
        // engine.create_animation_group(source, &key_group);

    let attrs = CubeBuilder::attrs_meta();

        let cell_col = 4.;
        let cell_row = 4.;
        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(cube, scene));
                    meshcmds.create.push(OpsMeshCreation::ops(scene, cube, MeshInstanceState::default()));

                    let id_geo = commands.spawn_empty().id();
                    geometrycmd.create.push(OpsGeomeryCreate::ops(cube, id_geo, attrs.clone(), Some(CubeBuilder::indices_meta())));
                    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cube, (i + 1) as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32));
                    
                    matcmds.usemat.push(OpsMaterialUse::Use(cube, defaultmat.0));
                    // let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                    // let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(1., 1., 1.)), LocalScaling(Vector3::new(0., 2. * (1.1 + (i as f32).sin()), 0.)), (60. * (1.1 + ((i * j) as f32).cos())) as u16, 30, EEasingMode::None);
                    // let asset_curve = if let Some(curve) = engine.check_anim_curve::<LocalScaling>(&key_curve0) {
                    //     curve
                    // } else {
                    //     engine.creat_anim_curve::<LocalScaling>(&key_curve0, curve)
                    // };
                    // let animation = engine.create_animation::<LocalScaling>(asset_curve);


                    // engine.create_target_animation(source, cube, &key_group, animation);
                }
            }
        }

        // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
            
        // 创建帧事件
    }


#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins_with_gltf();

    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    // app.run()
    loop { app.update(); }

}