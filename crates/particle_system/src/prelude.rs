
use pi_scene_shell::prelude::SystemParam;

pub use crate::tools::*;
pub use crate::base::*;
pub use crate::emitter::*;
pub use crate::modifier::*;
pub use crate::command::*;
pub use crate::extend::*;
pub use crate::iparticle_system_config::*;

use pi_scene_shell::prelude::*;

#[derive(SystemParam)]
pub struct ActionSetParticleSystem<'w> {
    pub calculator: ResMut<'w, ActionListCPUParticleCalculator>,
    pub create: ResMut<'w, ActionListCPUParticleSystem>,
    pub state: ResMut<'w, ActionListCPUParticleSystemState>,
    pub trailmaterial: ResMut<'w, ActionListCPUParticleSystemTrailMaterial>,
}
impl<'w> MemSize for ActionSetParticleSystem<'w> {
    fn memsize(&self) -> usize {
        self.calculator.memsize()
        + self.create.memsize()
        + self.state.memsize()
        + self.trailmaterial.memsize()
    }
}

#[derive(SystemParam)]
pub struct ResourceParticleSystem<'w> {
    pub calcultors: Res<'w, ShareAssetMgr<ParticleSystemCalculatorID>>,
    pub calculator_queue: Res<'w, ResParticleCalculatorUninstallQueue>,
}
impl<'w> MemSize for ResourceParticleSystem<'w> {
    fn memsize(&self) -> usize {
        self.calcultors.size()
        + self.calculator_queue.0.len() * 8 + 256
    }
}
