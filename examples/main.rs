#![feature(box_into_inner)]

use pi_3d::{PluginBundleDefault, context::{EnginShell, run}};
use pi_async::rt::{
    AsyncRuntime,
};
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::rhi::options::RenderOptions;
use std::{any::TypeId, sync::Arc, time::{Instant, Duration}};

pub fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());

    let mut shell = EnginShell::new(
        RenderOptions {
            backends: wgpu::Backends::VULKAN,
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        },
        window.clone()
    );

    shell.add_plugin(PluginBundleDefault);

    shell.ready();

    run(shell);
    

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


// pub fn main1() {
//     env_logger::init();

//     let mut world = World::new();
//     // let mut demo = Demo::new(&mut world);

//     let mut dynbuffer = DynUniformBuffer::new(Some("DynamicBindBUffer".to_string()), 16);

//     let mut engine = Engine::new(&mut world);
//     let stage_builders = engine.init(0, 0, 100, 100);

//     // 创建一个运行时
//     let pool = MultiTaskRuntimeBuilder::<(), StealableTaskPool<()>>::default();
//     // 创建一个运行时
//     let rt = pool.build();

//     // 创建派发器
//     let mut dispatcher = SingleDispatcher::new(rt.clone());
//     let mut stages = Vec::new();
//     stage_builders.into_iter().for_each(|stage| {
//         stages.push(Arc::new(stage.build(&world)));
//     });
//     dispatcher.init(stages, &world);

//     let scene01 = engine.new_scene();
//     let node01 = engine.new_transform_node(scene01);
//     let camera01 = engine.new_free_camera(scene01);
//     engine.set_parent(camera01, scene01, Some(node01));

//     println!("Run:");
//     // 运行派发器，通常每帧推动
//     dispatcher.run();

//     world.insert_resource(dynbuffer);

//     rt.spawn(rt.alloc(), async move {
//         engine.tick_run();
//         dispatcher.run().await;
//     });
//     loop {}
// }
