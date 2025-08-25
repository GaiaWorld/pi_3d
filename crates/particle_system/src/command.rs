use pi_assets::asset::Handle;
use pi_scene_shell::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{iparticle_system_config::IParticleSystemConfig, base::{ParticleSystemCalculatorID, ParticleAttribute, ParticleAttributes}};


pub struct OpsCPUParticleCalculator(pub(crate) Entity, pub(crate) IParticleSystemConfig);
impl OpsCPUParticleCalculator {
    pub fn ops(id: Entity, cfg: IParticleSystemConfig) -> Self {
        Self(id, cfg)
    }
}
pub type ActionListCPUParticleCalculator = ActionList<OpsCPUParticleCalculator>;

pub struct OpsCPUParticleSystem(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) Handle<ParticleSystemCalculatorID>, pub(crate) ParticleAttributes, pub(crate) u8);
impl OpsCPUParticleSystem {
    pub fn ops(scene: Entity, node: Entity, trailmesh: Entity, trailgeo: Entity, calculator: Handle<ParticleSystemCalculatorID>, atrts: Vec<ParticleAttribute>, update_buffer_interval_frame: u8) -> Self {
        Self(scene, node, trailmesh, trailgeo, calculator, ParticleAttributes(atrts), update_buffer_interval_frame)
    }
}
pub type ActionListCPUParticleSystem = ActionList<OpsCPUParticleSystem>;

#[derive(Deserialize, Serialize)]
pub enum ECPUParticleSystemState {
    Start(),
    TimeScale( f32),
    Stop(),
}

pub struct OpsCPUParticleSystemState(pub(crate) Entity, pub (crate) ECPUParticleSystemState);
impl OpsCPUParticleSystemState {
    pub fn ops(entity: Entity, val: ECPUParticleSystemState) -> Self {
        Self(entity, val)
    }
    pub fn ops_start(entity: Entity) -> Self {
        Self(entity, ECPUParticleSystemState::Start())
    }
    pub fn ops_speed(entity: Entity, speed: f32) -> Self {
        Self(entity, ECPUParticleSystemState::TimeScale( speed))
    }
    pub fn ops_stop(entity: Entity) -> Self {
        Self(entity, ECPUParticleSystemState::Stop())
    }
}
pub type ActionListCPUParticleSystemState = ActionList<OpsCPUParticleSystemState>;

pub struct OpsCPUParticleSystemTrailMaterial(pub(crate) Entity, pub(crate) Entity, pub(crate) PassTag);
impl OpsCPUParticleSystemTrailMaterial {
    pub fn ops(node: Entity, mat: Entity, pass: PassTag) -> Self {
        Self(node, mat, pass)
    }
}
pub type ActionListCPUParticleSystemTrailMaterial = ActionList<OpsCPUParticleSystemTrailMaterial>;
