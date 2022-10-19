use std::{sync::Arc, any::TypeId};

use pi_3d::{object::GameObject, flags::{SceneID01, SceneCameraID01}, scene::SceneParam, cameras::{camera::{Camera, CameraParam, CameraRenderData}, target_camera::TargetCameraParam, free_camera::FreeCameraParam}, transforms::transform_node::{TransformNode, LocalTransform, GlobalTransform, TransformDirty}, systems::init_stage, materials::default_material::{DefaultMaterialMeta, DefaultMaterialPropertype}, engine::Engine};
use pi_async::rt::{multi_thread::{MultiTaskRuntimeBuilder, StealableTaskPool}, AsyncRuntime};
use pi_ecs::prelude::{World, StageBuilder, IntoSystem, SingleDispatcher, Dispatcher, Query};
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Vector3};

// mod context;

pub fn main() {
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