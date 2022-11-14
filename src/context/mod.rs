#![feature(box_into_inner)]

use pi_graph::NGraph;
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::{
    components::view::{
        render_window::{RenderWindow, RenderWindows},
        target_alloc::ShareTargetView,
    },
    init_render,
    rhi::{options::RenderOptions, device::RenderDevice, dyn_uniform_buffer::{Uniform, Bind}, RenderQueue}, graph::graph::RenderGraph,
};
use pi_scene_math::Vector3;
use std::{any::TypeId, sync::Arc, time::Instant, mem::replace, borrow::BorrowMut};
use wgpu::PresentMode;

use crate::{
    engine::Engine,
    object::GameObject,bytes_write_to_memory, shaders::FragmentUniformBind,
    scene::interface::InterfaceScene,
    transforms::interface::InterfaceTransformNode,
    cameras::interface::InterfaceCamera,
    meshes::cube::InterfaceCube,
    main_camera_render::interface::InterfaceMainCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask},
    run_stage::RunStage,
    PluginBundleDefault,
    plugin::Plugin,
};
use pi_async::{
    prelude::{Mutex, WorkerRuntime},
    rt::{AsyncRuntime, AsyncRuntimeBuilder},
};
use pi_ecs::{
    entity::Id,
    prelude::{
        ArchetypeId, IntoSystem, Query, Res, ResMut, Setup, SingleDispatcher, StageBuilder, System,
        World, Dispatcher, ExecNode,
    },
};
use pi_share::{Share, ShareMutex, ShareRwLock};

use self::{shell_node::ScreenClearNode, frame_time::PluginFrameTime};

pub mod shell_node;
pub mod frame_time;

pub struct EnginShell {
    options: RenderOptions,
    pub win: Arc<winit::window::Window>,
    pub dispatcher: SingleDispatcher<WorkerRuntime>,
    pub world: World,
    pub rt: WorkerRuntime,
    pub engine: Option<Engine>,
    runstages: RunStage,
    stages: Vec<Arc<NGraph<usize, ExecNode>>>,
    render_stages: Vec<pi_render::RenderStage>,
}

impl EnginShell {
    async fn _run(&mut self) {
        self.engine.as_ref().unwrap().tick_run();
        self.dispatcher.run().await;
    }

    ///
    /// * example
    ///   ```Rust
    ///   RenderOptions {
    ///       backends: wgpu::Backends::VULKAN,
    ///       power_preference: wgpu::PowerPreference::HighPerformance,
    ///       ..Default::default()
    ///   }
    ///   ```
    pub fn new(
        options: RenderOptions,
        window: Arc<winit::window::Window>,
    ) -> Self {
        println!(">>>> new");

        env_logger::init();
        
        println!(">>>> new 0");
        let mut world = World::new();

        let runtime = AsyncRuntimeBuilder::default_worker_thread(None, None, None, None);

        let mut world = World::new();
        world.insert_resource(DispatchEnd::default());

        let mut stages = vec![];
        let mut first_stage = StageBuilder::new();
        PluginFrameTime.init(&mut world, &mut first_stage);
        stages.push(Arc::new(first_stage.build(&mut world)));

        let mut runstages = RunStage::default();
        let rt = runtime.clone();
        
        let mut result: Share<ShareRwLock<Option<EnginShell>>> = Share::new(ShareRwLock::new(None));
        
        let result1 = result.clone();

        let _ = runtime.spawn(runtime.alloc(), async move {
            println!(">>>> render_graphic");
            let render_stages = init_render(&mut world, options.clone(), window.clone(), rt.clone()).await;
            Self::window(&mut world, window.clone());
            let render_graphic = world.get_resource_mut::<RenderGraph>().unwrap();
            let clear_id =  render_graphic.add_node("Clear", ScreenClearNode { color: (0., 0., 0., 1.), depth: -1.  }).unwrap();

            *result1.write() = Some(
                Self {
                    options,
                    win: window,
                    dispatcher: SingleDispatcher::new(rt.clone()),
                    world: world,
                    rt,
                    engine: None,
                    runstages,
                    stages,
                    render_stages: vec![render_stages],
                }
            );
        });

        loop {
            if result.read().is_some() {
                match Share::try_unwrap(result) {
                    Ok(r) => {
                        let r = r.into_inner().unwrap();
                        return {
                            let boxed = Box::new(r);
                            *boxed
                        };
                    }
                    Err(r) => result = r,
                }
            }
        }
    }

    pub fn add_plugin<T: Plugin>(
        &mut self,
        mut plugin: T
    ) -> &mut Self {
        println!(">>>> add_plugin");
        if self.engine.is_none() {
            let size = self.win.inner_size();
            // 初始化 Engine stage
            // 注册原型
            self.world.new_archetype::<GameObject>().create();
            let node_archetype_id = self.world.archetypes().get_id_by_ident(TypeId::of::<GameObject>()).unwrap().clone();
            let archetype_id = self.world.clone().archetypes_mut().get_or_create_archetype::<GameObject>();
            // let mut engine = Engine::new(world.clone(), node_archetype_id);
            // let mut engine = Engine::new(node_archetype_id, Box::new( || { &self.world }));
            let mut engine = Engine::new(self.world.clone(), node_archetype_id);
            engine.init(0, 0, size.width, size.height);

            self.engine = Some(engine);
        }
        plugin.init(&mut self.world, self.engine.as_mut().unwrap(), &mut self.runstages);
        
        self
    }

    fn window(world: &mut World, win: Arc<winit::window::Window>) {
        // 创建 RenderWindow
        let render_window = RenderWindow::new(win, PresentMode::Mailbox);
        let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
        render_windows.insert(render_window);
    }

    pub fn ready(
        &mut self
    ) {
        println!(">>>> ready");
        for stage in self.runstages.drain() {
            self.stages.push(Arc::new(stage.build(&self.world)));
        }

        let render_stages = self.render_stages.pop().unwrap();
        self.stages.push(Arc::new(render_stages.prepare_stage.build(&self.world)));
        self.stages.push(Arc::new(render_stages.render_stage.build(&self.world)));
        
        let mut last_stage = StageBuilder::new();
        let last_run = move |end: Res<DispatchEnd>| {
            let mut l = end.0.lock();
            // println!("set end true, {:?}, {:p}", *l, &end.0);
            *l = true;
        };
        last_stage.add_node(IntoSystem::system(last_run, &mut self.world));
        self.stages.push(Arc::new(last_stage.build(&self.world)));
    
        let stages = replace(&mut self.stages, vec![]);
        let mut dispatcher = SingleDispatcher::new(self.rt.clone());
        dispatcher.init(stages, &self.world);
    }
}

pub fn run(mut shell: EnginShell) {
    println!(">>>> run");
    MULTI_MEDIA_RUNTIME
        .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
            loop {
                shell._run().await;
            }
        })
        .unwrap();
}

pub struct DispatchEnd(pub ShareMutex<bool>);

impl Default for DispatchEnd {
    fn default() -> Self {
        Self(ShareMutex::new(true))
    }
}

pub struct PreFrameTime(pub Arc<Mutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Default for PreFrameTime {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Instant::now())))
    }
}

// pub fn create_engine(win: &Arc<Window>, _r: f64) -> EnginShell {
//     let size = win.inner_size();
//     let runtime = AsyncRuntimeBuilder::default_worker_thread(None, None, None, None);

//     let mut world = World::new();

//     world.insert_resource(DispatchEnd::default());

//     let mut world1 = world.clone();
//     let win1 = win.clone();
//     let rt = runtime.clone();

//     let mut result: Share<ShareRwLock<Option<(Engine, SingleDispatcher<WorkerRuntime>)>>> =
//         Share::new(ShareRwLock::new(None));
//     let result1 = result.clone();

//     let _ = runtime.spawn(runtime.alloc(), async move {
//         // let world = &mut world1;
//         // let options = RenderOptions {
//         //     backends: wgpu::Backends::VULKAN,
//         //     power_preference: wgpu::PowerPreference::HighPerformance,
//         //     ..Default::default()
//         // };
        
// 		// // init_render
// 		// //   > insert_render_graph
// 		// //     > world.insert_resource(RenderGraph::new(w, device, queue)); // RenderGraph 在此处被创建

//         // init_data(world, win1);

//         // let mut stages = Vec::new();

//         // let mut first_stage = StageBuilder::new();
//         // PluginFrameTime::init(world, &mut first_stage);
//         // stages.push(Arc::new(first_stage.build(world)));

//         // // 初始化 Engine stage
//         // let mut engine = Engine::new(world);
//         // engine.init(0, 0, size.width, size.height);

//         // let mut runstages = RunStage::default();
//         // // 建立System运行队列
//         // PluginBundleDefault::init(&mut engine, &mut runstages);
//         // PluginTest::init(&mut engine, &mut runstages);

//         // for stage in runstages.drain() {
//         //     stages.push(Arc::new(stage.build(world)));
//         // }

//         // let render_stages = init_render(world, options, win1.clone(), rt.clone()).await;

//         // let render_graphic = world.get_resource_mut::<RenderGraph>().unwrap();
//         // let clear_id =  render_graphic.add_node("Clear", ScreenClearNode { color: (0., 0., 0., 1.), depth: -1.  }).unwrap();
//         // let device = world.get_resource::<RenderDevice>().unwrap();
//         // // stages.push(Arc::new(render_stages.extract_stage.build(world)));
//         // stages.push(Arc::new(render_stages.prepare_stage.build(world)));
//         // stages.push(Arc::new(render_stages.render_stage.build(world)));

//         // let mut last_stage = StageBuilder::new();

//         // let last_run = move |end: Res<DispatchEnd>| {
//         //     let mut l = end.0.lock();
//         //     // println!("set end true, {:?}, {:p}", *l, &end.0);
//         //     *l = true;
//         // };
//         // last_stage.add_node(IntoSystem::system(last_run, world));
//         // stages.push(Arc::new(last_stage.build(world)));

//         // let mut dispatcher = SingleDispatcher::new(rt);
//         // dispatcher.init(stages, world);

//         // *result1.write() = Some((engine, dispatcher));

//     });
//     loop {
//         // if result.read().is_some() {
//         //     match Share::try_unwrap(result) {
//         //         Ok(r) => {
//         //             let r = r.into_inner().unwrap();
//         //             let engine = Box::new(EnginShell {
//         //                 win: win.clone(),
//         //                 dispatcher: r.1,
//         //                 world: World::new(),
//         //                 engine: r.0,
//         //                 rt: runtime.clone(),
//         //             });
//         //             return Box::into_inner(engine);
//         //         }
//         //         Err(r) => result = r,
//         //     }
//         // }
//     }
// }

// pub fn run_engine(engine: &EnginShell) {
    
// }

// fn init_data(world: &mut World, win: Arc<winit::window::Window>) {
//     // 创建 RenderWindow
//     let render_window = RenderWindow::new(win, PresentMode::Mailbox);
//     let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
//     render_windows.insert(render_window);
// }