
use std::sync::Arc;

use pi_scene_shell::prelude::*;
use pi_scene_context::{prelude::*, geometry::instance::{instanced_buffer::{InstancedInfo, InstanceBufferAllocator}, types::ModelInstanceAttributes}};
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolVector3, TToolRotation}};

use crate::base::*;

pub fn sys_particle_active(
    mut items: Query<(&GlobalEnable, &ParticleActive, &mut ParticleRunningState, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleSystemEmission, &mut MeshInstanceState), Or<(Changed<GlobalEnable>, Changed<ParticleActive>)>>,
    performance: Res<ParticleSystemPerformance>,
    mut globalperformance: ResMut<Performance>,
) {
    // let time0 = pi_time::Instant::now();
    items.iter_mut().for_each(|(enable, active, mut state, mut ids, mut time, mut emission, mut instancestate)| {
        if enable.0 == true && active.0 == true {
            if state.0 == false {
                instancestate.use_single_instancebuffer = true;
                state.0 = true;

                ids.reset();
                let timescale = time.time_scale;
                *time = ParticleSystemTime::new(performance.frame_time_ms); time.time_scale = timescale;
                *emission = ParticleSystemEmission::new();
            }
        } else {
            state.0 = false;
        }
    });

    globalperformance.particlesystem = performance.total();
}


pub fn sys_prewarm(
    mut items: Query<
        (
            (&DisposeReady, &ParticleRunningState, &LocalScaling, &GlobalMatrix, &mut ParticleGravityFactor, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleModifyState),
            (&mut ParticleSystemEmission, &mut ParticleRandom, &mut ParticleBaseRandom, &mut ParticleEmitMatrix, &mut AbsoluteTransform, &mut ParticleLocalPosition, &mut ParticleDirection),
            (&mut ParticleAgeLifetime, &mut ParticleDieWaitTime, &mut ParticleStartScaling, &mut ParticleLocalScaling, &mut ParticleLocalRotation, &mut ParticleStartColor, &mut ParticleColorAndUV),
            (&mut ParticleForce, &mut ParticleVelocity, &mut ParticleSpeedFactor, &mut ParticleLimitVelocityScalar, &mut ParticleOrbitOffset, &mut ParticleOrbitVelocity, &mut ParticleOrbitRadial, Option<&mut ParticleTrail>)
        ),
        Changed<ParticleRunningState>
    >,
    calculators: Query<(
        &ParticleCalculatorBase, &ParticleCalculatorStartModifiers, &ParticleCalculatorOverLifetime
    )>,
    calculators_trail:  Query<&ParticleCalculatorTrail>,
) {
    items.iter_mut().for_each(|(
        (disposestate, state, localscl, gmatrix, mut gravities, mut ids, mut time, mut modifystate),
        (mut emission, mut random, mut randoms, mut emitmatrix, mut abstransform, mut locpos, mut directions),
        (mut ages, mut diewaittimes, mut startscl, mut plocscl, mut plocrot, mut startcol, mut colorsanduvs),
        (mut forces, mut velocity, mut speed, mut limitvelocty, mut orbitoffset, mut orbitvelocity, mut orbitradial, mut trails)
    )| {
        if let Ok((
            base, modifiers, overlifetime
        )) = calculators.get(ids.calculator.0) {
            let delta_ms = 66 as u32;

            if state.0 && disposestate.0 == false && base.prewarm {
                // log::error!("Prewarm!!!");
                let global_position = Vector3::zeros();

                let pretime = base.duration;
                let timescale = time.time_scale;
                time.time_scale = 1.;
                let mut runtime = 0;
                while runtime < pretime {
                    runtime += delta_ms;
                    time.run(delta_ms, 1000, base.duration);

                    fn_emission(base, &modifiers.emission, &mut random, &mut ids, &mut time, &mut emission, &mut randoms, &mut modifystate);
                    fn_emitmatrix(localscl, gmatrix, &ids, &mut emitmatrix, &mut abstransform, &global_position);
                    fn_emitter(&modifiers.shapeemitter, &modifiers.startspeed, &mut locpos, &mut directions, &ids, &time, &randoms);
                    if let (Ok(trailmodifier), Some(trails)) = (calculators_trail.get(ids.calculator.0), trails.as_deref_mut()) {
                        fn_start_lifetime(&modifiers.startlifetime, &ids, &time, &randoms, &mut ages, &mut diewaittimes, Some(trailmodifier), Some(trails));
                    } else {
                        fn_start_lifetime(&modifiers.startlifetime, &ids, &time, &randoms, &mut ages, &mut diewaittimes, None, None);
                    }
                    fn_gravity(&modifiers.gravity, &ages, &ids, &time, &emitmatrix, &randoms, &mut gravities);
                    fn_start_size(&modifiers.startsize, &ids, &time, &randoms, &mut startscl, &mut plocscl);
                    fn_start_rotation(&modifiers.startrotation, &ids, &time, &randoms, &mut plocrot);
                    fn_start_color(&modifiers.startcolor, &ids, &time, &randoms, &mut startcol, &mut colorsanduvs.color);
                    if let Some(calculator) = &overlifetime.texturesheet {
                        fn_start_texture_sheet(calculator, &ids, &randoms, &mut colorsanduvs.uv);
                        fn_texturesheet(calculator, &ids, &ages, &randoms, &mut colorsanduvs.uv);
                    }

                    fn_gravity(&modifiers.gravity, &ages, &ids, &time, &emitmatrix, &randoms, &mut gravities);
                    if let Some(calculator) = &overlifetime.color {
                        fn_color_over_life_time(calculator, &ids, &ages, &randoms, &startcol, &mut colorsanduvs.color);
                    }
                    if let Some(calculator) = &overlifetime.rotation {
                        fn_rotation_over_life_time(calculator, &ids, &time, &ages, &randoms, &mut plocrot);
                    }
                    if let Some(calculator) = &overlifetime.size {
                        fn_size_over_life_time(calculator, &ids, &time, &ages, &randoms, &startscl, &mut plocscl);
                    }
                    if let Some(calculator) = &overlifetime.velocity {
                        fn_velocity_over_life_time(calculator, &ids, &time, &ages, &randoms, &mut velocity);
                    }
                    if let Some(calculator) = &overlifetime.limitvelocity {
                        fn_limit_velocity_over_life_time(calculator, &ids, &time, &ages, &randoms, &mut limitvelocty);
                    }
                    fn_force_over_life_time(&overlifetime.force, &ids, &time, &ages, &emitmatrix, &randoms, &mut forces);
                    fn_orbit_over_life_time(&overlifetime.orbitoffset, &overlifetime.orbitvelocity, &overlifetime.orbitradial, &ids, &ages, &randoms, &mut orbitoffset, &mut orbitvelocity, &mut orbitradial);
                    if let Some(calculator) = &overlifetime.speed {
                        fn_speed_modifier_over_life_time(calculator, &ids, &time, &ages, &randoms, &mut speed);
                    }
                    fn_direction(&modifiers.shapeemitter, &ids, &time, &velocity, &gravities, &forces, &speed, &limitvelocty, &orbitoffset, &orbitvelocity, &orbitradial, &mut directions, &mut locpos);

                    if let Some(calculator) = &overlifetime.sizebyspeed {
                        fn_size_by_speed(calculator, &ids, &time, &directions, &randoms, &mut plocscl);
                    }
                    if let Some(calculator) = &overlifetime.colorbyspeed {
                        fn_color_by_speed(calculator, &ids, &time, &directions, &randoms, &mut colorsanduvs.color);
                    }
                    if let Some(calculator) = &overlifetime.rotationbyspeed {
                        fn_rotation_by_speed(calculator, &ids, &time, &directions, &randoms, &mut plocrot);
                    }

                    fn_ids(&mut ids, &ages, &time, &diewaittimes);
                }
                time.time_scale = timescale;
            }
        }
    });
}

/// 系统的启动
pub fn sys_emission(
    scenes: Query<&SceneTime>,
    calculators: Query<(&ParticleCalculatorBase, &ParticleCalculatorStartModifiers)>,
    mut particle_sys: Query<(&SceneID, &DisposeReady, &ParticleRunningState, &mut ParticleRandom, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleSystemEmission, &mut ParticleBaseRandom, &mut ParticleModifyState)>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(idscene, disposestate, state, mut random, mut ids, mut particlesystime, mut emissiondata, mut randoms, mut modifystate)| {
        if let (Ok(scenetime), Ok((base, calcemission))) = (scenes.get(idscene.0), calculators.get(ids.calculator.0)) {

            let delta_ms = scenetime.delta_ms() as u32;

            // log::warn!("{:?}, {:?}, {:?}, ", delta_ms, state.playing, disposestate.0);

            if state.0 && disposestate.0 == false {
                particlesystime.run(delta_ms, 1000, base.duration);
            } else {
                particlesystime.run(0, 1000, base.duration);
            }

            // log::warn!("Emission: {:?}, {:?}, ", delta_ms, particlesystime.running_delta_ms);

            // 间隔时间到达帧运行间隔
            if particlesystime.running_delta_ms > 0 {
                fn_emission(
                    &base, &calcemission.emission,
                    &mut random, &mut ids, &mut particlesystime, &mut emissiondata,
                    &mut randoms, &mut modifystate
                );
            }
        }
    });
    performance.sys_emission = (pi_time::Instant::now() - time0).as_micros() as u32;
}
fn fn_emission(
    base: &ParticleCalculatorBase, calcemission: &ParticleCalculatorEmission,
    random: &mut ParticleRandom, ids: &mut ParticleIDs, particlesystime: &mut ParticleSystemTime,
    emission: &mut ParticleSystemEmission, randoms: &mut ParticleBaseRandom, modifystate: &mut ParticleModifyState,
) {
    *modifystate = ParticleModifyState;

    let rate_over_time = calcemission.rateovertime.interpolate(particlesystime.emission_progress, random.random()) as usize;
    // log::warn!("Emission Rate: {:?}, ", rate_over_time);
    emission.start(
        base.looping, base.duration,
        &particlesystime, rate_over_time,
        &calcemission.bursts,
        ids
    );

    let newids = &ids.newids;
    let activeids = &ids.actives;

    randoms.run(newids, activeids, random);
}

pub fn sys_emitmatrix(
    mut particle_sys: Query<(&LocalScaling, &GlobalMatrix, &ParticleIDs, &ParticleSystemTime, &mut ParticleEmitMatrix, &mut AbsoluteTransform), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    let global_position = Vector3::zeros();
    particle_sys.iter_mut().for_each(|(local_scaling, transform, ids, time, mut emitmatrixdata, mut absolute)| {
        if time.running_delta_ms <= 0 { return; }
        
        fn_emitmatrix(
            local_scaling, transform, ids,
            &mut emitmatrixdata, &mut absolute,
            &global_position
        );
    });
    performance.sys_emitmatrix = (pi_time::Instant::now() - time0).as_micros() as u32;
}
fn fn_emitmatrix(
    local_scaling: &LocalScaling, transform: &GlobalMatrix, ids: &ParticleIDs,
    emitmatrixdata: &mut ParticleEmitMatrix, absolute: &mut AbsoluteTransform,
    global_position: &Vector3
) {
    let newids = &ids.newids;
    let activeids = &ids.actives;

    let global_scaling = absolute.scaling(transform.matrix()).clone();
    // let global_position = transform.position().clone();
    // log::warn!("Position: {:?} {:?}", &localpos.0, global_position);

    let iso = absolute.iso(transform.matrix());
    let global_rotation = absolute.rotation(transform.matrix());

    emitmatrixdata.emit(
        newids, activeids, &transform.matrix, &transform.matrix_inv, &iso, global_position, global_rotation, &global_scaling,
        &local_scaling.0
    );
}

pub fn sys_emitter(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleLocalPosition, &mut ParticleDirection), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    // let time = pi_time::Instant::now();

    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut locpos, mut directions)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(startspeed) = calculators.get(ids.calculator.0) {
            fn_emitter(&startspeed.shapeemitter, &startspeed.startspeed, &mut locpos, &mut directions, &ids, &time, &randoms);
            // let emitter = &emitter.0;
            // let newids = &ids.newids;
            // // let activeids = &ids.actives;

            // locpos.start(newids, &mut directions, randoms, time, emitter, startspeed);
        }
    });
    
    performance.sys_emitter = (pi_time::Instant::now() - time0).as_micros() as u32;
    // let time1 = pi_time::Instant::now();
    // log::warn!("emitter: {:?}", time1 - time);
}
fn fn_emitter(
    emitter: &ParticleCalculatorShapeEmitter, startspeed: &ParticleCalculatorStartSpeed,
    locpos: &mut ParticleLocalPosition, directions: &mut ParticleDirection, ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom
) {
    let emitter = &emitter.0;
    let newids = &ids.newids;
    // let activeids = &ids.actives;

    locpos.start(newids, directions, randoms, time, emitter, startspeed);
}

pub fn sys_start_lifetime(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    mut particle_sys: Query<(Entity, &ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleAgeLifetime, &mut ParticleDieWaitTime), Changed<ParticleModifyState>>,
    calculators_trail: Query<&ParticleCalculatorTrail>,
    mut particle_sys_trail: Query<&mut ParticleTrail>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(entity, ids, time, randoms, mut items, mut diewaittimes)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let (Ok(trailmodifier), Ok(mut trails)) = (calculators_trail.get(ids.calculator.0), particle_sys_trail.get_mut(entity)) {
                fn_start_lifetime(&calculator.startlifetime, &ids, &time, &randoms, &mut items, &mut diewaittimes, Some(trailmodifier), Some(&mut trails));
            } else {
                fn_start_lifetime(&calculator.startlifetime, &ids, &time, &randoms, &mut items, &mut diewaittimes, None, None);
            }
            // let calculator = &calculator.0;
            // let newids = &ids.newids;
            // items.start(time, newids, calculator, randoms);

            // if let (Ok(trailmodifier), Ok(mut trails)) = (calculators_trail.get(ids.calculator.0), particle_sys_trail.get_mut(entity)) {
            //     trails.start(newids, &items, &mut diewaittimes.0, &randoms, time, &trailmodifier.0);
            // } else {
            //     diewaittimes.start(newids, &items, randoms, time, None);
            // }
        }
    });
    performance.sys_start_lifetime = (pi_time::Instant::now() - time0).as_micros() as u32;
}
fn fn_start_lifetime(
    calculator: &ParticleCalculatorStartLifetime, 
    ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom, items: &mut ParticleAgeLifetime, diewaittimes: &mut ParticleDieWaitTime,
    trailmodifier: Option<&ParticleCalculatorTrail>, trails: Option<&mut ParticleTrail>
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    items.start(time, newids, calculator, randoms);

    if let (Some(trailmodifier), Some(trails)) = (trailmodifier, trails) {
        trails.start(newids, &items, &mut diewaittimes.0, &randoms, time, &trailmodifier.0);
    } else {
        diewaittimes.start(newids, &items, randoms, time, None);
    }
}

pub fn sys_start_size(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleStartScaling, &mut ParticleLocalScaling), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items, mut localscalings)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            fn_start_size(&calculator.startsize, ids, time, randoms, &mut items, &mut localscalings);
            // let calculator = &calculator.0;
            // let newids = &ids.newids;
            // // let activeids = &ids.actives;
            // items.start(newids, &mut localscalings, &randoms, time, calculator, );
        }
    });
    performance.sys_start_size = (pi_time::Instant::now() - time0).as_micros() as u32;
}
fn fn_start_size(
    calculator: &ParticleCalculatorStartSize,
    ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom, items: &mut ParticleStartScaling, localscalings: &mut ParticleLocalScaling
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    // let activeids = &ids.actives;
    items.start(newids, localscalings, &randoms, time, calculator, );
}

pub fn sys_start_rotation(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleLocalRotation), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            fn_start_rotation(&calculator.startrotation, ids, time, randoms, &mut items);
            // let calculator = &calculator.0;
            // let newids = &ids.newids;
            // items.start(newids, &randoms, time, calculator);
        }
    });
    performance.sys_start_rotation = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_start_rotation(
    calculator: &ParticleCalculatorStartRotation,
    ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom, items: &mut ParticleLocalRotation
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    items.start(newids, &randoms, time, calculator);
}

pub fn sys_start_color(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleStartColor, &mut ParticleColorAndUV), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items, mut colors)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            fn_start_color(&calculator.startcolor, ids, time, randoms, &mut items, &mut colors.color);
            // let calculator = &calculator.0;
            // let newids = &ids.newids;
            // items.start(newids, &mut colors, &randoms, time, calculator);
        }
    });
    performance.sys_start_color = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_start_color(
    calculator: &ParticleCalculatorStartColor,
    ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom, items: &mut ParticleStartColor, colors: &mut ParticleColor
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    items.start(newids, &mut colors.0, &randoms, time, calculator);
}

pub fn sys_gravity(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleEmitMatrix, &ParticleBaseRandom, &mut ParticleGravityFactor), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, emitmatrixs, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            fn_gravity(&calculator.gravity, ages, ids, time, emitmatrixs, randoms, &mut items);
        }
    });
    performance.sys_gravity = (pi_time::Instant::now() - time0).as_micros() as u32;
}
fn fn_gravity(calculator: &ParticleCalculatorGravity, ages: &ParticleAgeLifetime, ids: &ParticleIDs, time: &ParticleSystemTime, emitmatrixs: &ParticleEmitMatrix, randoms: &ParticleBaseRandom, items: &mut ParticleGravityFactor) {
    items.run(&ids.actives, &ages.0, emitmatrixs, &randoms.0, time, calculator);
}


pub fn sys_force_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleEmitMatrix, &ParticleBaseRandom, &mut ParticleForce), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, emitmatrixs, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            fn_force_over_life_time(&calculator.force, ids, time, ages, emitmatrixs, randoms, &mut items);
            // let calculator = &calculator.0;
            // // let newids = &ids.newids;
            // let activeids = &ids.actives;
            // items.run(activeids, ages, randoms, time, calculator);
        }
    });
    performance.sys_force_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_force_over_life_time(
    calculator: &ParticleCalculatorForceOverLifetime,
    ids: &ParticleIDs, time: &ParticleSystemTime, ages: &ParticleAgeLifetime, emitmatrixs: &ParticleEmitMatrix, randoms: &ParticleBaseRandom, items: &mut ParticleForce,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, emitmatrixs, randoms, time, calculator);
}

pub fn sys_start_texture_sheet(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleColorAndUV), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            
            if let Some(calculator) = &calculator.texturesheet { fn_start_texture_sheet(calculator, ids, randoms, &mut items.uv); }
            // let calculator = &calculator.0;
            // let newids = &ids.newids;
            // items.start(newids, &randoms, calculator);
        }
    });
    performance.sys_start_texture_sheet = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_start_texture_sheet(
    calculator: &ParticleCalculatorTextureSheet,
    ids: &ParticleIDs, randoms: &ParticleBaseRandom, items: &mut ParticleUV
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    items.start(newids, &randoms, calculator);
}

/// =================================== over life time
pub fn sys_color_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &ParticleStartColor, &mut ParticleColorAndUV), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, startcolors, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.color { fn_color_over_life_time(calculator, ids, ages, randoms, startcolors, &mut items.color); }
            // let calculator = &calculator.0;
            // // let newids = &ids.newids;
            // let activeids = &ids.actives;
            // items.run(activeids, ages, startcolors, randoms, calculator);
        }
    });
    performance.sys_color_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_color_over_life_time(
    calculator: &ParticleCalculatorColorOverLifetime,
    ids: &ParticleIDs, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, startcolors: &ParticleStartColor, items: &mut ParticleColor,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, startcolors, randoms, calculator);
}

pub fn sys_rotation_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleLocalRotation), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.rotation { fn_rotation_over_life_time(calculator, ids, time, ages, randoms, &mut items); }
            // let calculator = &calculator.0;
            // // let newids = &ids.newids;
            // let activeids = &ids.actives;
            // items.run(activeids, ages, randoms, time, calculator);
        }
    });
    performance.sys_rotation_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_rotation_over_life_time(
    calculator: &ParticleCalculatorRotationOverLifetime,
    ids: &ParticleIDs, time: &ParticleSystemTime, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, items: &mut ParticleLocalRotation,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, randoms, time, calculator);
}

pub fn sys_size_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &ParticleStartScaling, &mut ParticleLocalScaling), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, startsizes, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.size { fn_size_over_life_time(calculator, ids, time, ages, randoms, startsizes, &mut items); }
            // let calculator = &calculator.0;
            // // let newids = &ids.newids;
            // let activeids = &ids.actives;
            // items.run(activeids, ages, &startsizes.0, randoms, calculator);
        }
    });
    performance.sys_size_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_size_over_life_time(
    calculator: &ParticleCalculatorSizeOverLifetime,
    ids: &ParticleIDs, _time: &ParticleSystemTime, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, startsizes: &ParticleStartScaling, items: &mut ParticleLocalScaling,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, &startsizes.0, randoms, calculator);
}

pub fn sys_velocity_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleVelocity), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.velocity { fn_velocity_over_life_time(calculator, ids, time, ages, randoms, &mut items); }
            // let calculator = &calculator.0;
            // // let newids = &ids.newids;
            // let activeids = &ids.actives;
            // items.run(activeids, ages, randoms, time, calculator);
        }
    });
    performance.sys_velocity_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_velocity_over_life_time(
    calculator: &ParticleCalculatorVelocityOverLifetime,
    ids: &ParticleIDs, time: &ParticleSystemTime, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, items: &mut ParticleVelocity,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, randoms, time, calculator);
}

pub fn sys_orbit_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleOrbitOffset, &mut ParticleOrbitVelocity, &mut ParticleOrbitRadial), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items, mut items2, mut items3)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculators) = calculators.get(ids.calculator.0) {
            fn_orbit_over_life_time(&calculators.orbitoffset, &calculators.orbitvelocity, &calculators.orbitradial, ids, ages, randoms, &mut items, &mut items2, &mut items3);
        }

        // // let newids = &ids.newids;
        // let activeids = &ids.actives;
        // items.run(activeids, ages, randoms, time, offset, velocity, radial);
    });
    performance.sys_orbit_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_orbit_over_life_time(
    offset: &ParticleCalculatorOrbitOffset,
    velocity: &ParticleCalculatorOrbitVelocity,
    radial: &ParticleCalculatorOrbitRadial,
    ids: &ParticleIDs, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, items: &mut ParticleOrbitOffset, items2: &mut ParticleOrbitVelocity, items3: &mut ParticleOrbitRadial,
) {
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, randoms, offset);
    items2.run(activeids, ages, randoms, velocity);
    items3.run(activeids, ages, randoms, radial);
}

pub fn sys_speed_modifier_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleSpeedFactor), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.speed { fn_speed_modifier_over_life_time(calculator, ids, time, ages, randoms, &mut items); }
            // let calculator = &calculator.0;
            // // let newids = &ids.newids;
            // let activeids = &ids.actives;
            // items.run(activeids, ages, randoms, time, calculator);
        }
    });
    performance.sys_speed_modifier_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_speed_modifier_over_life_time(
    calculator: &ParticleCalculatorSpeedModifier,
    ids: &ParticleIDs, time: &ParticleSystemTime, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, items: &mut ParticleSpeedFactor,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, randoms, time, calculator);
}

pub fn sys_limit_velocity_over_life_time(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleLimitVelocityScalar), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.limitvelocity { fn_limit_velocity_over_life_time(calculator, ids, time, ages, randoms, &mut items); }
            // let calculator = &calculator.0;
            // // let newids = &ids.newids;
            // let activeids = &ids.actives;
            // items.run(activeids, ages, randoms, time, calculator);
        }
    });
    performance.sys_limit_velocity_over_life_time = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_limit_velocity_over_life_time(
    calculator: &ParticleCalculatorLimitVelocityOverLifetime,
    ids: &ParticleIDs, time: &ParticleSystemTime, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, items: &mut ParticleLimitVelocityScalar,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, randoms, time, calculator);
}

pub fn sys_direction(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    mut particle_sys: Query<(
        &ParticleIDs, &ParticleSystemTime,
        &ParticleVelocity, &ParticleGravityFactor, &ParticleForce, 
        &ParticleOrbitOffset, &ParticleOrbitVelocity, &ParticleOrbitRadial,
        &ParticleSpeedFactor, &ParticleLimitVelocityScalar,
        &mut ParticleDirection, &mut ParticleLocalPosition
    ), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(
        |(
            ids, time,
            velocities, gravities, forces,
            orbitsoffset, orbitsvelocity, orbitsradial,
            speedfactors, limitscalars,
            mut direction, mut positions
        )| {
            if time.running_delta_ms <= 0 { return; }

            if let Ok(calculator) = calculators.get(ids.calculator.0) {
                fn_direction(&calculator.shapeemitter, ids, time, velocities, gravities, forces, speedfactors, limitscalars, orbitsoffset, orbitsvelocity, orbitsradial, &mut direction, &mut positions);
                // let emitter = &calculator.0;
                // // let newids = &ids.newids;
                // let activeids = &ids.actives;
                // direction.run(activeids, forces, gravities, velocities, limitscalars, orbits, speedfactors, &mut positions, emitter, time);
            }
        }
    );
    performance.sys_direction = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_direction(
    calculator: &ParticleCalculatorShapeEmitter,
    ids: &ParticleIDs, time: &ParticleSystemTime,
    velocities: &ParticleVelocity, gravities: &ParticleGravityFactor, forces: &ParticleForce, speedfactors: &ParticleSpeedFactor, limitscalars: &ParticleLimitVelocityScalar,
    orbitsoffset: &ParticleOrbitOffset, orbitsvelocity: &ParticleOrbitVelocity, orbitsradial: &ParticleOrbitRadial,
    direction: &mut ParticleDirection, positions: &mut ParticleLocalPosition
) {
    let emitter = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    direction.run(activeids, forces, gravities, velocities, limitscalars, orbitsvelocity, orbitsoffset, orbitsradial, speedfactors, positions, emitter, time);
}

// 
pub fn sys_color_by_speed(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleDirection, &ParticleBaseRandom, &mut ParticleColorAndUV), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, directions, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.colorbyspeed { fn_color_by_speed(calculator, ids, time, directions, randoms, &mut items.color); }
            // let calculator = &calculator.0;
            // let activeids = &ids.actives;
            // items.speed(activeids, directions, randoms, calculator);
        }
    });
    performance.sys_color_by_speed = (pi_time::Instant::now() - time0).as_micros() as u32;
}

pub fn fn_color_by_speed(
    calculator: &ParticleCalculatorColorBySpeed,
    ids: &ParticleIDs, time: &ParticleSystemTime, directions: &ParticleDirection, randoms: &ParticleBaseRandom, items: &mut ParticleColor
) {
    let calculator = &calculator.0;
    let activeids = &ids.actives;
    items.speed(activeids, directions, randoms, calculator);
}

// 
pub fn sys_size_by_speed(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleDirection, &ParticleBaseRandom, &mut ParticleLocalScaling), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, directions, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.sizebyspeed { fn_size_by_speed(calculator, ids, time, directions, randoms, &mut items); }
            // let calculator = &calculator.0;
            // let activeids = &ids.actives;
            // items.speed(activeids, directions, randoms, calculator);
        }
    });
    performance.sys_size_by_speed = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_size_by_speed(
    calculator: &ParticleCalculatorSizeBySpeed,
    ids: &ParticleIDs, time: &ParticleSystemTime, directions: &ParticleDirection, randoms: &ParticleBaseRandom, items: &mut ParticleLocalScaling
) {
    let calculator = &calculator.0;
    let activeids = &ids.actives;
    items.speed(activeids, directions, randoms, calculator);
}

// 
pub fn sys_rotation_by_speed(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleDirection, &ParticleBaseRandom, &mut ParticleLocalRotation), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(ids, time, directions, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            if let Some(calculator) = &calculator.rotationbyspeed { fn_rotation_by_speed(calculator, ids, time, directions, randoms, &mut items); }
            // let calculator = &calculator.0;
            // let activeids = &ids.actives;
            // items.speed(activeids, directions, randoms, time, calculator);
        }
    });
    performance.sys_rotation_by_speed = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_rotation_by_speed(
    calculator: &ParticleCalculatorRotationBySpeed,
    ids: &ParticleIDs, time: &ParticleSystemTime, directions: &ParticleDirection, randoms: &ParticleBaseRandom, items: &mut ParticleLocalRotation
) {
    let calculator = &calculator.0;
    let activeids = &ids.actives;
    items.speed(activeids, directions, randoms, time, calculator);
}

pub fn sys_ids(
    mut particle_sys: Query<(&mut ParticleIDs, &ParticleAgeLifetime, &ParticleSystemTime, &ParticleDieWaitTime), Changed<ParticleModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(|(mut ids, ages, time, diewaittimes)| {
        if time.running_delta_ms <= 0 { return; }
        fn_ids(&mut ids, ages, time, diewaittimes);
    });
    performance.sys_ids = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_ids(
    ids: &mut ParticleIDs, ages: &ParticleAgeLifetime, _time: &ParticleSystemTime, diewaittimes: &ParticleDieWaitTime
) {
    ids.newids.clear();

    let items = [ids.actives.clone(), ids.dies.clone()].concat();

    ids.actives.clear();
    ids.dies.clear();
    items.iter().for_each(|idx| {
        let age = ages.get(*idx).unwrap();
        let diewait = diewaittimes.0.get(*idx).unwrap();
        // log::warn!("Age: {:?}, Lifetime: {:?}", age.age, age.lifetime);
        if age.age <= age.lifetime {
            ids.actives.push(*idx);
        } else if age.age < age.lifetime + diewait {
            ids.dies.push(*idx);
        } else {
            ids.unactives.push(*idx);
        }
    });
    // ids.dies.clone().drain(..).for_each(|idx| {
    //     let age = ages.get(idx).unwrap();
    //     let diewait = diewaittimes.0.get(idx).unwrap();
    //     // log::warn!("Age: {:?}, Lifetime: {:?}", age.age, age.lifetime);
    //     if age.age <= age.lifetime {
    //         ids.actives.push(idx);
    //     } else if age.age < age.lifetime + diewait {
    //         ids.dies.push(idx);
    //     } else {
    //         ids.unactives.push(idx);
    //     }
    // });

    // log::warn!("actives: {:?}", ids.actives);
}

pub fn sys_texturesheet(
    texturesheets: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<
        (&ParticleIDs, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleColorAndUV), Changed<ParticleModifyState>
    >,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    particle_sys.iter_mut().for_each(
        |(
            ids, ages, baserandoms,
            mut uvs
        )| {
            if let Ok(calculator) = texturesheets.get(ids.calculator.0) {
                if let Some(calculator) = &calculator.texturesheet { fn_texturesheet(calculator, ids, ages, baserandoms, &mut uvs.uv); }
                // let activeids = &ids.actives;
                // uvs.run(activeids, ages, baserandoms, &texturesheet.0);
            }
        }
    );
    performance.sys_texturesheet = (pi_time::Instant::now() - time0).as_micros() as u32;
}
pub fn fn_texturesheet(
    texturesheet: &ParticleCalculatorTextureSheet,
    ids: &ParticleIDs, ages: &ParticleAgeLifetime, baserandoms: &ParticleBaseRandom, uvs: &mut ParticleUV
) {
    let activeids = &ids.actives;
    uvs.run(activeids, ages, baserandoms, &texturesheet.0);
}

pub fn sys_update_buffer(
    calculators: Query<&ParticleCalculatorBase>,
    particle_sys: Query<
        (Entity, &ParticleAttributes, &ParticleRunningState, &ParticleSystemTime, &ParticleIDs, &ParticleLocalScaling, &ParticleLocalRotation, &ParticleLocalPosition, &ParticleDirection, &ParticleEmitMatrix, &ParticleColorAndUV),
    >,
    meshes: Query<(&GlobalEnable, &GeometryID, &ModelInstanceAttributes)>,
    mut meshrenderenables: Query<&mut RenderGeometryEable>,
    instanceinfos: Query<&InstancedInfo>,
    mut slots: (
        Query<&mut AssetResVBSlot01>,
        Query<&mut AssetResVBSlot02>,
        Query<&mut AssetResVBSlot03>,
        Query<&mut AssetResVBSlot04>,
        Query<&mut AssetResVBSlot05>,
        Query<&mut AssetResVBSlot06>,
        Query<&mut AssetResVBSlot07>,
        Query<&mut AssetResVBSlot08>,
        // Query<&mut AssetResVBSlot09>,
        // Query<&mut AssetResVBSlot10>,
        // Query<&mut AssetResVBSlot11>,
        // Query<&mut AssetResVBSlot12>,
        // Query<&mut AssetResVBSlot13>,
        // Query<&mut AssetResVBSlot14>,
        // Query<&mut AssetResVBSlot15>,
        // Query<&mut AssetResVBSlot16>,
    ),
    mut performance: ResMut<ParticleSystemPerformance>,
    instant: Res<EngineInstant>,
) {
    let time0 = pi_time::Instant::now();
    // let mut ptime = pi_time::Instant::now();
    // let mut ptime1 = pi_time::Instant::now();
    let currms = (time0 - instant.0).as_millis() as u64;
    performance.update_buffer = (performance.last_running_time + performance.update_frame_time_ms as u64) < currms;
    if performance.update_buffer {
        performance.last_running_time = currms;
        // log::warn!("ParticleBuffer: ");
        let mut count_particles = 0;
        let mut collectdata: Vec<u8> = Vec::with_capacity(performance.maxparticles as usize * (4 + 4 + 16) * 4);

        particle_sys.iter().for_each(
            |(
                entity, _attributes, state, _time, ids, scalings, rotations, positions, directions, emitmatrixs,
                colorsanduvs
            )| {
                let particle_count = ids.actives.len();

                // log::warn!("sys_update_buffer A");
                if state.0 == false || particle_count == 0 {
                    if let Ok(mut rendergeometry) = meshrenderenables.get_mut(entity) {
                        *rendergeometry = RenderGeometryEable(false);
                        return;
                    }
                }

                // if time.running_delta_ms <= 0 { return; }
                if let Ok((enable, idgeo, _instanceattributes)) = meshes.get(entity) {

                    if enable.0 == false { return; }
                    
                    let id_geo = idgeo.0;
                    if let Ok(instanceinfo) = instanceinfos.get(id_geo) {

                        count_particles += particle_count;

                        // log::warn!("sys_update_buffer B");
                        if let Ok(calculator) = calculators.get(ids.calculator.0) {
                            collectdata.clear();
                            // let mut collectdata: Vec<f32> = Vec::with_capacity(length * (4 + 4 + 16));
                            
                            let renderalign = calculator.render_align();
                            let updatebuffer = renderalign.is_some();
                            // log::warn!("ActiveCount: {:?}", length);

                            if let Some(renderalign) = renderalign {
                                let calc_local = match renderalign {
                                    ERenderAlignment::View                  => calc_local_other,
                                    ERenderAlignment::World                 => calc_local_other,
                                    ERenderAlignment::Local                 => calc_local_other,
                                    ERenderAlignment::Facing                => calc_local_other,
                                    ERenderAlignment::Velocity              => calc_local_other,
                                    ERenderAlignment::StretchedBillboard    => calc_local_strentched,
                                    ERenderAlignment::HorizontalBillboard   => calc_local_other,
                                    ERenderAlignment::VerticalBillboard     => calc_local_other,
                                };
                                let calc_matrix = match renderalign {
                                    ERenderAlignment::View                  => calc_matrix_view  ,
                                    ERenderAlignment::World                 => calc_matrix_world ,
                                    ERenderAlignment::Local                 => calc_matrix_local ,
                                    ERenderAlignment::Facing                => calc_matrix_facing    ,
                                    ERenderAlignment::Velocity              => calc_matrix_velocity  ,
                                    ERenderAlignment::StretchedBillboard    => calc_matrix_strentched    ,
                                    ERenderAlignment::HorizontalBillboard   => calc_matrix_horizontal    ,
                                    ERenderAlignment::VerticalBillboard     => calc_matrix_vertical  ,
                                };
                                let mut emitposition = Vector3::zeros();
                                // let zero = Vector3::zeros();
                                let mut g_velocity = Vector3::zeros();

                                let mut stripe = (16 + 4 + 4);
                                // let mut collect_common: Vec<u8> = Vec::with_capacity(ids.actives.len() * (16 + 4 + 4) * 4);
                                let mut collect_float: Vec<f32> = Vec::with_capacity(ids.actives.len() * stripe);
                                unsafe { collect_float.set_len(ids.actives.len() * stripe); }

                                // let collect = &mut collect_float;
                                // let mut tempbytes = instanceattributes.bytes().clone();

                                // let mut matrixoffset = None;
                                // let mut coloroffset = None;
                                // let mut tilloffset = None;
                                // let mut extaoffset: Option<usize> = None;
                                // let mut extboffset: Option<usize> = None;
                                
                                // // 粒子系统内部各粒子的网格属性是相同的
                                // attributes.0.iter().for_each(|v| {
                                //     match v.vtype {
                                //         EParticleAttributeType::Matrix => if instanceattributes.worldmatrix() {
                                //             matrixoffset = Some(0 as usize);
                                //             stripe += 16 * 4;
                                //         },
                                //         EParticleAttributeType::Color => if let Some(offset) = instanceattributes.offset(&v.attr) {
                                //             coloroffset = Some(offset.offset() as usize);
                                //             stripe += 4 * 4;
                                //         },
                                //         EParticleAttributeType::Tilloff => if let Some(offset) = instanceattributes.offset(&v.attr) {
                                //             tilloffset = Some(offset.offset() as usize);
                                //             stripe += 4 * 4;
                                //         },
                                //         EParticleAttributeType::Extend4A => if let Some(_offset) = instanceattributes.offset(&v.attr) {
                                //             // let mut idx = offset.offset() as usize;
                                //             // bytemuck::cast_slice(matrix.as_slice()).iter().for_each(|v| { tempbytes[idx] = *v; idx += 1; });
                                //         },
                                //         EParticleAttributeType::Extend4B => if let Some(_offset) = instanceattributes.offset(&v.attr) {
                                //             // let mut idx = offset.offset() as usize;
                                //             // bytemuck::cast_slice(matrix.as_slice()).iter().for_each(|v| { tempbytes[idx] = *v; idx += 1; });
                                //         },
                                //     }
                                // });
        
                                let mut index = 0;
                                ids.actives.iter().for_each(|idx| {
                                    let scaling = scalings.get(*idx).unwrap();
                                    let eulers = rotations.get(*idx).unwrap();
        
                                    let mut translation = positions.get(*idx).unwrap().clone();
                                    // log::warn!("LOCAL: {:?}", translation);
        
                                    translation = translation + calculator.pivot.clone();
        
                                    let direction = directions.get(*idx).unwrap();
                                    let emitmatrix = emitmatrixs.get(*idx).unwrap();
        
                                    CoordinateSytem3::transform_normal(&direction.value, &emitmatrix.matrix, &mut g_velocity);
                                    CoordinateSytem3::transform_coordinates(&translation, &emitmatrix.matrix, &mut emitposition);
    
                                    // emitposition.copy_from_slice(emitmatrix.matrix.fixed_view::<3, 1>(0, 3).as_slice());
                                    let vlen = direction.length; // CoordinateSytem3::length(&direction.value);
                                    // log::warn!("Velocity: {:?}", (g_velocity, direction.value));
                                    // log::warn!("Translation: {:?}", emitposition);
                                    // let matrix = emitmatrix.matrix.clone();

                                    let matrix = if updatebuffer {
                                        let l_rotation = CoordinateSytem3::rotation_matrix_from_euler_angles(eulers.x, eulers.y, eulers.z);
                                        let mut matrix = calc_matrix(
                                            &emitposition, &emitmatrix.scaling, &emitmatrix.rotation, &g_velocity,
                                            &Vector3::zeros(), &scaling, &l_rotation, &eulers
                                        );
                            
                                        if let Some(local) = calc_local(&g_velocity, calculator.stretched_length_scale, calculator.stretched_velocity_scale * vlen) {
                                            matrix = matrix * local;
                                        }
                                        matrix
                                    } else {
                                        // let mut matrix = Matrix::identity();
                                        // CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &emitmatrix.rotation, &emitposition, &mut matrix);
                                        let matrix = &emitmatrix.matrix;
                                        let mut local = Matrix::identity();
                                        CoordinateSytem3::matrix4_compose_euler_angle(scaling, eulers, &translation, &mut local);
                                        // log::warn!("MAREIX: {:?}", matrix);
                                        // log::warn!("LOCAL: {:?}", local);
                                        matrix * local
                                    };
        
                                    let color = colorsanduvs.color.0.get(*idx).unwrap();
                                    let uv = colorsanduvs.uv.0.get(*idx).unwrap();

                                    // let offset = idx * stripe;
                                    // 获取粒子的网格实例化属性写入顶点Buffer
                                    {
                                        let mut ii = 0;
                                        // matrix.as_slice().iter().for_each(|v| { collect_float.push(*v); });
                                        // color.as_slice().iter().for_each(|v| { collect_float.push(*v); });
                                        // [uv.uscale, uv.vscale, uv.uoffset, uv.voffset].iter().for_each(|v| { collect_float.push(*v); });
                                        
                                        matrix.as_slice().iter().for_each(|v| { collect_float[ii + stripe * index] = *v; ii+=1; });
                                        color.as_slice().iter().for_each(|v| { collect_float[ii + stripe * index] = *v; ii+=1; });
                                        [uv.uscale, uv.vscale, uv.uoffset, uv.voffset].iter().for_each(|v| { collect_float[ii + stripe * index] = *v; ii+=1; });
                                    }

                                    index += 1;
                                });

                                let collect_common: Vec<u8> = bytemuck::cast_slice(collect_float.as_slice()).to_vec();
                                reset_instances_buffer_range(id_geo, &instanceinfo, &mut slots, collect_common, ids.actives.len() as u32);
                            }
                        }
                    }
                }
            }
        );

        performance.particles = count_particles as u32;
    }

    performance.sys_update_buffer = (pi_time::Instant::now() - time0).as_micros() as u32;
    // let ptime1 = pi_time::Instant::now();
    // log::warn!("ParticleBuffer update_buffer: {:?}", ptime1 - ptime);
    // log::warn!("ParticleBuffer: End");
}

pub fn sys_update_buffer_trail(
    trailmodifiers: Query<&ParticleCalculatorTrail>,
    mut particle_sys: Query<
        (&ParticleRunningState, &ParticleSystemTime, &ParticleIDs, &ParticleEmitMatrix, &ParticleBaseRandom, &ParticleColorAndUV, &ParticleLocalPosition, &ParticleLocalScaling, &ParticleLocalRotation, &ParticleDirection, &ParticleTrailMesh, &mut ParticleTrail),
    >,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut meshes: Query<&mut RenderGeometryEable>,
    mut trailbuffer: ResMut<ResParticleTrailBuffer>,
    queue: Res<PiRenderQueue>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    let time0 = pi_time::Instant::now();
    if performance.update_buffer {
        if let Some(trailbuffer) = &mut trailbuffer.0 {
            particle_sys.iter_mut().for_each(
                |(
                    state, time, ids, emitmatrixs, randoms, colors, positions, scalings, rotations, directions, trailmesh, mut trails
                )| {
                    // log::warn!("Trail Update: 00");
                    if let Ok(mut geometry) = geometries.get_mut(trailmesh.geo){
                        if state.0 == false {
                            if let Ok(mut rendergeometry) = meshes.get_mut(trailmesh.mesh) {
                                *rendergeometry = RenderGeometryEable(false);
                            }
                            return;
                        }
                        if let Ok(trailmodifier) = trailmodifiers.get(ids.calculator.0) {
                            let newids = &ids.newids;
                            trails.run_new(newids, randoms, &colors.color.0, positions, scalings, rotations, emitmatrixs, directions, &trailmodifier.0);

                            let activeids = [ids.actives.clone(), ids.dies.clone()].concat();
                            // log::warn!("Trail Update: {:?}", activeids.len());
                            trails.run(&activeids, randoms, &colors.color.0, positions, scalings, rotations, emitmatrixs, time, &trailmodifier.0);

                            let mut start = u32::MAX;
                            let mut end = 0;
                            let trailworldspace = trailmodifier.0.use_world_space;
                            activeids.iter().for_each(|idx| {
                                let item = trails.pathlist.get_mut(*idx).unwrap();
                                let parentmatrix = &emitmatrixs.get(*idx).unwrap().matrix;

                                // log::warn!("Trail: {:?}, {:?}", age, flag);
                                if item.3 {
                                    let (istart, iend) = trailbuffer.collect(&item, trailworldspace, parentmatrix);
                                    start = istart.min(start);
                                    end = iend.max(end);
                                // } else {
                                //     log::error!("Trail: OK");
                                }
                            });

                            if let Some(geometry) = &mut geometry.0 {
                                if let Some(vertices) = geometry.vertices.get_mut(0) {
                                    if start < end {
                                        vertices.buffer = EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), start, end)));
                                    } else {
                                        vertices.buffer = EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0)));
                                    }
                                }
                            }
                        } else {
                            if let Some(geometry) = &mut geometry.0 {
                                if let Some(vertices) = geometry.vertices.get_mut(0) {
                                    vertices.buffer = EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0)));
                                }
                            }
                        }
                    }
                }
            );
            trailbuffer.after_collect(&queue);
            performance.sys_update_buffer_trail = (pi_time::Instant::now() - time0).as_micros() as u32;
        }   
    }
}

pub fn sys_dispose_about_particle_system(
    particles: Query<(Entity, &DisposeReady, &ParticleTrailMesh), Changed<DisposeReady>>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    particles.iter().for_each(|(entity, state, trailmesh)| {
        if state.0 == false { return; }

        disposereadylist.push(OpsDisposeReadyForRef::ops(trailmesh.mesh));
        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
