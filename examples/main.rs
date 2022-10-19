#![feature(box_into_inner)]

mod context;
use context::{create_engine, EnginShell};
use pi_3d::{
    cameras::{
        camera::{Camera, CameraParam, CameraRenderData},
        free_camera::FreeCameraParam,
        target_camera::TargetCameraParam,
    },
    engine::Engine,
    flags::{SceneCameraID01, SceneID01},
    materials::default_material::{DefaultMaterialMeta, DefaultMaterialPropertype},
    object::GameObject,
    scene::SceneParam,
    systems::init_stage,
    transforms::transform_node::{GlobalTransform, LocalTransform, TransformDirty, TransformNode},
};
use pi_async::rt::{
    multi_thread::{MultiTaskRuntimeBuilder, StealableTaskPool},
    AsyncRuntime,
};
use pi_ecs::prelude::{Dispatcher, IntoSystem, Query, SingleDispatcher, StageBuilder, World};
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Vector3};
use std::{any::TypeId, sync::Arc, time::{Instant, Duration}};

pub fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());

    let size = window.inner_size();

    let engine = create_engine(&window, 0.0);

    run_loop(engine);
    
    run_window_loop(window, event_loop);
}

fn run_loop(engine: EnginShell) {
    MULTI_MEDIA_RUNTIME
        .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
            // example.init(&mut engine, (size.width as usize, size.height as usize)).await;

            let mut pre_frame_time = Instant::now();
            loop {
                // 运行
                // example.render(&mut engine);

                engine.dispatcher.run().await;

                let time = Instant::now();
                // let _use_time = Instant::now() - pre_frame_time;
                let time1 = pre_frame_time.clone();

                if time > time1 {
                    let d = time - time1;
                    let duration = if d > Duration::from_millis(16) {
                        Duration::from_millis(0)
                    } else {
                        Duration::from_millis(16) - d
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

pub fn main1() {
    env_logger::init();

    let mut world = World::new();
    // let mut demo = Demo::new(&mut world);

    let mut dynbuffer = DynUniformBuffer::new(Some("DynamicBindBUffer".to_string()), 16);

    let mut engine = Engine::new(&mut world);
    let stage_builders = engine.init(0, 0, 100, 100);

    // 创建一个运行时
    let pool = MultiTaskRuntimeBuilder::<(), StealableTaskPool<()>>::default();
    // 创建一个运行时
    let rt = pool.build();

    // 创建派发器
    let mut dispatcher = SingleDispatcher::new(rt.clone());
    let mut stages = Vec::new();
    stage_builders.into_iter().for_each(|stage| {
        stages.push(Arc::new(stage.build(&world)));
    });
    dispatcher.init(stages, &world);

    let scene01 = engine.new_scene();
    let node01 = engine.new_transform_node(scene01);
    let camera01 = engine.new_free_camera(scene01);

    println!("Run:");
    // 运行派发器，通常每帧推动
    dispatcher.run();

    world.insert_resource(dynbuffer);

    rt.spawn(rt.alloc(), async move {
        engine.tick_run();
        dispatcher.run().await;
    });
    loop {}
}
