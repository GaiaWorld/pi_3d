use pi_ecs::{prelude::{StageBuilder, Setup}, world::World};

use crate::default_render::{default_material_sys::DefaultMaterialUniformTickUpdate, world_init_default_materail};

use self::{transform_node_sys::{LocalRotationMatrixCacl, LocalMatrixCacl, WorldMatrixCacl}, camera_sys::{TargetCameraEffectLocalRotation, TargetCameraViewMatrixCacl, CameraTransformMatricCacl, FreeCameraProjectionCacl}, uniform_scene_sys::SceneUniformTickUpdate, uniform_camera_sys::CameraUniformTickUpdate, command_sys::UserCommandTick};

pub mod scene_sys;
pub mod camera_sys;
pub mod obj_sys;
pub mod command_sys;
pub mod transform_node_sys;
pub mod pipeline_sys;
pub mod uniform_scene_sys;
pub mod uniform_camera_sys;
pub mod attribute_position_sys;

pub fn init_stage(world: &mut World) -> Vec<StageBuilder>  {
    let mut stages = Vec::new();

    // UserCommand 阶段
    let mut command_stage = StageBuilder::new();
    UserCommandTick::setup(world, &mut command_stage);

    stages.push(command_stage);

    // 节点属性计算阶段
    let mut transform_node_stage = StageBuilder::new();

    let local_rotation_cacl = LocalRotationMatrixCacl::setup(world, &mut transform_node_stage).unwrap();
    let local_matrix_cacl = LocalMatrixCacl::setup(world, &mut transform_node_stage).unwrap();
    let world_matrix_cacl = WorldMatrixCacl::setup(world, &mut transform_node_stage).unwrap();
    let camera_control_target = TargetCameraEffectLocalRotation::setup(world, &mut transform_node_stage).unwrap();
    let camera_view_target = TargetCameraViewMatrixCacl::setup(world, &mut transform_node_stage).unwrap();
    let camera_project_free = FreeCameraProjectionCacl::setup(world, &mut transform_node_stage).unwrap();
    let camera_transform = CameraTransformMatricCacl::setup(world, &mut transform_node_stage).unwrap();
    
    transform_node_stage = transform_node_stage.order(local_rotation_cacl, camera_control_target)
                                                .order(camera_control_target, local_matrix_cacl);
    //                                             .order(world_matrix_cacl, camera_view_target)
    //                                             .order(camera_view_target, camera_project_free)
    //                                             .order(camera_project_free, camera_transform);

    stages.push(transform_node_stage);

    // Buildin Uniform 处理阶段
    let mut buildin_uniform_stage = StageBuilder::new();
    SceneUniformTickUpdate::setup(world, &mut buildin_uniform_stage);
    CameraUniformTickUpdate::setup(world, &mut buildin_uniform_stage);
    
    stages.push(buildin_uniform_stage);
    
    let mut stage_uniform_update = StageBuilder::new();
    let mut stage_before_render = StageBuilder::new();

    world_init_default_materail(world, &mut stage_uniform_update, &mut stage_before_render);
    
    stages.push(stage_uniform_update);
    stages.push(stage_before_render);

    stages
}