use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;

use crate::{
    object::{ObjectID, GameObject},
    flags::SceneID,
    scene::environment::fog::SceneFog, animation::base::{SceneAnimationContext, AnimationGroups}, transforms::tree_left_right::{TreeLeftRoot, TreeRightRoot},
};

use super::{coordinate_system::SceneCoordinateSytem, scene_time::SceneTime};

#[derive(Debug)]
pub enum SceneCommand {
    Create(ObjectID, ObjectID, ObjectID),
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
        mut leftroot_cmd: Commands<GameObject, TreeLeftRoot>,
        mut rightroot_cmd: Commands<GameObject, TreeRightRoot>,
        mut obj_cmd: Commands<GameObject, SceneID>,
        mut sceanime_cmd: Commands<GameObject, SceneAnimationContext>,
        mut objanime_cmd: Commands<GameObject, AnimationGroups>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                SceneCommand::Create(entity, left, right) => {
                    coordsys_cmd.insert(entity, SceneCoordinateSytem::default());
                    scenetime_cmd.insert(entity, SceneTime::new());
                    scenefog_cmd.insert(entity, SceneFog::new());
                    sceanime_cmd.insert(entity, SceneAnimationContext::new());
                    leftroot_cmd.insert(entity, TreeLeftRoot::new(left));
                    rightroot_cmd.insert(entity, TreeRightRoot::new(right));
                },
                SceneCommand::AddObject(entity, scene) => {
                    obj_cmd.insert(entity, scene);
                    objanime_cmd.insert(entity, AnimationGroups::default());
                },
            }
        });

    }
}
