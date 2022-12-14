use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::Vector4;
use render_data_container::VertexBufferPool;

use crate::{object::{ObjectID, GameObject}, renderers::{render_blend::RenderBlend, render_primitive::PrimitiveState, render_depth_and_stencil::RenderDepthAndStencil}, layer_mask::LayerMask, resources::RenderDynUniformBuffer, geometry::instance::{instance_color::InstanceColor, instance_tilloff::InstanceTillOff}};

use super::{model::BuildinModelBind, instance::{instanced_mesh::{InstanceList, InstanceSource, InstanceSourceRecord}, world_matrix::InstancedWorldMatrixDirty, instance_color::InstancedColorDirty, instance_tilloff::InstanceTillOffDirty}, abstract_mesh::AbstructMesh};

#[derive(Debug)]
pub enum MeshCommand {
    Create(ObjectID),
    CreateInstance(ObjectID, ObjectID),
    InstanceColor(ObjectID, Vector4),
    InstanceTillOff(ObjectID, Vector4),
    Destroy(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleMeshCommandList {
    pub list: Vec<MeshCommand>,
}

pub struct SysMeshCommand;
#[setup]
impl SysMeshCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMeshCommandList>,
        mut meshes: Query<GameObject, (Write<BuildinModelBind>, Write<InstanceList>, Write<AbstructMesh>, Write<InstancedColorDirty>, Write<InstanceTillOffDirty>)>,
        mut instances: Query<GameObject, (Write<InstanceSource>, Write<AbstructMesh>, Write<InstanceColor>, Write<InstanceTillOff>)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut ins_record: ResMut<InstanceSourceRecord>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                MeshCommand::Create(entity) => {
                    match meshes.get_mut(entity) {
                        Some((mut item, _, mut abstruct_mesh, mut inscolor_dirty, _)) => {
                            item.write(BuildinModelBind::new(&mut dynbuffer));
                            abstruct_mesh.write(AbstructMesh);
                        },
                        None => {
                            
                        },
                    }
                },
                MeshCommand::InstanceColor(instance, color) => {
                    match instances.get_mut(instance) {
                        Some((source, mut abstruct_mesh, mut inscolor, _)) => {
                            if let Some(source) = source.get() {
                                match meshes.get_mut(source.0) {
                                    Some((mut item, mut list, _, mut inscolor_dirty, _)) => {
                                        inscolor_dirty.write(InstancedColorDirty);
                                    },
                                    None => {}
                                };
                            }

                            inscolor.write(InstanceColor(color));
                        },
                        None => todo!(),
                    }
                },
                MeshCommand::InstanceTillOff(instance, value) => {
                    match instances.get_mut(instance) {
                        Some((source, mut abstruct_mesh, _, mut instilloff)) => {
                            if let Some(source) = source.get() {
                                match meshes.get_mut(source.0) {
                                    Some((mut item, mut list, _, mut inscolor_dirty, mut instilloff_dirty)) => {
                                        instilloff_dirty.write(InstanceTillOffDirty);
                                    },
                                    None => {}
                                };
                            }

                            instilloff.write(InstanceTillOff(value));
                        },
                        None => todo!(),
                    }
                },
                MeshCommand::Destroy(_) => todo!(),
                MeshCommand::CreateInstance(source, instance) => {
                    match meshes.get_mut(source.clone()) {
                        Some((mut item, mut list, _, mut inscolor_dirty, mut instilloff_dirty)) => {
                            match list.get_mut() {
                                Some(source) => {
                                    source.list.push(instance);
                                    list.notify_modify();
                                    // println!(">>> CreateInstance 0");
                                },
                                None => {
                                    let mut temp = InstanceList::new(&mut ins_record);
                                    temp.list.push(instance);
                                    list.write(temp);
                                    // println!(">>> CreateInstance 1");
                                },
                            }
                            inscolor_dirty.write(InstancedColorDirty);
                            instilloff_dirty.write(InstanceTillOffDirty);
                        },
                        None => {
                            
                        },
                    }
                    

                    match instances.get_mut(instance) {
                        Some((mut instance, mut abstruct_mesh, mut inscolor, mut instilloff)) => {
                            instance.write(InstanceSource(source));
                            abstruct_mesh.write(AbstructMesh);
                            inscolor.write(InstanceColor(Vector4::new(1., 1., 1., 1.)));
                            instilloff.write(InstanceTillOff(Vector4::new(1., 1., 0., 0.)));
                        },
                        None => todo!(),
                    }
                },
            }
        });
    }
}
