use bevy::{prelude::{ResMut, Commands, Query}, ecs::entity};
use pi_scene_math::{Vector3, Vector4, Matrix};

use crate::{command::*, base::*, extend::format};

pub fn sys_particle_calculator(
    mut cmds: ResMut<ActionListCPUParticleCalculator>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleCalculator(entity, cfg, count)| {
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else { return; };

        // log::warn!("particle_calculator");
        format(&mut entitycmd, &cfg);
    });
}

pub fn sys_create_cpu_partilce_system(
    mut cmds: ResMut<ActionListCPUParticleSystem>,
    mut commands: Commands,
    mut calculators: Query<&ParticleCalculatorBase>,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleSystem(entity, sourcemesh, calculator, maxcount, count)| {
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            // log::warn!("create_cpu_partilce_system CANNT");
            return;
        };

        if let Ok(base) = calculators.get(calculator.0) {
            log::warn!("create_cpu_partilce_system");
            let maxcount = base.maxcount;
            let mut vec_vec3_arr: Vec<Vec<Vector3>> = Vec::with_capacity(maxcount);
            for _ in 0..maxcount {
                vec_vec3_arr.push(vec![]);
            }

            entitycmd
                .insert(ParticleRandom::new(0))
                .insert(ParticleSystemTime::new())
                .insert(ParticleSystemEmission::new())
                .insert(ParticleIDs::new(calculator, maxcount))
                .insert(ParticleBaseRandom::new(maxcount))
                .insert(ParticleAgeLifetime::new(maxcount))
                .insert(ParticleStartColor::new(maxcount))
                .insert(ParticleStartScaling::new(maxcount))
                // .insert(ParticleStartRotation::new(maxcount))
                .insert(ParticleLocalPosition::new(maxcount))
                .insert(ParticleLocalRotation::new(maxcount))
                .insert(ParticleLocalScaling::new(maxcount))
                .insert(ParticleColor::new(maxcount))
                .insert(ParticleEmitMatrix::new(maxcount))
                .insert(ParticleGravityFactor::new(maxcount))
                .insert(ParticleForce::new(maxcount))
                .insert(ParticleVelocity::new(maxcount))
                .insert(ParticleSpeedFactor::new(maxcount))
                .insert(ParticleOrbitVelocity::new(maxcount))
                .insert(ParticleLimitVelocityScalar::new(maxcount))
                .insert(ParticleDirection::new(maxcount))
                .insert(ParticleUV::new(maxcount))
                .insert(ParticleCustomV4::new(maxcount))
                .insert(ParticleGlobalPosList(vec_vec3_arr.clone()))
                .insert(ParticleLocalPosList(vec_vec3_arr))
                ;
        } else if count < 1 {
            // log::warn!("create_cpu_partilce_system FAIL");
            cmds.push(OpsCPUParticleSystem(entity, sourcemesh, calculator, maxcount, count + 1));
        }

    });
}