use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;

use crate::{iparticle_system_config::IParticleSystemConfig, base::ParticleSystemCalculatorID};


pub struct OpsCPUParticleCalculator(pub(crate) Entity, pub(crate) IParticleSystemConfig, pub(crate) u8);
impl OpsCPUParticleCalculator {
    pub fn ops(id: Entity, cfg: IParticleSystemConfig) -> Self {
        Self(id, cfg, 0)
    }
}
pub type ActionListCPUParticleCalculator = ActionList<OpsCPUParticleCalculator>;

pub struct OpsCPUParticleSystem(pub(crate) Entity, pub(crate) Entity, pub(crate) Handle<ParticleSystemCalculatorID>, pub(crate) usize, pub(crate) u8);
impl OpsCPUParticleSystem {
    pub fn ops(node: Entity, sourcemesh: Entity, calculator: Handle<ParticleSystemCalculatorID>, maxcount: usize) -> Self {
        Self(node, sourcemesh, calculator, maxcount, 0)
    }
}
pub type ActionListCPUParticleSystem = ActionList<OpsCPUParticleSystem>;