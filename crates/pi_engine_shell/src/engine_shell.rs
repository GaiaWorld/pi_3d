use std::{sync::Arc, any::TypeId, mem::replace};

use bevy::prelude::Resource;
use pi_async::{prelude::WorkerRuntime, rt::{AsyncRuntimeBuilder, AsyncRuntime}};
use pi_ecs::{prelude::{SingleDispatcher, ExecNode, StageBuilder, Res, IntoSystem, Dispatcher}, world::World, storage::Local};
use pi_graph::NGraph;
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::{rhi::options::RenderOptions, RenderStage, init_render, graph::graph::RenderGraph, components::view::render_window::{RenderWindow, RenderWindows}};
use pi_share::{Share, ShareRwLock};
use winit::event_loop::EventLoop;

use crate::{frame_time::PluginFrameTime, shell_node::ScreenClearNode, object::{GameObject, ObjectID, PluginObject}, DispatchEnd, run_stage::RunStage, plugin::Plugin, setup::{TSetup, SetupFn}};

// pub use bevy::app::App;


pub struct AppShell {
    event_loop: Option<EventLoop<()>>,
    window: Arc<winit::window::Window>,
    runstages: RunStage,
    render_stages: Option<pi_render::RenderStage>,
    engine: Option<EnginShell>,
}

impl AppShell {
    pub fn new(
        options: RenderOptions,
    ) -> Self {

        let event_loop = winit::event_loop::EventLoop::new();
        let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());

        let runtime = AsyncRuntimeBuilder::default_worker_thread(None, None, None, None);

        let mut world = World::new();
        world.insert_resource(crate::DispatchEnd::default());
        
        world.insert_resource(window.clone());
        
        let mut result: Share<ShareRwLock<Option<(World, RenderStage, Arc<winit::window::Window>, RenderOptions)>>> = Share::new(ShareRwLock::new(None));
        
        let result1 = result.clone();
        let rt = runtime.clone();

        let _ = runtime.spawn(runtime.alloc(), async move {
            log::debug!(">>>> render_graphic");
            let render_stages = init_render(&mut world, options.clone(), window.clone(), rt.clone()).await;

            *result1.write() = Some(
                (
                    world,
                    render_stages,
                    window,
                    options,
                )
            );
        });

        loop {
            if result.read().is_some() {
                match Share::try_unwrap(result) {
                    Ok(r) => {
                        let (world, render_stages, window, options) = r.into_inner().unwrap();

                        return {
                            let boxed = Box::new(
                                Self::_new(options, window, event_loop, world, render_stages, runtime)
                            );
                            *boxed
                        };
                    }
                    Err(r) => result = r,
                }
            }
        }
    }

    fn _new(
        options: RenderOptions,
        window: Arc<winit::window::Window>,
        event_loop: EventLoop<()>,
        mut world: World,
        render_stages: pi_render::RenderStage,
        runtime: WorkerRuntime,
    ) -> Self {

        let runstages = RunStage::new();
    
        Self::window(&mut world, window.clone());
        let render_graphic = world.get_resource_mut::<RenderGraph>().unwrap();
        render_graphic.add_node("Clear", ScreenClearNode { color: (0., 0., 0., 1.), depth: -1.  }).unwrap();

        Self {
            window,
            event_loop: Some(event_loop),
            runstages,
            render_stages: Some(render_stages),
            engine: Some(EnginShell::new(options, world, runtime)),
        }
    }

    pub fn add_plugin<T: Plugin>(
        &mut self,
        mut plugin: T
    ) -> &mut Self {
        log::debug!(">>>> add_plugin");
        plugin.init(&mut self.engine.as_mut().unwrap(), &mut self.runstages);
        
        self
    }

    fn window(world: &mut World, win: Arc<winit::window::Window>) {
        // 创建 RenderWindow
        let render_window = RenderWindow::new(win, wgpu::PresentMode::Mailbox);
        let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
        render_windows.insert(render_window);
    }

    pub fn ready(
        &mut self,
    ) -> &mut Self {
        let render_stage = replace(&mut self.render_stages, None).unwrap();
        self.engine.as_mut().unwrap().ready(
            &mut self.runstages,
            render_stage
        );

        self
    }

    pub fn setup(
        &mut self,
        setup: &SetupFn,
    ) -> &mut Self {
        setup(self.engine.as_ref().unwrap());
        self
    }

    pub fn run(&mut self) {
        log::debug!(">>>> run");

        let event_loop = replace(&mut self.event_loop, None).unwrap();
        let window = self.window.clone();
        let mut engine = replace(&mut self.engine, None).unwrap();

        MULTI_MEDIA_RUNTIME
            .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                loop {
                    log::debug!("====== Run ================");
                    engine._run().await;
                }
            })
            .unwrap();
           
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
}

pub struct EnginShell {
    options: RenderOptions,
    world: World,
    node_archetype_id: Local,
    single_dispatcher: SingleDispatcher<WorkerRuntime>,
    resource_type_ids: Vec<Local>,
}

impl EnginShell {
    pub fn new(
        options: RenderOptions,
        mut world: World,
        runtime: WorkerRuntime,
    ) -> Self {
        // 注册原型
        world.new_archetype::<GameObject>().create();
        let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<GameObject>()).unwrap().clone();
        let archetype_id = world.clone().archetypes_mut().get_or_create_archetype::<GameObject>();

        Self {
            options,
            world,
            node_archetype_id,
            single_dispatcher: SingleDispatcher::new(runtime),
            resource_type_ids: vec![],
        }
    }
    async fn _run(&mut self) {
        // self.engine.as_ref().unwrap().tick_run();
        
        unsafe { 
            let world: &mut World = &mut self.world;
            //  log::debug!("Engine Tick Run: >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            let node_archetype_id = self.node_archetype_id;
            world.archetypes_mut()[node_archetype_id].flush();
        }

        self.single_dispatcher.run().await;
    }

    pub fn ready(&mut self, runstages: &mut RunStage, render_stages: RenderStage) {
        let mut stages = vec![];
        let mut first_stage = StageBuilder::new();
        PluginFrameTime.init(&mut self.world, &mut first_stage);
        PluginObject.init(&mut self.world, &mut first_stage);
        stages.push(Arc::new(first_stage.build(&self.world)));

        runstages.log();

        for stage in runstages.drain() {
            stages.push(Arc::new(stage.build(&self.world)));
        }

        stages.push(Arc::new(render_stages.prepare_stage.build(&self.world)));
        stages.push(Arc::new(render_stages.render_stage.build(&self.world)));
        
        let mut last_stage = StageBuilder::new();
        let last_run = move |end: Res<DispatchEnd>| {
            let mut l = end.0.lock();
            // log::debug!("set end true, {:?}, {:p}", *l, &end.0);
            *l = true;
        };
        last_stage.add_node(IntoSystem::system(last_run, &mut self.world));
        stages.push(Arc::new(last_stage.build(&self.world)));

        self.single_dispatcher.init(stages, &self.world);

    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn node_archetype_id(&self) -> Local {
        self.node_archetype_id
    }
}