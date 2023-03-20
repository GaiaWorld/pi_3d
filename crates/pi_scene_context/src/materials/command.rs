use std::{mem::replace};

use pi_ecs::{prelude::{Event, Query, ResMut, Commands}};
use pi_ecs_macros::{setup, listen};
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo, assets::sync_load::AssetSyncLoad};
use pi_render::renderer::shader::KeyShaderMeta;

use crate::pass::EPassTag;

use super::{
    material::{MaterialID, MaterialUsedList, DirtyMaterialUsedList},
    shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta, ShaderEffectMeta},
    uniforms::{
        texture::{UniformTextureWithSamplerParams},
    }
};



#[derive(Debug)]
pub enum EMatCreateCommand {
    Use(ObjectID, KeyShaderMeta, EPassTag),
}
#[derive(Debug, Default)]
pub struct SingleMatCreateCommands(pub Vec<EMatCreateCommand>);

pub struct SysMaterailCreateCommands;
impl TSystemStageInfo for SysMaterailCreateCommands {
    
}
#[setup]
impl SysMaterailCreateCommands {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleMatCreateCommands>,
        mut items: Commands<GameObject, AssetKeyShaderEffect>,
        mut usedlist_cmd: Commands<GameObject, MaterialUsedList>,
        mut dirty_cmd: Commands<GameObject, DirtyMaterialUsedList>,
        mut passtag_cmd: Commands<GameObject, EPassTag>,
        mut texparams_cmd: Commands<GameObject, UniformTextureWithSamplerParams>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EMatCreateCommand::Use(entity, key, passtag) => {
                    items.insert(entity, AssetKeyShaderEffect(key));
                    usedlist_cmd.insert(entity, MaterialUsedList::default());
                    passtag_cmd.insert(entity, passtag);
                    texparams_cmd.insert(entity, UniformTextureWithSamplerParams::default());
                    dirty_cmd.insert(entity, DirtyMaterialUsedList);
                },
            }
        });
    }
}

#[derive(Debug)]
pub enum EMaterialIDCommand {
    Use(ObjectID, MaterialID),
    UnUse(ObjectID, MaterialID),
}

#[derive(Debug, Default)]
pub struct SingleMaterialIDCommandList {
    pub list: Vec<EMaterialIDCommand>,
}

pub struct SysMaterialIDCommand;
impl TSystemStageInfo for SysMaterialIDCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMaterailCreateCommands::key()
        ]
    }
}
#[setup]
impl SysMaterialIDCommand {
    #[listen(entity=(GameObject, Delete))]
    fn listen(
        e: Event,
        items: Query<GameObject, (ObjectID, &MaterialID)>,
        mut material: Query<GameObject, &mut MaterialUsedList>,
    ) {
        if let Some((obj, id_mat)) = items.get_by_entity(e.id) {
            material.iter_mut().for_each(|mut list| {
                list.0.remove(&obj);
            });
        }
    }
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleMaterialIDCommandList>,
        mut material: Query<GameObject, &mut MaterialUsedList>,
        mut dirty_cmd: Commands<GameObject, DirtyMaterialUsedList>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                EMaterialIDCommand::Use(obj, id_mat) => {
                    if let Some(mut list) = material.get_mut(id_mat.0) {
                        list.0.insert(obj.clone(), obj.clone());
                        dirty_cmd.insert(id_mat.0.clone(), DirtyMaterialUsedList);
                    }
                },
                EMaterialIDCommand::UnUse(obj, id_mat) => {
                    if let Some(mut list) = material.get_mut(id_mat.0) {
                        list.0.remove(&obj);
                    }
                },
            }
        });
    }
}

pub type SysAssetShaderEffectLoad = AssetSyncLoad::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta, SysMaterailCreateCommands>;
