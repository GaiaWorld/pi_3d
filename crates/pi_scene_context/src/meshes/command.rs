use std::mem::replace;

use pi_assets::mgr::AssetMgr;
use pi_ecs::{prelude::{ResMut, Query, Commands, EntityDelete, Event, Res}};
use pi_ecs_macros::{setup, listen};
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_render::{rhi::device::RenderDevice, renderer::bind_buffer::{BindBufferAllocator}};
use pi_scene_math::{Vector4, Matrix};
use pi_share::Share;

use crate::{
    object::{ObjectID, GameObject},
    geometry::{
        instance::{instance_color::{InstanceColor, InstancedColorDirty}, instance_tilloff::{InstanceTillOff, InstanceTillOffDirty}, InstanceList, InstanceSource, InstanceSourceRecord, instance_world_matrix::InstancedWorldMatrixDirty}
    },
    pass::*,
    renderers::pass::*
};

use super::{model::{RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty, BindModel}, abstract_mesh::AbstructMesh, Mesh};

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
        mut bind_model_cmd: Commands<GameObject, BindModel>,
        mut bev_cmd: (
            Commands<GameObject, Pass01BindEffectValue>, Commands<GameObject, Pass02BindEffectValue>, Commands<GameObject, Pass03BindEffectValue>, Commands<GameObject, Pass04BindEffectValue>, 
            Commands<GameObject, Pass05BindEffectValue>, Commands<GameObject, Pass06BindEffectValue>, Commands<GameObject, Pass07BindEffectValue>, Commands<GameObject, Pass08BindEffectValue>, 
        ),
        mut bet_cmd: (
            Commands<GameObject, Pass01BindEffectTextures>, Commands<GameObject, Pass02BindEffectTextures>, Commands<GameObject, Pass03BindEffectTextures>, Commands<GameObject, Pass04BindEffectTextures>, 
            Commands<GameObject, Pass05BindEffectTextures>, Commands<GameObject, Pass06BindEffectTextures>, Commands<GameObject, Pass07BindEffectTextures>, Commands<GameObject, Pass08BindEffectTextures>, 
        ),
        mut bgscene_cmd: (
            Commands<GameObject, Pass01BindGroupScene>, Commands<GameObject, Pass02BindGroupScene>, Commands<GameObject, Pass03BindGroupScene>, Commands<GameObject, Pass04BindGroupScene>, 
            Commands<GameObject, Pass05BindGroupScene>, Commands<GameObject, Pass06BindGroupScene>, Commands<GameObject, Pass07BindGroupScene>, Commands<GameObject, Pass08BindGroupScene>, 
        ),
        mut bgmodel_cmd: (
            Commands<GameObject, Pass01BindGroupModel>, Commands<GameObject, Pass02BindGroupModel>, Commands<GameObject, Pass03BindGroupModel>, Commands<GameObject, Pass04BindGroupModel>, 
            Commands<GameObject, Pass05BindGroupModel>, Commands<GameObject, Pass06BindGroupModel>, Commands<GameObject, Pass07BindGroupModel>, Commands<GameObject, Pass08BindGroupModel>, 
        ),
        mut bgtex_cmd: (
            Commands<GameObject, Pass01BindGroupTextureSamplers>, Commands<GameObject, Pass02BindGroupTextureSamplers>, Commands<GameObject, Pass03BindGroupTextureSamplers>, Commands<GameObject, Pass04BindGroupTextureSamplers>, 
            Commands<GameObject, Pass05BindGroupTextureSamplers>, Commands<GameObject, Pass06BindGroupTextureSamplers>, Commands<GameObject, Pass07BindGroupTextureSamplers>, Commands<GameObject, Pass08BindGroupTextureSamplers>, 
        ),
        mut ready_cmd: (
            Commands<GameObject, Pass01Ready>, Commands<GameObject, Pass02Ready>, Commands<GameObject, Pass03Ready>, Commands<GameObject, Pass04Ready>, 
            Commands<GameObject, Pass05Ready>, Commands<GameObject, Pass06Ready>, Commands<GameObject, Pass07Ready>, Commands<GameObject, Pass08Ready>, 
        ),
        mut shader_cmd: (
            Commands<GameObject, Pass01Shader>, Commands<GameObject, Pass02Shader>, Commands<GameObject, Pass03Shader>, Commands<GameObject, Pass04Shader>, 
            Commands<GameObject, Pass05Shader>, Commands<GameObject, Pass06Shader>, Commands<GameObject, Pass07Shader>, Commands<GameObject, Pass08Shader>,  
        ),
        mut draw_cmd: (
            Commands<GameObject, Pass01Draw>, Commands<GameObject, Pass02Draw>, Commands<GameObject, Pass03Draw>, Commands<GameObject, Pass04Draw>, 
            Commands<GameObject, Pass05Draw>, Commands<GameObject, Pass06Draw>, Commands<GameObject, Pass07Draw>, Commands<GameObject, Pass08Draw>, 
        ),
        mut ins_record: ResMut<InstanceSourceRecord>,
        mut allocator: ResMut<BindBufferAllocator>,
        device: ResMut<RenderDevice>,
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

                    if let Some(bind) = BindModel::new(&device, &mut allocator) {
                        bind_model_cmd.insert(entity.clone(), bind);

                        bev_cmd.0.insert(entity.clone(), Pass01BindEffectValue(None));
                        bev_cmd.1.insert(entity.clone(), Pass02BindEffectValue(None));
                        bev_cmd.2.insert(entity.clone(), Pass03BindEffectValue(None));
                        bev_cmd.3.insert(entity.clone(), Pass04BindEffectValue(None));
                        bev_cmd.4.insert(entity.clone(), Pass05BindEffectValue(None)); 
                        bev_cmd.5.insert(entity.clone(), Pass06BindEffectValue(None));
                        bev_cmd.6.insert(entity.clone(), Pass07BindEffectValue(None));
                        bev_cmd.7.insert(entity.clone(), Pass08BindEffectValue(None));
                        
    
                        bet_cmd.0.insert(entity.clone(), Pass01BindEffectTextures(None));
                        bet_cmd.1.insert(entity.clone(), Pass02BindEffectTextures(None));
                        bet_cmd.2.insert(entity.clone(), Pass03BindEffectTextures(None));
                        bet_cmd.3.insert(entity.clone(), Pass04BindEffectTextures(None));
                        bet_cmd.4.insert(entity.clone(), Pass05BindEffectTextures(None));
                        bet_cmd.5.insert(entity.clone(), Pass06BindEffectTextures(None));
                        bet_cmd.6.insert(entity.clone(), Pass07BindEffectTextures(None));
                        bet_cmd.7.insert(entity.clone(), Pass08BindEffectTextures(None));
    
                        bgscene_cmd.0.insert(entity.clone(), Pass01BindGroupScene(None));
                        bgscene_cmd.1.insert(entity.clone(), Pass02BindGroupScene(None));
                        bgscene_cmd.2.insert(entity.clone(), Pass03BindGroupScene(None));
                        bgscene_cmd.3.insert(entity.clone(), Pass04BindGroupScene(None));
                        bgscene_cmd.4.insert(entity.clone(), Pass05BindGroupScene(None));
                        bgscene_cmd.5.insert(entity.clone(), Pass06BindGroupScene(None));
                        bgscene_cmd.6.insert(entity.clone(), Pass07BindGroupScene(None));
                        bgscene_cmd.7.insert(entity.clone(), Pass08BindGroupScene(None));
                        
                        bgmodel_cmd.0.insert(entity.clone(), Pass01BindGroupModel(None));
                        bgmodel_cmd.1.insert(entity.clone(), Pass02BindGroupModel(None));
                        bgmodel_cmd.2.insert(entity.clone(), Pass03BindGroupModel(None));
                        bgmodel_cmd.3.insert(entity.clone(), Pass04BindGroupModel(None));
                        bgmodel_cmd.4.insert(entity.clone(), Pass05BindGroupModel(None));
                        bgmodel_cmd.5.insert(entity.clone(), Pass06BindGroupModel(None));
                        bgmodel_cmd.6.insert(entity.clone(), Pass07BindGroupModel(None));
                        bgmodel_cmd.7.insert(entity.clone(), Pass08BindGroupModel(None));
                        
                        bgtex_cmd.0.insert(entity.clone(), Pass01BindGroupTextureSamplers(None));
                        bgtex_cmd.1.insert(entity.clone(), Pass02BindGroupTextureSamplers(None));
                        bgtex_cmd.2.insert(entity.clone(), Pass03BindGroupTextureSamplers(None));
                        bgtex_cmd.3.insert(entity.clone(), Pass04BindGroupTextureSamplers(None));
                        bgtex_cmd.4.insert(entity.clone(), Pass05BindGroupTextureSamplers(None));
                        bgtex_cmd.5.insert(entity.clone(), Pass06BindGroupTextureSamplers(None));
                        bgtex_cmd.6.insert(entity.clone(), Pass07BindGroupTextureSamplers(None));
                        bgtex_cmd.7.insert(entity.clone(), Pass08BindGroupTextureSamplers(None));
                        
                        ready_cmd.0.insert(entity.clone(), Pass01Ready(None));
                        ready_cmd.1.insert(entity.clone(), Pass02Ready(None));
                        ready_cmd.2.insert(entity.clone(), Pass03Ready(None));
                        ready_cmd.3.insert(entity.clone(), Pass04Ready(None));
                        ready_cmd.4.insert(entity.clone(), Pass05Ready(None));
                        ready_cmd.5.insert(entity.clone(), Pass06Ready(None));
                        ready_cmd.6.insert(entity.clone(), Pass07Ready(None));
                        ready_cmd.7.insert(entity.clone(), Pass08Ready(None));
                        
                        shader_cmd.0.insert(entity.clone(), Pass01Shader(None));
                        shader_cmd.1.insert(entity.clone(), Pass02Shader(None));
                        shader_cmd.2.insert(entity.clone(), Pass03Shader(None));
                        shader_cmd.3.insert(entity.clone(), Pass04Shader(None));
                        shader_cmd.4.insert(entity.clone(), Pass05Shader(None));
                        shader_cmd.5.insert(entity.clone(), Pass06Shader(None));
                        shader_cmd.6.insert(entity.clone(), Pass07Shader(None));
                        shader_cmd.7.insert(entity.clone(), Pass08Shader(None));
                        
                        draw_cmd.0.insert(entity.clone(), Pass01Draw(None));
                        draw_cmd.1.insert(entity.clone(), Pass02Draw(None));
                        draw_cmd.2.insert(entity.clone(), Pass03Draw(None));
                        draw_cmd.3.insert(entity.clone(), Pass04Draw(None));
                        draw_cmd.4.insert(entity.clone(), Pass05Draw(None));
                        draw_cmd.5.insert(entity.clone(), Pass06Draw(None));
                        draw_cmd.6.insert(entity.clone(), Pass07Draw(None));
                        draw_cmd.7.insert(entity.clone(), Pass08Draw(None));
                    } else {
                        log::warn!("BindModel New() Fail !");
                    }

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
        meshes: Query<GameObject, &mut InstanceList>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EMeshModifyCommand::Destroy(id_mesh) => {
                    // if let Some(instances) = meshes.get(id_mesh) {
                    //     instances.list.iter().for_each(|id_instance| {
                    //         delete.despawn(id_instance.clone());
                    //     });
                    // }
                    // delete.despawn(id_mesh);
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
    /// Mesh 销毁时 附带销毁InstancedMesh
    #[listen(entity=(GameObject, Delete))]
    fn listen(
        e: Event,
        meshes: Query<GameObject, &InstanceList>,
        mut delete: EntityDelete<GameObject>,
    ) {
        if let Some(instances) = meshes.get_by_entity(e.id) {
            instances.list.iter().for_each(|id| {
                delete.despawn(id.clone());
            });
        }
    }
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