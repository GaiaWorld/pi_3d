use std::{sync::Arc, any::TypeId};

use pi_3d::{object::GameObject, flags::{SceneID01, SceneCameraID01}, scene::SceneParam, cameras::{camera::{Camera, CameraParam, CameraRenderData}, target_camera::TargetCameraParam, free_camera::FreeCameraParam}, transforms::transform_node::{TransformNode, LocalTransform, GlobalTransform, TransformDirty}, systems::init_stage};
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

    world.new_archetype::<GameObject>().create(); // 创建Node原型
    
    let node_archetype_id = world.archetypes().get_id_by_ident(TypeId::of::<GameObject>()).unwrap().clone();

    let stage_builders = init_stage(&mut world);
    
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

    // 
    let mut scene01 = world.spawn::<GameObject>();
    scene01.insert(SceneID01);
    scene01.insert(SceneParam { coordsys: CoordinateSytem3::left() });

    let mut node01 = world.spawn::<GameObject>();
    node01.insert(SceneID01);
    node01.insert(TransformNode);
    let mut lt = LocalTransform::default();
    lt.position = Vector3::new(1., 10., 100.);
    lt.euler = Vector3::new(0., (90.0f32).to_radians(), 0.);
    lt.scaling = Vector3::new(10., 10., 10.);
    node01.insert(lt);
    node01.insert(GlobalTransform::default());
    node01.insert(TransformDirty::default());

    let mut camera01 = world.spawn::<GameObject>();
    camera01.insert(SceneID01);
    camera01.insert(TransformNode);
    camera01.insert(TransformDirty::default());
    camera01.insert(LocalTransform::default());
    camera01.insert(GlobalTransform::default());
    camera01.insert(SceneCameraID01);
    camera01.insert(CameraParam::default());
    camera01.insert(TargetCameraParam::default());
    camera01.insert(FreeCameraParam::default());
    camera01.insert(CameraRenderData::new(&mut dynbuffer));

    println!("Run:");
	// 运行派发器，通常每帧推动
	dispatcher.run();

    world.insert_resource(dynbuffer);

    rt.spawn(rt.alloc(), async move {
        dispatcher.run().await;
    });
    loop {}
}