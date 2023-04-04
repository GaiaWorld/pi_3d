use pi_atom::Atom;
use pi_engine_shell::prelude::*;

use crate::object::ObjectID;

use self::enable::SingleEnableCommands;

pub mod enable;


pub struct SceneID01;
pub struct SceneID02;
pub struct SceneID03;
pub struct SceneID04;

pub struct SceneCameraID01;
pub struct SceneCameraID02;
pub struct SceneCameraID03;
pub struct SceneCameraID04;
pub struct SceneCameraID05;
pub struct SceneCameraID06;

pub struct CullingFlag(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneID(pub ObjectID);
#[derive(Debug)]
pub struct CameraID(pub usize);

pub struct UniqueName(pub Atom);

pub struct Enable(pub bool);

pub struct PluginFlags;
impl Plugin for PluginFlags {
    fn build(&self, app: &mut App) {
        app.world.insert_resource(SingleEnableCommands::default());
    }
}