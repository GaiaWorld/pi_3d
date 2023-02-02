use std::mem::replace;

use pi_ecs::prelude::{ResMut, Commands};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_scene_math::Number;



#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: Number,
    pub y: Number,
    pub w: Number,
    pub h: Number,
}
impl Default for Viewport {
    fn default() -> Self {
        Self { x: 0., y: 0., w: 1., h: 1. }
    }
}

#[derive(Debug)]
pub enum EViewerCommand {
    Viewport(ObjectID, Viewport)
}

#[derive(Debug, Default)]
pub struct SingleViewerCommands(pub Vec<EViewerCommand>);

pub struct SysViewerCommand;
impl TSystemStageInfo for SysViewerCommand {}
#[setup]
impl SysViewerCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleViewerCommands>,
        mut viewport_cmd: Commands<GameObject, Viewport>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EViewerCommand::Viewport(obj, viewport) => {
                    viewport_cmd.insert(obj, viewport);
                },
            }
        });
    }
}