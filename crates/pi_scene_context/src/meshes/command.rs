use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query}, query::Write};
use pi_ecs_macros::setup;
use render_data_container::VertexBufferPool;

use crate::{object::{ObjectID, GameObject}, renderers::{render_blend::RenderBlend, render_primitive::PrimitiveState, render_depth_and_stencil::RenderDepthAndStencil}, layer_mask::LayerMask, resources::RenderDynUniformBuffer};

use super::{model::BuildinModelBind, instance::{instanced_mesh::{InstanceList, InstanceSource, InstanceSourceRecord}, world_matrix::InstancedBufferWorldMatrix}};

#[derive(Debug)]
pub enum MeshCommand {
    Create(ObjectID),
    WithInstance(ObjectID, usize),
    WithInstanceColor(ObjectID, usize),
    CreateInstance(ObjectID, ObjectID),
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
        mut meshes: Query<GameObject, (Write<BuildinModelBind>, Write<InstanceList>, Write<InstancedBufferWorldMatrix>)>,
        mut instances: Query<GameObject, (Write<InstanceSource>)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut vbpool: ResMut<VertexBufferPool>,
        mut ins_source_record: ResMut<InstanceSourceRecord>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                MeshCommand::Create(entity) => {
                    match meshes.get_mut(entity) {
                        Some((mut item, _, _)) => {
                            item.write(BuildinModelBind::new(&mut dynbuffer));
                        },
                        None => {
                            
                        },
                    }
                },
                MeshCommand::Destroy(_) => todo!(),
                MeshCommand::WithInstance(source, index) => {
                    match meshes.get_mut(source.clone()) {
                        Some((mut item, mut inslist, mut wmbuffer)) => {
                            let mut list = InstanceList::new(&mut ins_source_record);
                            list.list.push(source.clone());

                            wmbuffer.write(InstancedBufferWorldMatrix::new(index, list.id(), &mut vbpool));
                            inslist.write(list);
                        },
                        None => {
                            
                        },
                    }
                },
                MeshCommand::WithInstanceColor(source, index) => {
                    
                },
                MeshCommand::CreateInstance(source, instance) => {
                    match meshes.get_mut(source.clone()) {
                        Some((mut item, mut list, _)) => {
                            match list.get_mut() {
                                Some(source) => {
                                    source.list.push(instance.clone());
                                    list.notify_modify();
                                },
                                None => {
                                    //
                                },
                            }
                        },
                        None => {
                            
                        },
                    }
                    

                    match instances.get_mut(instance) {
                        Some(mut instance) => {
                            instance.write(InstanceSource(source));
                        },
                        None => todo!(),
                    }
                },
            }
        });
    }
}
