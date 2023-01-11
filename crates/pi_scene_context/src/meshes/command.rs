use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Commands}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{Vector4, Matrix};
use render_data_container::VertexBufferPool;
use render_shader::instance_code::EInstanceCode;

use crate::{object::{ObjectID, GameObject}, renderers::{render_blend::RenderBlend, render_primitive::PrimitiveState, render_depth_and_stencil::RenderDepthAndStencil}, layer_mask::LayerMask,  geometry::{instance::{instance_color::{InstanceColor, InstancedColorDirty}, instance_tilloff::{InstanceTillOff, InstanceTillOffDirty}, InstanceList, InstanceSource, InstanceSourceRecord, instance_world_matrix::InstancedWorldMatrixDirty}, indices::SysGeometryCommand}, transforms::command::SysTransformNodeCommand};

use super::{model::{BuildinModelBind, RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty}, abstract_mesh::AbstructMesh, Mesh};

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
impl TSystemStageInfo for SysMeshCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysTransformNodeCommand::key()
        ]
    }
}
#[setup]
impl SysMeshCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMeshCommandList>,
        mut meshes: Query<GameObject, &mut InstanceList>,
        
        mut instances: Query<GameObject, &InstanceSource>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut ins_record: ResMut<InstanceSourceRecord>,
        mut mesh_cmd: Commands<GameObject, Mesh>,
        mut absmesh_cmd: Commands<GameObject, AbstructMesh>,
        mut source_cmd: Commands<GameObject, InstanceSource>,
        mut wmdirty_cmd: Commands<GameObject, RenderMatrixDirty>,
        mut ins_wm_cmd: Commands<GameObject, InstancedWorldMatrixDirty>,
        mut ins_colordirty_cmd: Commands<GameObject, InstancedColorDirty>,
        mut ins_tilloffdirty_cmd: Commands<GameObject, InstanceTillOffDirty>,
        mut ins_color_cmd: Commands<GameObject, InstanceColor>,
        mut ins_tilloff_cmd: Commands<GameObject, InstanceTillOff>,
        mut render_wm_cmd: Commands<GameObject, RenderWorldMatrix>,
        mut render_wminv_cmd: Commands<GameObject, RenderWorldMatrixInv>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                MeshCommand::Create(entity) => {
                    mesh_cmd.insert(entity.clone(), Mesh);
                    absmesh_cmd.insert(entity.clone(), AbstructMesh);
                    render_wm_cmd.insert(entity.clone(), RenderWorldMatrix(Matrix::identity()));
                    render_wminv_cmd.insert(entity.clone(), RenderWorldMatrixInv(Matrix::identity()));
                    wmdirty_cmd.insert(entity.clone(), RenderMatrixDirty(true));
                    ins_wm_cmd.insert(entity.clone(), InstancedWorldMatrixDirty(true));
                    ins_colordirty_cmd.insert(entity.clone(), InstancedColorDirty(true));
                    ins_tilloffdirty_cmd.insert(entity.clone(), InstanceTillOffDirty(true));
                },
                MeshCommand::InstanceColor(instance, color) => {
                    match instances.get(instance) {
                        Some(source) => {
                            if let Some(mut inslist) = meshes.get_mut(source.0) {
                                ins_colordirty_cmd.insert(source.0, InstancedColorDirty(true));
                                ins_color_cmd.insert(instance, InstanceColor(color));
                                if inslist.list.contains(&instance) == false {
                                    inslist.list.push(instance);
                                }
                            }
                        },
                        None =>  {
                            cmds.list.push(cmd);
                        },
                    }
                },
                MeshCommand::InstanceTillOff(instance, value) => {
                    match instances.get(instance) {
                        Some(source) => {
                            if let Some(mut inslist) = meshes.get_mut(source.0) {
                                ins_tilloffdirty_cmd.insert(source.0, InstanceTillOffDirty(true));
                                ins_tilloff_cmd.insert(instance, InstanceTillOff(value));
                                if inslist.list.contains(&instance) == false {
                                    inslist.list.push(instance);
                                }
                            }
                        },
                        None =>  {
                            cmds.list.push(cmd);
                        },
                    }
                },
                MeshCommand::Destroy(_) => todo!(),
                MeshCommand::CreateInstance(source, id_instance) => {

                    match instances.get(id_instance) {
                        Some(instance) => {
                            // Error
                        },
                        None => {
                            match meshes.get_mut(source.clone()) {
                                Some((mut list)) => {
                                    if list.list.contains(&id_instance) == false {
                                        list.list.push(id_instance);
                                    }
                                    
                                    ins_colordirty_cmd.insert(source.clone(), InstancedColorDirty(true));
                                    ins_tilloffdirty_cmd.insert(source.clone(), InstanceTillOffDirty(true));
                                    ins_wm_cmd.insert(source.clone(), InstancedWorldMatrixDirty(true));

                                    source_cmd.insert(id_instance, InstanceSource(source));
                                    ins_color_cmd.insert(id_instance, InstanceColor(Vector4::new(1., 1., 1., 1.)));
                                    ins_tilloff_cmd.insert(id_instance, InstanceTillOff(Vector4::new(1., 1., 0., 0.)));

                                    absmesh_cmd.insert(id_instance, AbstructMesh);
                                    wmdirty_cmd.insert(id_instance, RenderMatrixDirty(true));
                                    render_wm_cmd.insert(id_instance, RenderWorldMatrix(Matrix::identity()));
                                    render_wminv_cmd.insert(id_instance, RenderWorldMatrixInv(Matrix::identity()));
                                },
                                None => {
                                    cmds.list.push(cmd);
                                },
                            }
                        },
                    }
                    

                },
            }
        });
    }
}
