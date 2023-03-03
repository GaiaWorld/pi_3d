use std::mem::replace;

use pi_assets::mgr::AssetMgr;
use pi_ecs::{prelude::{ResMut, Query, Commands, EntityDelete, Event, Res, EntityCommands, Component}};
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
    renderers::pass::*, state::{MeshStates, DirtyMeshStates}
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
        mut meshstate_cmd: Commands<GameObject, MeshStates>,
        mut meshstateflag_cmd: Commands<GameObject, DirtyMeshStates>,
        mut absmesh_cmd: Commands<GameObject, AbstructMesh>,
        mut wmdirty_cmd: Commands<GameObject, RenderMatrixDirty>,
        mut ins_wm_cmd: Commands<GameObject, InstancedWorldMatrixDirty>,
        mut ins_colordirty_cmd: Commands<GameObject, InstancedColorDirty>,
        mut ins_tilloffdirty_cmd: Commands<GameObject, InstanceTillOffDirty>,
        mut render_wm_cmd: Commands<GameObject, RenderWorldMatrix>,
        mut render_wminv_cmd: Commands<GameObject, RenderWorldMatrixInv>,
        mut ins_list_cmd: Commands<GameObject, InstanceList>,
        mut bind_model_cmd: Commands<GameObject, BindModel>,
        mut effect_value_cmd: Commands<GameObject, PassDirtyBindEffectValue>,
        mut effect_value_flag_cmd: Commands<GameObject, FlagPassDirtyBindEffectValue>,
        mut effect_textures_cmd: Commands<GameObject, PassDirtyBindEffectTextures>,
        mut effect_textures_flag_cmd: Commands<GameObject, FlagPassDirtyBindEffectTextures>,
        mut entity_cmd: EntityCommands<GameObject>,
        mut source_cmd: Commands<GameObject, PassSource>,
        mut bev_cmd: Commands<GameObject, PassBindEffectValue>,
        mut bet_cmd: Commands<GameObject, PassBindEffectTextures>,
        mut bgscene_cmd: Commands<GameObject, PassBindGroupScene>,
        mut bgmodel_cmd: Commands<GameObject, PassBindGroupModel>,
        mut bgtex_cmd: Commands<GameObject, PassBindGroupTextureSamplers>,
        mut bindgroups_cmd: Commands<GameObject, PassBindGroups>,
        mut ready_cmd: Commands<GameObject, PassReady>,
        mut shader_cmd: Commands<GameObject, PassShader>,
        mut pipeline_cmd: Commands<GameObject, PassPipeline>,
        mut draw_cmd: Commands<GameObject, PassDraw>,
        mut ins_record: ResMut<InstanceSourceRecord>,
        mut allocator: ResMut<BindBufferAllocator>,
        mut pass01_cmd: Commands<GameObject, Pass01>,
        mut pass02_cmd: Commands<GameObject, Pass02>,
        mut pass03_cmd: Commands<GameObject, Pass03>,
        mut pass04_cmd: Commands<GameObject, Pass04>,
        mut pass05_cmd: Commands<GameObject, Pass05>,
        mut pass06_cmd: Commands<GameObject, Pass06>,
        mut pass07_cmd: Commands<GameObject, Pass07>,
        mut pass08_cmd: Commands<GameObject, Pass08>,
        mut passid01_cmd: Commands<GameObject, PassID01>,
        mut passid02_cmd: Commands<GameObject, PassID02>,
        mut passid03_cmd: Commands<GameObject, PassID03>,
        mut passid04_cmd: Commands<GameObject, PassID04>,
        mut passid05_cmd: Commands<GameObject, PassID05>,
        mut passid06_cmd: Commands<GameObject, PassID06>,
        mut passid07_cmd: Commands<GameObject, PassID07>,
        mut passid08_cmd: Commands<GameObject, PassID08>,
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
                    meshstate_cmd.insert(entity, MeshStates::default());
                    meshstateflag_cmd.insert(entity, DirtyMeshStates);

                    if let Some(bind) = BindModel::new(&device, &mut allocator) {
                        bind_model_cmd.insert(entity.clone(), bind);
                        
                        create_passobj::<Pass01,PassID01>(entity, &mut entity_cmd, &mut pass01_cmd, &mut passid01_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);
                        create_passobj::<Pass02,PassID02>(entity, &mut entity_cmd, &mut pass02_cmd, &mut passid02_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);
                        create_passobj::<Pass03,PassID03>(entity, &mut entity_cmd, &mut pass03_cmd, &mut passid03_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);
                        create_passobj::<Pass04,PassID04>(entity, &mut entity_cmd, &mut pass04_cmd, &mut passid04_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);
                        create_passobj::<Pass05,PassID05>(entity, &mut entity_cmd, &mut pass05_cmd, &mut passid05_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);
                        create_passobj::<Pass06,PassID06>(entity, &mut entity_cmd, &mut pass06_cmd, &mut passid06_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);
                        create_passobj::<Pass07,PassID07>(entity, &mut entity_cmd, &mut pass07_cmd, &mut passid07_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);
                        create_passobj::<Pass08,PassID08>(entity, &mut entity_cmd, &mut pass08_cmd, &mut passid08_cmd, &mut source_cmd, &mut bev_cmd, &mut bet_cmd, &mut bgscene_cmd, &mut bgmodel_cmd, &mut bgtex_cmd, &mut bindgroups_cmd, &mut ready_cmd, &mut shader_cmd, &mut pipeline_cmd, &mut draw_cmd);

                        effect_value_cmd.insert(entity, PassDirtyBindEffectValue(0));
                        effect_value_flag_cmd.insert(entity, FlagPassDirtyBindEffectValue);
                        effect_textures_cmd.insert(entity, PassDirtyBindEffectTextures(0));
                        effect_textures_flag_cmd.insert(entity, FlagPassDirtyBindEffectTextures);
                    } else {
                        log::warn!("BindModel New() Fail !");
                    }

                },
            }
        });
    }
}

fn create_passobj<T: TPass + Component, T2: TPassID + Component>(
    model: ObjectID,
    entity_cmd: &mut EntityCommands<GameObject>,
    pass_cmd: &mut Commands<GameObject, T>,
    passid_cmd: &mut Commands<GameObject, T2>,
    source_cmd: &mut Commands<GameObject, PassSource>,
    bev_cmd: &mut Commands<GameObject, PassBindEffectValue>,
    bet_cmd: &mut Commands<GameObject, PassBindEffectTextures>,
    bgscene_cmd: &mut Commands<GameObject, PassBindGroupScene>,
    bgmodel_cmd: &mut Commands<GameObject, PassBindGroupModel>,
    bgtex_cmd: &mut Commands<GameObject, PassBindGroupTextureSamplers>,
    bindgroups_cmd: &mut Commands<GameObject, PassBindGroups>,
    ready_cmd: &mut Commands<GameObject, PassReady>,
    shader_cmd: &mut Commands<GameObject, PassShader>,
    pipeline_cmd: &mut Commands<GameObject, PassPipeline>,
    draw_cmd: &mut Commands<GameObject, PassDraw>,
) -> ObjectID {
    let id = entity_cmd.spawn();

    passid_cmd.insert(model, T2::new(id));
    pass_cmd.insert(id, T::new());
    source_cmd.insert(id, PassSource(model));
    bev_cmd.insert(id, PassBindEffectValue(None));
    bet_cmd.insert(id, PassBindEffectTextures(None));
    bgscene_cmd.insert(id, PassBindGroupScene(None));
    bgmodel_cmd.insert(id, PassBindGroupModel(None));
    bgtex_cmd.insert(id, PassBindGroupTextureSamplers(None));
    bindgroups_cmd.insert(id, PassBindGroups(None));
    ready_cmd.insert(id, PassReady(None));
    shader_cmd.insert(id, PassShader(None));
    pipeline_cmd.insert(id, PassPipeline(None));
    draw_cmd.insert(id, PassDraw(None));

    id
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