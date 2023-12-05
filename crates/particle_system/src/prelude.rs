
use pi_engine_shell::prelude::SystemParam;

pub use crate::tools::*;
pub use crate::base::*;
pub use crate::emitter::*;
pub use crate::modifier::*;
pub use crate::command::*;
pub use crate::extend::*;
pub use crate::iparticle_system_config::*;

use pi_engine_shell::prelude::*;

#[derive(SystemParam)]
pub struct ActionSetParticleSystem<'w> {
    pub calculator: ResMut<'w, ActionListCPUParticleCalculator>,
    pub create: ResMut<'w, ActionListCPUParticleSystem>,
    pub state: ResMut<'w, ActionListCPUParticleSystemState>,
    pub trailmaterial: ResMut<'w, ActionListCPUParticleSystemTrailMaterial>,
}

#[derive(SystemParam)]
pub struct ResourceParticleSystem<'w> {
    pub calcultors: Res<'w, ShareAssetMgr<ParticleSystemCalculatorID>>,
    pub calculator_queue: Res<'w, ResParticleCalculatorUninstallQueue>,
}