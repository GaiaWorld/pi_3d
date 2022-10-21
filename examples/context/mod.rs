use pi_render::{
    components::view::{
        render_window::{RenderWindow, RenderWindows},
        target_alloc::ShareTargetView,
    },
    init_render,
    rhi::{options::RenderOptions, device::RenderDevice},
};
use std::{any::TypeId, sync::Arc, time::Instant};
use wgpu::PresentMode;
use winit::window::Window;

use pi_3d::{
    engine::Engine,
    object::GameObject,
    systems::{
        camera_sys::{
            CameraTransformMatricCacl, FreeCameraProjectionCacl, TargetCameraEffectLocalRotation,
            TargetCameraViewMatrixCacl,
        },
        transform_node_sys::{LocalMatrixCacl, LocalRotationMatrixCacl, WorldMatrixCacl},
    },
};
use pi_async::{
    prelude::{Mutex, WorkerRuntime},
    rt::{AsyncRuntime, AsyncRuntimeBuilder},
};
use pi_ecs::{
    entity::Id,
    prelude::{
        ArchetypeId, IntoSystem, Query, Res, ResMut, Setup, SingleDispatcher, StageBuilder, System,
        World,
    },
};
use pi_share::{Share, ShareMutex, ShareRwLock};

pub struct EnginShell {
    pub win: Arc<Window>,
    pub dispatcher: SingleDispatcher<WorkerRuntime>,
    pub world: World,
    pub rt: WorkerRuntime,
    pub engine: Engine,
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

pub fn create_engine(win: &Arc<Window>, _r: f64) -> EnginShell {
    let size = win.inner_size();
    let runtime = AsyncRuntimeBuilder::default_worker_thread(None, None, None, None);

    let mut world = World::new();

    world.insert_resource(DispatchEnd::default());

    let mut world1 = world.clone();
    let win1 = win.clone();
    let rt = runtime.clone();

    let mut result: Share<ShareRwLock<Option<(Engine, SingleDispatcher<WorkerRuntime>)>>> =
        Share::new(ShareRwLock::new(None));
    let result1 = result.clone();

    let _ = runtime.spawn(runtime.alloc(), async move {
        let world = &mut world1;
        let options = RenderOptions::default();
        
		// init_render
		//   > insert_render_graph
		//     > world.insert_resource(RenderGraph::new(w, device, queue)); // RenderGraph 在此处被创建
        let render_stages = init_render(world, options, win1.clone(), rt.clone()).await;

        let limits = world.get_resource::<RenderDevice>().unwrap().limits();
        let min_uniform_buffer_offset_alignment = limits.min_uniform_buffer_offset_alignment;
        world.insert_resource(pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer::new(
            Some("DynUniformBuffer".to_string()),
            min_uniform_buffer_offset_alignment.max(192),
        ));

        init_data(world, win1);

        let mut stages = Vec::new();

        let mut first_stage = StageBuilder::new();
        let first_run = move |mut frame_start_time: ResMut<FrameStartTime>| {
            frame_start_time.0 = Instant::now();
        };
        first_stage.add_node(IntoSystem::system(first_run, world));
        stages.push(Arc::new(first_stage.build(world)));

        // 初始化 Engine stage
        let mut engine = Engine::new(world);
        let gui_stages = engine.init(0, 0, size.width, size.height);
        for stage in gui_stages.into_iter() {
            stages.push(Arc::new(stage.build(world)));
        }
        // stages.push(Arc::new(render_stages.extract_stage.build(world)));
        // stages.push(Arc::new(render_stages.prepare_stage.build(world)));
        // stages.push(Arc::new(render_stages.render_stage.build(world)));

        let mut last_stage = StageBuilder::new();

        let last_run = move |end: Res<DispatchEnd>| {
            let mut l = end.0.lock();
            // println!("set end true, {:?}, {:p}", *l, &end.0);
            *l = true;
        };
        last_stage.add_node(IntoSystem::system(last_run, world));
        stages.push(Arc::new(last_stage.build(world)));

        let mut dispatcher = SingleDispatcher::new(rt);
        dispatcher.init(stages, world);


        // Test Code
        let scene01 = engine.new_scene();
        let node01 = engine.new_transform_node(scene01);
        let camera01 = engine.new_free_camera(scene01);
        engine.set_parent(camera01, scene01, Some(node01));


        *result1.write() = Some((engine, dispatcher));

    });
    loop {
        if result.read().is_some() {
            match Share::try_unwrap(result) {
                Ok(r) => {
                    let r = r.into_inner().unwrap();
                    let engine = Box::new(EnginShell {
                        win: win.clone(),
                        dispatcher: r.1,
                        world: World::new(),
                        engine: r.0,
                        rt: runtime.clone(),
                    });
                    return Box::into_inner(engine);
                }
                Err(r) => result = r,
            }
        }
    }
}

pub fn run_engine(engine: &EnginShell) {}

fn init_data(world: &mut World, win: Arc<winit::window::Window>) {
    // 创建 RenderWindow
    let render_window = RenderWindow::new(win, PresentMode::Mailbox);
    let render_windows = world.get_resource_mut::<RenderWindows>().unwrap();
    render_windows.insert(render_window);
}
