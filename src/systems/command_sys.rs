use pi_ecs::{prelude::{ResMut, EntityDelete, Query}, storage::Null, query::Write};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::{EntityTree, EntityTreeMut};
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;
use pi_scene_math::coordiante_system::CoordinateSytem3;
use pi_slotmap_tree::Storage;

use crate::{resources::command::{UserCommands, TransformNodeTreeCommand, ObjectNewCommand}, object::{ObjectID, GameObject}, scene::SceneParam, transforms::transform_node::{TransformNode, GlobalTransform, LocalTransform, TransformDirty}, meshes::Mesh, cameras::{camera::{Camera, CameraRenderData, CameraParam}, target_camera::TargetCameraParam, free_camera::FreeCameraParam}};


pub struct UserCommandTick;
#[setup]
impl UserCommandTick {
    #[system]
    pub fn tick(
        mut command: ResMut<UserCommands>,
        entitys: Query<'static, 'static, GameObject, ObjectID>,
        mut tree: EntityTreeMut<GameObject>,
        mut entity_delete: EntityDelete<GameObject>,
        scene_query: Query<GameObject, Write<SceneParam>>,
        transform_query: Query<GameObject, (Write<TransformNode>, Write<LocalTransform>, Write<GlobalTransform>, Write<TransformDirty>)>,
        camera_query: Query<GameObject, (Write<CameraParam>, Write<TargetCameraParam>, Write<FreeCameraParam>, Write<CameraRenderData>)>,
        mut dynbuffer:  ResMut<DynUniformBuffer>,
    ) {
        cmd_new_object(
            &mut command.new_objects,
            scene_query,
            transform_query,
            camera_query,
            &mut dynbuffer,
        );
        cmd_node_tree(&mut command.tree, &entitys, &mut tree, &mut entity_delete);
    }
}

fn cmd_new_object(
    cmds: &mut Vec<ObjectNewCommand>,
    mut scene_query: Query<GameObject, Write<SceneParam>>,
    mut transform_query: Query<GameObject, (Write<TransformNode>, Write<LocalTransform>, Write<GlobalTransform>, Write<TransformDirty>)>,
    mut camera_query: Query<GameObject, (Write<CameraParam>, Write<TargetCameraParam>, Write<FreeCameraParam>, Write<CameraRenderData>)>,
    dynbuffer: &mut DynUniformBuffer,
) {
    println!("cmd_new_object");
    cmds.drain(..).for_each(|cmd| {
        match cmd {
            ObjectNewCommand::NewScene(id) => {
                match scene_query.get_mut(id) {
                    Some(mut write) => {
                        write.insert_no_notify(SceneParam { coordsys: CoordinateSytem3::left() });
                    },
                    None => {},
                }
            },
            ObjectNewCommand::NewTransformNode(id) => {
                match transform_query.get_mut(id) {
                    Some((mut transformnode, mut local, mut world, mut dirty)) => {
                        transformnode.insert_no_notify(TransformNode);
                        local.insert_no_notify(LocalTransform::default());
                        world.insert_no_notify(GlobalTransform::default());
                        dirty.insert_no_notify(TransformDirty::default());
                    },
                    None => {},
                }
            },
            ObjectNewCommand::NewMesh(_) => todo!(),
            ObjectNewCommand::NewFreeCamera(id) => {
                match camera_query.get_mut(id) {
                    Some((mut camera, mut target_camera, mut free_camera, mut camera_render)) => {
                        camera.insert_no_notify(CameraParam::default());
                        target_camera.insert_no_notify(TargetCameraParam::default());
                        free_camera.insert_no_notify(FreeCameraParam::default());
                        camera_render.insert_no_notify(CameraRenderData::new(dynbuffer));
                    },
                    None => {},
                }
            },
        }
    });
}

fn cmd_node_tree(
    cmds: &mut Vec<TransformNodeTreeCommand>,
    entitys: & Query<'static, 'static, GameObject, ObjectID>,
    tree: &mut EntityTreeMut<GameObject>,
    entity_delete: &mut EntityDelete<GameObject>,
) {
    println!("cmd_node_tree");
    cmds.drain(..).for_each(|cmd| {
        match cmd {
            crate::resources::command::TransformNodeTreeCommand::Append(child, parent) => {
                if entitys.get(child).is_some() {
                    tree.insert_child(child, parent, usize::MAX);
                }
            },
            crate::resources::command::TransformNodeTreeCommand::Remove(node) => {
                tree.remove(node);
            },
            crate::resources::command::TransformNodeTreeCommand::Destroy(node) => {
                tree.remove(node);

                entity_delete.despawn(node);

                // 删除所有子节点对应的实体
                if let Some(down) = tree.get_down(node) {
                    let head = down.head();
                    if !head.is_null() {
                        for node in tree.recursive_iter(head) {
                            entity_delete.despawn(node);
                        }
                    }
                }
            },
        }
    });
}