use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Commands, EntityDelete}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{Vector4, Matrix};

use crate::{
    object::{ObjectID, GameObject},
    geometry::{
        instance::{instance_color::{InstanceColor, InstancedColorDirty}, instance_tilloff::{InstanceTillOff, InstanceTillOffDirty}, InstanceList, InstanceSource, InstanceSourceRecord, instance_world_matrix::InstancedWorldMatrixDirty}
    }
};

use super::{model::{RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty}, abstract_mesh::AbstructMesh, Mesh};

#[derive(Debug)]
pub enum EMeshCreateCommand {
    Create(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleMeshCreateCommandList {
    pub list: Vec<EMeshCreateCommand>,
}

pub struct SysMeshCreateCommand;
impl TSystemStageInfo for SysMeshCreateCommand {
}
#[setup]
impl SysMeshCreateCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMeshCreateCommandList>,
        mut mesh_cmd: Commands<GameObject, Mesh>,
        mut absmesh_cmd: Commands<GameObject, AbstructMesh>,
        mut wmdirty_cmd: Commands<GameObject, RenderMatrixDirty>,
        mut ins_wm_cmd: Commands<GameObject, InstancedWorldMatrixDirty>,
        mut ins_colordirty_cmd: Commands<GameObject, InstancedColorDirty>,
        mut ins_tilloffdirty_cmd: Commands<GameObject, InstanceTillOffDirty>,
        mut render_wm_cmd: Commands<GameObject, RenderWorldMatrix>,
        mut render_wminv_cmd: Commands<GameObject, RenderWorldMatrixInv>,
        mut ins_list_cmd: Commands<GameObject, InstanceList>,
        mut ins_record: ResMut<InstanceSourceRecord>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EMeshCreateCommand::Create(entity) => {
                    mesh_cmd.insert(entity.clone(), Mesh);
                    ins_list_cmd.insert(entity.clone(), InstanceList::new(&mut ins_record));
                    absmesh_cmd.insert(entity.clone(), AbstructMesh);
                    render_wm_cmd.insert(entity.clone(), RenderWorldMatrix(Matrix::identity()));
                    render_wminv_cmd.insert(entity.clone(), RenderWorldMatrixInv(Matrix::identity()));
                    wmdirty_cmd.insert(entity.clone(), RenderMatrixDirty(true));
                    ins_wm_cmd.insert(entity.clone(), InstancedWorldMatrixDirty(true));
                    ins_colordirty_cmd.insert(entity.clone(), InstancedColorDirty(true));
                    ins_tilloffdirty_cmd.insert(entity.clone(), InstanceTillOffDirty(true));
                },
            }
        });
    }
}

#[derive(Debug)]
pub enum EMeshModifyCommand {
    Destroy(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleMeshModifyCommandList {
    pub list: Vec<EMeshModifyCommand>,
}

pub struct SysMeshModifyCommand;
impl TSystemStageInfo for SysMeshModifyCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMeshCreateCommand::key()
        ]
    }
}
#[setup]
impl SysMeshModifyCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMeshModifyCommandList>,
        mut meshes: Query<GameObject, &mut InstanceList>,
        mut delete: EntityDelete<GameObject>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EMeshModifyCommand::Destroy(id_mesh) => {
                    if let Some(instances) = meshes.get(id_mesh) {
                        instances.list.iter().for_each(|id_instance| {
                            delete.despawn(id_instance.clone());
                        });
                    }
                    delete.despawn(id_mesh);
                },
            }
        });
    }
}


#[derive(Debug)]
pub enum EInstanceMeshCreateCommand {
    CreateInstance(ObjectID, ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleInstanceMeshCreateCommandList {
    pub list: Vec<EInstanceMeshCreateCommand>,
}

pub struct SysInstanceMeshCreateCommand;
impl TSystemStageInfo for SysInstanceMeshCreateCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMeshCreateCommand::key()
        ]
    }
}
#[setup]
impl SysInstanceMeshCreateCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleInstanceMeshCreateCommandList>,
        mut meshes: Query<GameObject, &mut InstanceList>,
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
                EInstanceMeshCreateCommand::CreateInstance(source, id_instance) => {
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
        });
    }
}


#[derive(Debug)]
pub enum EInstanceMeshModifyCommand {
    InstanceColor(ObjectID, Vector4),
    InstanceTillOff(ObjectID, Vector4),
    DestroyInstance(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleInstanceMeshModifyCommandList {
    pub list: Vec<EInstanceMeshModifyCommand>,
}

pub struct SysInstanceMeshModifyCommand;
impl TSystemStageInfo for SysInstanceMeshModifyCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysInstanceMeshCreateCommand::key()
        ]
    }
}
#[setup]
impl SysInstanceMeshModifyCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleInstanceMeshModifyCommandList>,
        mut meshes: Query<GameObject, &mut InstanceList>,
        instances: Query<GameObject, &InstanceSource>,
        mut ins_wm_cmd: Commands<GameObject, InstancedWorldMatrixDirty>,
        mut ins_colordirty_cmd: Commands<GameObject, InstancedColorDirty>,
        mut ins_tilloffdirty_cmd: Commands<GameObject, InstanceTillOffDirty>,
        mut ins_color_cmd: Commands<GameObject, InstanceColor>,
        mut ins_tilloff_cmd: Commands<GameObject, InstanceTillOff>,
        mut delete: EntityDelete<GameObject>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EInstanceMeshModifyCommand::InstanceColor(instance, color) => {
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
                EInstanceMeshModifyCommand::InstanceTillOff(instance, value) => {
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
                EInstanceMeshModifyCommand::DestroyInstance(id_instance) => {
                    if let Some(source) = instances.get(id_instance) {
                        match meshes.get_mut(source.0) {
                            Some((mut list)) => {
                                let mut index = 0;
                                let mut flag = false;
                                for v in list.list.iter() {
                                    if v == &id_instance {
                                        flag = true;
                                        break;
                                    }
                                    index += 1;
                                }
                                if flag {
                                    list.list.swap_remove(index);
                                }
                                
                                ins_colordirty_cmd.insert(source.0.clone(), InstancedColorDirty(true));
                                ins_tilloffdirty_cmd.insert(source.0.clone(), InstanceTillOffDirty(true));
                                ins_wm_cmd.insert(source.0.clone(), InstancedWorldMatrixDirty(true));
                            },
                            None => {
                                cmds.list.push(cmd);
                            },
                        }
                    }

                    delete.despawn(id_instance);
                },
            }
        });
    }
}