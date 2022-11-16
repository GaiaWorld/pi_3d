#![feature(box_into_inner)]

mod shell_node;
mod engine_shell;

use engine_shell::{create_engine, EnginShell};
use pi_async::rt::{
    AsyncRuntime,
};
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use std::{any::TypeId, sync::Arc, time::{Instant, Duration}};

use pi_3d::{plugin::Plugin, object::ObjectID,
    transforms::{command::{SingleTransformNodeCommandList, TransformNodeCommand}, interface::InterfaceTransformNode},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    meshes::cube::InterfaceCube,
    main_camera_render::interface::InterfaceMainCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask}
};
use pi_ecs::prelude::{ResMut, Setup};
use pi_ecs_macros::setup;
use pi_scene_math::Vector3;

#[derive(Debug, Default)]
pub struct SingleTestData {
    pub transforms: Vec<(ObjectID, f32, f32, f32)>,
}

pub struct SysTest;
#[setup]
impl SysTest {
    #[system]
    pub fn sys(
        mut list: ResMut<SingleTestData>,
        mut transform_commands: ResMut<SingleTransformNodeCommandList>,
    ) {
        list.transforms.iter_mut().for_each(|mut item| {
            item.1 = item.1 + 16.0;
            item.2 = item.2 + 16.0;
            item.3 = item.3 + 16.0;
            let x0 = item.1 % 4000.0 / 4000.0;
            let x = x0 * 3.1415926 * 2.;
            let y0 = item.2 % 4000.0 / 4000.0;
            let y = y0 * 3.1415926 * 2.;
            let z0 = item.3 % 4000.0 / 4000.0;
            let z = z0 * 3.1415926 * 2.;
            // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 3., 0., 0.)));
            // transform_commands.list.push(TransformNodeCommand::ModifyScaling(item.0, Vector3::new(x.cos() + 0.5, x.sin() + 0.5, x + 0.5)));
            transform_commands.list.push(TransformNodeCommand::ModifyRotation(item.0, Vector3::new(x, y, z)));
        });
    }
}

pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut pi_3d::engine::Engine,
        stages: &mut pi_3d::run_stage::RunStage,
    ) -> Result<(), pi_3d::plugin::ErrorPlugin> {

        SysTest::setup(&mut world, stages.command_stage());

        let tes_size = 100;

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        let node01 = engine.create_transform_node(scene01);
        engine.active_camera(camera01, true);
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        engine.free_camera_orth_size(camera01, tes_size as f32);

        let cube = engine.new_cube(scene01);
        engine.transform_position(cube, Vector3::new(0., 0., 0.));

        engine.layer_mask(camera01, LayerMask::default());
        engine.layer_mask(cube, LayerMask::default());
        engine.transform_parent(cube, node01);

        let mut testdata = SingleTestData::default();
        // testdata.transforms.push((node01, 0., 0., 0.));
        // testdata.transforms.push((camera01, 0., 0., 0.));

        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = engine.new_cube(scene01);
                    engine.transform_position(cube, Vector3::new(i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, k as f32 * 2. - (tes_size) as f32));
                    engine.transform_rotation_euler(cube, Vector3::new(i as f32 * 0.2, j as f32 * 0.2, k as f32 * 0.2));
                    engine.layer_mask(cube, LayerMask::default());
                    // testdata.transforms.push((cube, i as f32 * 100., j as f32 * 100., k as f32 * 100.));
                }
            }
        }

        world.insert_resource(testdata);

        Ok(())
    }
}


pub fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());

    let size = window.inner_size();

    let engine = create_engine(&window, 0.0, PluginTest);

    run_loop(engine);
    
    run_window_loop(window, event_loop);
}

fn run_loop(mut engine_sheel: EnginShell) {
    MULTI_MEDIA_RUNTIME
        .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
            // example.init(&mut engine, (size.width as usize, size.height as usize)).await;

            let mut pre_frame_time = Instant::now();
            loop {
                // 运行
                // example.render(&mut engine);
                engine_sheel.run().await;

                let time = Instant::now();
                // let _use_time = Instant::now() - pre_frame_time;
                let time1 = pre_frame_time.clone();

                if time > time1 {
                    let d = time - time1;
                    let delay = 2;
                    let duration = if d > Duration::from_millis(delay) {
                        Duration::from_millis(0)
                    } else {
                        Duration::from_millis(delay) - d
                    };
                    spin_sleep::sleep(duration);
                }
                pre_frame_time = time;
            }
        })
        .unwrap();
}

fn run_window_loop(window: Arc<winit::window::Window>, event_loop: winit::event_loop::EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::Resized(_size),
                ..
            } => {}
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {}
            winit::event::Event::WindowEvent {
                // 窗口 关闭，退出 循环
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("RenderExample::clean");
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
            _ => {}
        }
    });
}