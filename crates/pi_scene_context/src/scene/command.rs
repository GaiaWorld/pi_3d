use std::mem::replace;

use pi_ecs::{sys::system, prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;

use crate::{object::{ObjectID, GameObject}, flags::SceneID, resources::RenderDynUniformBuffer};

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
#[setup]
impl SysSceneCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleSceneCommandList>,
        mut scenes: Query<GameObject, (Write<SceneCoordinateSytem>, Write<SceneTime>)>,
        mut objects: Query<GameObject, Write<SceneID>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                SceneCommand::Create(entity) => {
                    match scenes.get_mut(entity) {
                        Some(mut scene) => {
                            scene.0.insert_no_notify(SceneCoordinateSytem::default());
                            scene.1.insert_no_notify(SceneTime::new(&mut dynbuffer));
                        },
                        None => todo!(),
                    }
                },
                SceneCommand::AddObject(entity, scene) => {
                    match objects.get_mut(entity) {
                        Some(mut object) => {
                            object.insert_no_notify(scene);
                        },
                        None => todo!(),
                    }
                },
            }
        });

    }
}
