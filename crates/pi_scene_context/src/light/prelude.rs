
use pi_engine_shell::prelude::*;

use crate::viewer::prelude::TCullingPerformance;

use super::command::{ActionListLightCreate, ActionListLightParam};

pub use super::{
    command::*,
    base::*,
    shadow_generator::*
};

#[derive(Resource, Default)]
pub struct StateLight {
    pub culling_time: u32,
}
impl TCullingPerformance for StateLight {
    fn culling_time(&mut self, ms: u32) {
        self.culling_time = ms;
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageLighting {
    LightingCommand,
    LightingCommandApply,
    LightingRenderer,
    LightingCalcMatrix,
    LightingCulling,
}


#[derive(SystemParam)]
pub struct ActionSetLighting<'w> {
    pub create: ResMut<'w, ActionListLightCreate>,
    pub param: ResMut<'w, ActionListLightParam>,
}