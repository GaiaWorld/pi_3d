#![feature(box_into_inner)]

use pi_3d::{PluginBundleDefault, context::{EnginShell, run}};
use pi_async::rt::{
    AsyncRuntime,
};
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::rhi::options::RenderOptions;
use std::{any::TypeId, sync::Arc, time::{Instant, Duration}};

mod plugin_test;
use plugin_test::PluginTest;


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
    shell.add_plugin(PluginTest);

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