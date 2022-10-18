use pi_ecs::{prelude::{StageBuilder, Setup}, world::World};

use self::{transform_node_sys::{LocalRotationMatrixCacl, LocalMatrixCacl, WorldMatrixCacl}, camera_sys::{TargetCameraEffectLocalRotation, TargetCameraViewMatrixCacl, CameraTransformMatricCacl, FreeCameraProjectionCacl}};

pub mod scene_sys;
pub mod camera_sys;
pub mod obj_sys;
pub mod tree_sys;
pub mod transform_node_sys;
pub mod pipeline_sys;
pub mod default_material_sys;
pub mod uniform_scene_sys;
pub mod uniform_camera_sys;
pub mod attribute_position_sys;

pub fn init_stage(world: &mut World) -> Vec<StageBuilder>  {
    let mut stages = Vec::new();

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

    stages
}