use std::mem::replace;

use pi_ecs::{sys::system, prelude::{ResMut, Query, Setup, Commands}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;

use crate::{
    object::{ObjectID, GameObject},
    flags::SceneID,
    scene::environment::fog::SceneFog,
};

use super::{coordinate_system::SceneCoordinateSytem, scene_time::SceneTime};

#[derive(Debug)]
pub enum SceneCommand {
    Create(ObjectID),
    AddObject(ObjectID, SceneID),
}

#[derive(Debug, Default)]
pub struct SingleSceneCommandList {
    pub list: Vec<SceneCommand>,
}
pub struct SysSceneCommand;
impl TSystemStageInfo for SysSceneCommand {
    
}
#[setup]
impl SysSceneCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleSceneCommandList>,
        mut coordsys_cmd: Commands<GameObject, SceneCoordinateSytem>,
        mut scenetime_cmd: Commands<GameObject, SceneTime>,
        mut scenefog_cmd: Commands<GameObject, SceneFog>,
        mut obj_cmd: Commands<GameObject, SceneID>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                SceneCommand::Create(entity) => {
                    coordsys_cmd.insert(entity, SceneCoordinateSytem::default());
                    scenetime_cmd.insert(entity, SceneTime::new());
                    scenefog_cmd.insert(entity, SceneFog::new());
                },
                SceneCommand::AddObject(entity, scene) => {
                    obj_cmd.insert(entity, scene);
                },
            }
        });

    }
}
