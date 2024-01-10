use pi_assets::asset::Handle;
use pi_scene_shell::prelude::*;

use crate::{iparticle_system_config::IParticleSystemConfig, base::{ParticleSystemCalculatorID, ParticleAttribute, ParticleAttributes}};


pub struct OpsCPUParticleCalculator(pub(crate) Entity, pub(crate) IParticleSystemConfig, pub(crate) u8);
impl OpsCPUParticleCalculator {
    pub fn ops(id: Entity, cfg: IParticleSystemConfig) -> Self {
        Self(id, cfg, 0)
    }
}
pub type ActionListCPUParticleCalculator = ActionList<OpsCPUParticleCalculator>;

pub struct OpsCPUParticleSystem(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) Handle<ParticleSystemCalculatorID>, pub(crate) ParticleAttributes, pub(crate) u8);
impl OpsCPUParticleSystem {
    pub fn ops(scene: Entity, node: Entity, trailmesh: Entity, trailgeo: Entity, calculator: Handle<ParticleSystemCalculatorID>, atrts: Vec<ParticleAttribute>) -> Self {
        Self(scene, node, trailmesh, trailgeo, calculator, ParticleAttributes(atrts), 0)
    }
}
pub type ActionListCPUParticleSystem = ActionList<OpsCPUParticleSystem>;

pub enum OpsCPUParticleSystemState {
    Start(Entity, u8),
    TimeScale(Entity, f32, u8),
    Stop(Entity, u8),
}
impl OpsCPUParticleSystemState {
    pub fn ops_start(entity: Entity) -> Self {
        Self::Start(entity, 0)
    }
    pub fn ops_speed(entity: Entity, speed: f32) -> Self {
        Self::TimeScale(entity, speed, 0)
    }
    pub fn ops_stop(entity: Entity) -> Self {
        Self::Stop(entity, 0)
    }
}
pub type ActionListCPUParticleSystemState = ActionList<OpsCPUParticleSystemState>;

pub struct OpsCPUParticleSystemTrailMaterial(pub(crate) Entity, pub(crate) Entity, pub(crate) PassTag, pub(crate) u8);
impl OpsCPUParticleSystemTrailMaterial {
    pub fn ops(node: Entity, mat: Entity, pass: PassTag) -> Self {
        Self(node, mat, pass, 0)
    }
}
pub type ActionListCPUParticleSystemTrailMaterial = ActionList<OpsCPUParticleSystemTrailMaterial>;
