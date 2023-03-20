use pi_ecs::prelude::{ResMut, Commands, Query};
use pi_engine_shell::object::{ObjectID, GameObject};
use pi_scene_context::materials::material::MaterialID;

use crate::base::{ShadowGeneratorParam, ShadowGeneratorDepthScale};

pub struct SingleCommands(pub Vec<ObjectID>);

pub struct SysCommand;
impl SysCommand {
    fn sys(
        mut cmds: ResMut<SingleCommands>,
        mut light_cmd: Commands<GameObject, MaterialID>,
        mut light_cmd: Commands<GameObject, ShadowGeneratorParam>,
        mut light_cmd2: Commands<GameObject, ShadowGeneratorDepthScale>,
    ) {

    }
}

pub struct SysUniform;
impl SysUniform {
    fn sys(
        lights: Query<GameObject, (&ShadowGeneratorParam, &ShadowGeneratorDepthScale), >
    ) {

    }
}
