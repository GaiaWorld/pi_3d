
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
pub struct ParticleSystemActionSet<'w> {
    pub calcultors: Res<'w, ShareAssetMgr<ParticleSystemCalculatorID>>,
    pub calculator_queue: Res<'w, ResParticleCalculatorUninstallQueue>,
    pub calculator_cmds: ResMut<'w, ActionListCPUParticleCalculator>,
    pub particlesys_cmds: ResMut<'w, ActionListCPUParticleSystem>,
    pub particlesys_state_cmds: ResMut<'w, ActionListCPUParticleSystemState>,
}