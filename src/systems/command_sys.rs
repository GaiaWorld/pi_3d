use pi_ecs::{prelude::{ResMut, EntityDelete, Query, Res, Entities}, storage::Null, query::{Write, WithOut, With}};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::{EntityTree, EntityTreeMut};
use pi_render::rhi::{dyn_uniform_buffer::DynUniformBuffer, device::RenderDevice, RenderQueue};
use pi_scene_math::coordiante_system::CoordinateSytem3;
use pi_slotmap_tree::Storage;

use crate::{resources::{command::{UserCommands, TransformNodeTreeCommand, ObjectCommand, MeshBuilderCommand, CameraCommand}, SingleGeometryBufferPool}, object::{ObjectID, GameObject}, scene::SceneParam, transforms::transform_node::{TransformNode, GlobalTransform, LocalTransform, TransformDirty}, meshes::{Mesh, cube::CubeBuilder}, cameras::{camera::{Camera, CameraRenderData, CameraParam}, target_camera::TargetCameraParam, free_camera::FreeCameraParam}, flags::{RenderLayerMask, RenderSortParam, RenderBlend, PrimitiveState, RenderDepthAndStencil, SceneID}, shaders::buildin_attributes::{BuildinAttributePosition, BuildinAttributeNormal, BuildinAttributeIndices}, default_render::default_material::{DefaultMaterialPropertype, DefaultMaterialMeta}, renderers::{render_object::{RenderObjectID, RenderObjectOpaqueList}, main_camera::MainCameraBindGroup}};


pub struct UserCommandTick;
#[setup]
impl UserCommandTick {
    #[system]
    pub fn tick(
        mut command: ResMut<UserCommands>,
        mut tree: EntityTreeMut<GameObject>,
        mut entity_delete: EntityDelete<GameObject>,
        mut entitys: Query<GameObject, ObjectID>,
        mut scene_query: Query<GameObject, Write<SceneParam>>,
        mut transform_query: Query<GameObject, (Write<SceneID>, Write<TransformNode>, Write<LocalTransform>, Write<GlobalTransform>, Write<TransformDirty>)>,
        mut camera_query: Query<GameObject, (Write<SceneID>, Write<CameraParam>, Write<TargetCameraParam>, Write<FreeCameraParam>, Write<CameraRenderData>, Write<RenderLayerMask>)>,
        mut mesh_query: Query<GameObject, (Write<SceneID>, Write<RenderLayerMask>, Write<RenderSortParam>, Write<RenderBlend>, Write<PrimitiveState>, Write<RenderDepthAndStencil>)>,
        mut mesh_builders: Query<GameObject, (Write<BuildinAttributePosition>, Write<BuildinAttributeNormal>, Write<BuildinAttributeIndices>, Write<DefaultMaterialMeta>)>,
        mut camera_render_without: Query<GameObject, (Write<RenderObjectID>, Write<MainCameraBindGroup>)>,
        mut render_entities: Query<GameObject, (Write<RenderObjectOpaqueList>)>,
        mut dynbuffer:  ResMut<DynUniformBuffer>,
        device: Res<RenderDevice>,
        mut queue: ResMut<RenderQueue>,
        mut gbp: ResMut<SingleGeometryBufferPool>,
    ) {
        cmd_object(
            &mut command.objects,
            &mut entity_delete,
            &mut tree,
            &mut scene_query,
            &mut transform_query,
            &mut camera_query,
            &mut mesh_query,
            &mut dynbuffer,
        );
        cmd_node_tree(&mut command.tree, &entitys, &mut tree);
        cmd_mesh_builder(&mut command.mesh_builder, &mut mesh_builders, &device, &mut queue, &mut gbp, &mut dynbuffer);
        cmd_camera_render(&mut command.cameras, &mut camera_render_without, &mut render_entities, &device, &mut dynbuffer);
    }
}

fn cmd_object(
    cmds: &mut Vec<ObjectCommand>,
    entity_delete: &mut EntityDelete<GameObject>,
    tree: &mut EntityTreeMut<GameObject>,
    scene_query: &mut Query<GameObject, Write<SceneParam>>,
    transform_query: &mut Query<GameObject, (Write<SceneID>, Write<TransformNode>, Write<LocalTransform>, Write<GlobalTransform>, Write<TransformDirty>)>,
    camera_query: &mut Query<GameObject, (Write<SceneID>, Write<CameraParam>, Write<TargetCameraParam>, Write<FreeCameraParam>, Write<CameraRenderData>, Write<RenderLayerMask>)>,
    mesh_query: &mut Query<GameObject, (Write<SceneID>, Write<RenderLayerMask>, Write<RenderSortParam>, Write<RenderBlend>, Write<PrimitiveState>, Write<RenderDepthAndStencil>)>,
    dynbuffer: &mut DynUniformBuffer,
) {
    println!("cmd_new_object");
    cmds.drain(..).for_each(|cmd| {
        match cmd {
            ObjectCommand::NewScene(id) => {
                match scene_query.get_mut(id) {
                    Some(mut write) => {
                        println!("new Scene {:?}", id);
                        write.insert_no_notify(SceneParam { coordsys: CoordinateSytem3::left() });
                    },
                    None => {},
                }
            },
            ObjectCommand::NewTransformNode(id, scene_id) => {
                match transform_query.get_mut(id) {
                    Some(mut item) => {
                        item.0.insert_no_notify(SceneID(scene_id));
                        item.1.insert_no_notify(TransformNode);
                        item.2.insert_no_notify(LocalTransform::default());
                        item.3.insert_no_notify(GlobalTransform::default());
                        item.4.insert_no_notify(TransformDirty::default());
                    },
                    None => {},
                }
            },
            ObjectCommand::NewMesh(id, scene_id) => {
                match mesh_query.get_mut(id) {
                    Some(mut item) => {
                        item.0.insert_no_notify(SceneID(scene_id));
                        item.1.insert_no_notify(RenderLayerMask::default());
                        item.2.insert_no_notify(RenderSortParam::opaque());
                        item.3.insert_no_notify(RenderBlend::default());
                        item.4.insert_no_notify(PrimitiveState::default());
                        item.5.insert_no_notify(RenderDepthAndStencil::default());
                    },
                    None => {
                        
                    },
                }
            },
            ObjectCommand::NewFreeCamera(id, scene_id) => {
                match camera_query.get_mut(id) {
                    Some(mut item) => {
                        item.0.insert_no_notify(SceneID(scene_id));
                        item.1.insert_no_notify(CameraParam::default());
                        item.2.insert_no_notify(TargetCameraParam::default());
                        item.3.insert_no_notify(FreeCameraParam::default());
                        item.4.insert_no_notify(CameraRenderData::new(dynbuffer));
                        item.5.insert_no_notify(RenderLayerMask::default());
                    },
                    None => {},
                }
            },
            ObjectCommand::Destroy(id) => {
                tree.remove(id);

                entity_delete.despawn(id);

                // 删除所有子节点对应的实体
                if let Some(down) = tree.get_down(id) {
                    let head = down.head();
                    if !head.is_null() {
                        for id in tree.recursive_iter(head) {
                            entity_delete.despawn(id);
                        }
                    }
                }
            },
        }
    });
}

fn cmd_node_tree(
    cmds: &mut Vec<TransformNodeTreeCommand>,
    entitys: & Query<GameObject, ObjectID>,
    tree: &mut EntityTreeMut<GameObject>,
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
        }
    });
}

fn cmd_mesh_builder(
    cmds: &mut Vec<MeshBuilderCommand>,
    mesh_builders: &mut Query<GameObject, (Write<BuildinAttributePosition>, Write<BuildinAttributeNormal>, Write<BuildinAttributeIndices>, Write<DefaultMaterialMeta>)>,
    device: &RenderDevice,
    queue: &mut RenderQueue,
    gbp: &mut SingleGeometryBufferPool,
    dynbuffer: &mut DynUniformBuffer,
) {
    println!("cmd_mesh_builder");

    cmds.drain(..).for_each(|cmd| {
        match cmd {
            MeshBuilderCommand::Cube(id) => {
                match mesh_builders.get_mut(id) {
                    Some(mut item) => {
                        let data = CubeBuilder::build(device, queue, gbp);
                        item.0.insert_no_notify(data.0);
                        item.1.insert_no_notify(data.1);
                        item.2.insert_no_notify(data.2);
                        let mut temp = DefaultMaterialMeta::new(dynbuffer);
                        temp.init(device, dynbuffer);
                        item.3.insert_no_notify(temp);
                    },
                    None => {
                        
                    },
                }
            },
            MeshBuilderCommand::Plane(_) => {
                
            },
        }
    });
}

fn cmd_camera_render(
    cmds: &mut Vec<CameraCommand>,
    camera_render_without: &mut Query<GameObject, (Write<RenderObjectID>, Write<MainCameraBindGroup>)>,
    render_entities: &mut Query<GameObject, (Write<RenderObjectOpaqueList>)>,
    device: &RenderDevice,
    dynbuffer: &mut DynUniformBuffer,
) {
    println!("cmd_camera_render");
    cmds.drain(..).for_each(|cmd| {
        match cmd {
            CameraCommand::ActiveRender(id, render_id) => {
                match camera_render_without.get_mut(id) {
                    Some(mut item) => {
                        match render_entities.get_mut(render_id) {
                            Some(mut renderer) => {
                                item.0.insert_no_notify(RenderObjectID(render_id));
                                item.1.insert_no_notify(MainCameraBindGroup::new(device, dynbuffer));
                                renderer.insert_no_notify(RenderObjectOpaqueList::default());
                            },
                            None => {},
                        }
                    },
                    None => {
                        
                    },
                }
            },
            CameraCommand::DisableRender(_) => {
                
            },
        }
    });
}