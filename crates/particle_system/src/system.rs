
use std::ops::Range;

use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};
use pi_scene_context::{geometry::instance::{instanced_buffer::*, types::ModelInstanceAttributes}, prelude::*};
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::TToolMatrix, SQuaternion, Vector4};

use crate::{base::*, ActionListCPUParticleSystemState, OpsCPUParticleSystemState};

pub fn runif_particlesystem(
    items: Query<&ParticleSystemRunningState>,
    state: Res<EngineCustomPlugins>,
) -> bool {
    let mut activenum = 0;
    items.iter().for_each(|state| {
        if state.isrunning {
            activenum += 1;
        }
    });
    0 < activenum && state.active
}

pub fn sys_particle_active(
    mut items: Query<(Entity, &GlobalEnable, &SceneID, &ParticleSystemActive, &mut ParticleSystemRunningState, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleSystemEmission), Or<(Changed<GlobalEnable>, Changed<ParticleSystemActive>)>>,
    psperformance: Res<ParticleSystemPerformance>,
    calculators: Query<&ParticleCalculatorBase>,
    scenes: Query<&SceneTime>,
    mut cmds: ResMut<ActionListCPUParticleSystemState>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_particle_active"));
    // let time0 = pi_time::Instant::now();
    items.iter_mut().for_each(|(entity, enable, idscene, active, mut state, mut ids, mut time, mut emission)| {
        if enable.0 == true && active.0 == true {
            if state.isrunning == false {
                if let (Ok(calculator), Ok(scenetime)) = (calculators.get(ids.calculator.as_ref().unwrap().0), scenes.get(idscene.0)) {

                    if 0 < calculator.delay && state.deltatime < calculator.delay as u64 {
                        cmds.push(OpsCPUParticleSystemState(entity, crate::command::ECPUParticleSystemState::Start()));
                        state.deltatime += scenetime.delta_ms();
                        return;
                    }

                    let timescale = time.time_scale;
                    *time = ParticleSystemTime::new(psperformance.frame_time_ms); time.time_scale = timescale;
                    *emission = ParticleSystemEmission::new();
                    ids.reset();

                    state.isrunning = true;
                    state.deltatime = 0;
                }

            }
        } else {
            state.isrunning = false;
            state.deltatime = 0;
        }
    });

    performance.particlesystem = psperformance.total();
}


pub fn sys_prewarm(
    mut items: Query<
        (
            (&DisposeReady, &ParticleSystemRunningState, &LocalScaling, &GlobalMatrix, &mut ParticleGravityFactor, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleSystemModifyState),
            (&mut ParticleSystemEmission, &mut ParticleRandom, &mut ParticleBaseRandom, &mut ParticleEmitMatrix, &mut AbsoluteTransform, &mut ParticleDirection),
            (&mut ParticleOrbitOffset, &mut ParticleOrbitVelocity, &mut ParticleOrbitRadial, Option<&mut ParticleTrail>),
            (&mut ParticleDieWaitTime, &mut ParticleStart, &mut ParticleLocal, &mut ParticleVelocityAndForce)
        ),
        Changed<ParticleSystemRunningState>
    >,
    calculators: Query<(
        &ParticleCalculatorBase, &ParticleCalculatorStartModifiers, &ParticleCalculatorOverLifetime
    )>,
    calculators_trail:  Query<&ParticleCalculatorTrail>,
    mut psperformance: ResMut<ParticleSystemPerformance>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_prewarm"));
    if psperformance.debug { psperformance.time = pi_time::Instant::now(); }

    let mut tempvec3 = Vector3::zeros();
    let mut orbit_center: Vector3 = Vector3::zeros();
    let mut orbit_direction: Vector3 = Vector3::zeros();
    let mut tmprotation = Rotation3::identity();
    let mut tmpscl = Vector3::zeros();
    let mut tmpvec = Vector3::zeros();

    items.iter_mut().for_each(|(
        (disposestate, state, localscl, gmatrix, mut gravities, mut ids, mut time, mut modifystate),
        (mut emission, mut random, mut randoms, mut emitmatrix, mut abstransform, mut directions),
        (mut orbitoffset, mut orbitvelocity, mut orbitradial, mut trails),
        (mut diewaittimes, mut particlestart, mut particlelocal, mut particlevelocityforce)
    )| {
        if let Ok((
            base, modifiers, overlifetime
        )) = calculators.get(ids.calculator.as_ref().unwrap().0) {
            let delta_ms = 66 as u32;

            if state.isrunning && disposestate.0 == false && base.prewarm {
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
                    fn_emitmatrix(localscl, gmatrix, &ids, &mut emitmatrix, &mut abstransform, &global_position, &mut tmpscl, &mut tmprotation, &mut tmpvec);
                    fn_emitter(&modifiers.shapeemitter, &modifiers.startspeed, &mut particlelocal.position, &mut directions, &ids, &time, &randoms, &mut tempvec3);
                    if let (Ok(trailmodifier), Some(trails)) = (calculators_trail.get(ids.calculator.as_ref().unwrap().0), trails.as_deref_mut()) {
                        fn_start_lifetime(&modifiers.startlifetime, &ids, &time, &randoms, &mut particlestart.ages, &mut diewaittimes, Some(trailmodifier), Some(trails));
                    } else {
                        fn_start_lifetime(&modifiers.startlifetime, &ids, &time, &randoms, &mut particlestart.ages, &mut diewaittimes, None, None);
                    }
                    fn_gravity(&modifiers.gravity, &particlestart.ages, &ids, &time, &emitmatrix, &randoms, &mut particlevelocityforce.gravities);
                    fn_start_size(&modifiers.startsize, &ids, &time, &randoms, &mut particlestart.scale, &mut particlelocal.scalings);
                    fn_start_rotation(&modifiers.startrotation, &ids, &time, &randoms, &mut particlelocal.rotation);
                    fn_start_color(&modifiers.startcolor, &ids, &time, &randoms, &mut particlestart.color, &mut particlelocal.colorsanduvs.color);
                    if let Some(calculator) = &overlifetime.texturesheet {
                        fn_start_texture_sheet(calculator, &ids, &randoms, &mut particlelocal.colorsanduvs.uv);
                        fn_texturesheet(calculator, &ids, &particlestart.ages, &randoms, &mut particlelocal.colorsanduvs.uv);
                    }

                    fn_gravity(&modifiers.gravity, &particlestart.ages, &ids, &time, &emitmatrix, &randoms, &mut gravities);
                    if let Some(calculator) = &overlifetime.color {
                        fn_color_over_life_time(calculator, &ids, &particlestart.ages, &randoms, &particlestart.color, &mut particlelocal.colorsanduvs.color);
                    }
                    if let Some(calculator) = &overlifetime.rotation {
                        fn_rotation_over_life_time(calculator, &ids, &time, &particlestart.ages, &randoms, &mut particlelocal.rotation);
                    }
                    if let Some(calculator) = &overlifetime.size {
                        fn_size_over_life_time(calculator, &ids, &time, &particlestart.ages, &randoms, &particlestart.scale, &mut particlelocal.scalings);
                    }
                    if let Some(calculator) = &overlifetime.velocity {
                        fn_velocity_over_life_time(calculator, &ids, &time, &particlestart.ages, &randoms, &mut particlevelocityforce.velocity);
                    }
                    if let Some(calculator) = &overlifetime.limitvelocity {
                        fn_limit_velocity_over_life_time(calculator, &ids, &time, &particlestart.ages, &randoms, &mut particlevelocityforce.limitvelocityscalar);
                    }
                    fn_force_over_life_time(&overlifetime.force, &ids, &time, &particlestart.ages, &emitmatrix, &randoms, &mut particlevelocityforce.forces);
                    fn_orbit_over_life_time(&overlifetime.orbitoffset, &overlifetime.orbitvelocity, &overlifetime.orbitradial, &ids, &particlestart.ages, &randoms, &mut orbitoffset, &mut orbitvelocity, &mut orbitradial);
                    if let Some(calculator) = &overlifetime.speed {
                        fn_speed_modifier_over_life_time(calculator, &ids, &time, &particlestart.ages, &randoms, &mut particlevelocityforce.speedfector);
                    }
                    fn_direction(&modifiers.shapeemitter, &ids, &time, &particlevelocityforce.velocity, &particlevelocityforce.gravities, &particlevelocityforce.forces, &particlevelocityforce.speedfector, &particlevelocityforce.limitvelocityscalar, &orbitoffset, &orbitvelocity, &orbitradial, &mut directions, &mut particlelocal.position, &mut tempvec3, &mut orbit_center, &mut orbit_direction);

                    if let Some(calculator) = &overlifetime.sizebyspeed {
                        fn_size_by_speed(calculator, &ids, &time, &directions, &randoms, &mut particlelocal.scalings);
                    }
                    if let Some(calculator) = &overlifetime.colorbyspeed {
                        fn_color_by_speed(calculator, &ids, &time, &directions, &randoms, &mut particlelocal.colorsanduvs.color);
                    }
                    if let Some(calculator) = &overlifetime.rotationbyspeed {
                        fn_rotation_by_speed(calculator, &ids, &time, &directions, &randoms, &mut particlelocal.rotation);
                    }

                    fn_ids(&mut ids, &particlestart.ages, &time, &diewaittimes);
                }
                time.time_scale = timescale;
            }
        }
    });
    
    if psperformance.debug { psperformance.sys_prewarm = (pi_time::Instant::now() - psperformance.time).as_micros() as u32; }
}

/// 系统的启动
pub fn sys_ids(
    mut particle_sys: Query<(&mut ParticleIDs, &ParticleStart, &ParticleSystemTime, &ParticleDieWaitTime), Changed<ParticleSystemModifyState>>,
    mut psperformance: ResMut<ParticleSystemPerformance>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_ids"));
    if psperformance.debug { psperformance.time = pi_time::Instant::now(); }

    particle_sys.iter_mut().for_each(|(mut ids, ages, time, diewaittimes)| {
        if time.running_delta_ms <= 0 { return; }
        fn_ids(&mut ids, &ages.ages, time, diewaittimes);
    });

    if psperformance.debug { psperformance.sys_ids = (pi_time::Instant::now() - psperformance.time).as_micros() as u32; }
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
    ids.actives.sort();
    // log::warn!("actives: {:?}", ids.actives);
}
pub fn sys_emission(
    scenes: Query<&SceneTime>,
    calculators: Query<(&ParticleCalculatorBase, &ParticleCalculatorStartModifiers)>,
    mut particle_sys: Query<(&SceneID, &DisposeReady, &ParticleSystemRunningState, &mut ParticleRandom, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleSystemEmission, &mut ParticleBaseRandom, &mut ParticleSystemModifyState)>,
    mut psperformance: ResMut<ParticleSystemPerformance>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_emission"));
    if psperformance.debug { psperformance.time = pi_time::Instant::now(); }

    particle_sys.iter_mut().for_each(|(idscene, disposestate, state, mut random, mut ids, mut particlesystime, mut emissiondata, mut randoms, mut modifystate)| {
        if let (Ok(scenetime), Ok((base, calcemission))) = (scenes.get(idscene.0), calculators.get(ids.calculator.as_ref().unwrap().0)) {

            let delta_ms = scenetime.delta_ms() as u32;

            // log::warn!("{:?}, {:?}, {:?}, ", delta_ms, state.playing, disposestate.0);

            if state.isrunning && disposestate.0 == false {
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

    if psperformance.debug { psperformance.sys_emission = (pi_time::Instant::now() - psperformance.time).as_micros() as u32; }
}
fn fn_emission(
    base: &ParticleCalculatorBase, calcemission: &ParticleCalculatorEmission,
    random: &mut ParticleRandom, ids: &mut ParticleIDs, particlesystime: &mut ParticleSystemTime,
    emission: &mut ParticleSystemEmission, randoms: &mut ParticleBaseRandom, modifystate: &mut ParticleSystemModifyState,
) {
    *modifystate = ParticleSystemModifyState;

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
    mut particle_sys: Query<(&LocalScaling, &GlobalMatrix, &ParticleIDs, &ParticleSystemTime, &mut ParticleEmitMatrix, &mut AbsoluteTransform), Changed<ParticleSystemModifyState>>,
    mut psperformance: ResMut<ParticleSystemPerformance>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_emitmatrix"));
    if psperformance.debug { psperformance.time = pi_time::Instant::now(); }

    let global_position = Vector3::zeros();
    let mut tmprotation = Rotation3::identity();
    let mut tmpscl = Vector3::zeros();
    let mut tmpvec = Vector3::zeros();
    particle_sys.iter_mut().for_each(|(local_scaling, transform, ids, time, mut emitmatrixdata, mut absolute)| {
        if time.running_delta_ms <= 0 { return; }
        
        fn_emitmatrix(
            local_scaling, transform, ids,
            &mut emitmatrixdata, &mut absolute,
            &global_position, &mut tmpscl, &mut tmprotation, &mut tmpvec
        );
    });
    if psperformance.debug { psperformance.sys_emitmatrix = (pi_time::Instant::now() - psperformance.time).as_micros() as u32; }
}
fn fn_emitmatrix(
    local_scaling: &LocalScaling, transform: &GlobalMatrix, ids: &ParticleIDs,
    emitmatrixdata: &mut ParticleEmitMatrix, absolute: &mut AbsoluteTransform,
    global_position: &Vector3, tmpscl: &mut Vector3, tmprotation: &mut Rotation3, tmpvec: &mut Vector3
) {
    let newids = &ids.newids;
    let activeids = &ids.actives;

    // let global_position = transform.position().clone();
    // log::warn!("Position: {:?} {:?}", &localpos.0, global_position);

    let iso = absolute.iso(transform.matrix(), tmpvec, tmprotation);
    let global_scaling = tmpscl;
    global_scaling.clone_from(absolute.scaling(transform.matrix(), tmpvec, tmprotation));
    let global_rotation = absolute.rotation_quaternion(transform.matrix(), tmpvec, tmprotation);

    emitmatrixdata.emit(
        newids, activeids, &transform.matrix, &transform.matrix_inv, &iso, global_position, global_rotation, &global_scaling,
        &local_scaling.0
    );
}

pub fn sys_start(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    calculators_overlifetime: Query<&ParticleCalculatorOverLifetime>,
    mut performance: ResMut<ParticleSystemPerformance>,
    mut particle_sys: Query<(
        Entity, &ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom
        , &mut ParticleLocal, &mut ParticleDirection
        , &mut ParticleStart, &mut ParticleDieWaitTime
    ), Changed<ParticleSystemModifyState>>,
    calculators_trail: Query<&ParticleCalculatorTrail>,
    mut particle_sys_trail: Query<&mut ParticleTrail>,
) {
    if performance.debug { performance.time = pi_time::Instant::now(); }

    let mut tempvec3 = Vector3::zeros();
    particle_sys.iter_mut().for_each(|(
        entity, ids, time, randoms
        , mut particlelocal, mut directions
        , mut particlestart, mut diewaittimes
    )| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.as_ref().unwrap().0) {
            fn_emitter(&calculator.shapeemitter, &calculator.startspeed, &mut particlelocal.position, &mut directions, &ids, &time, &randoms, &mut tempvec3);

            if let (Ok(trailmodifier), Ok(mut trails)) = (calculators_trail.get(ids.calculator.as_ref().unwrap().0), particle_sys_trail.get_mut(entity)) {
                fn_start_lifetime(&calculator.startlifetime, &ids, &time, &randoms, &mut particlestart.ages, &mut diewaittimes, Some(trailmodifier), Some(&mut trails));
            } else {
                fn_start_lifetime(&calculator.startlifetime, &ids, &time, &randoms, &mut particlestart.ages, &mut diewaittimes, None, None);
            }

            fn_start_size(&calculator.startsize, ids, time, randoms, &mut particlestart.scale, &mut particlelocal.scalings);
            
            fn_start_rotation(&calculator.startrotation, ids, time, randoms, &mut particlelocal.rotation);
            
            fn_start_color(&calculator.startcolor, ids, time, randoms, &mut particlestart.color, &mut particlelocal.colorsanduvs.color);
            
        }
        if let Ok(calculator) = calculators_overlifetime.get(ids.calculator.as_ref().unwrap().0) {
            if let Some(calculator) = &calculator.texturesheet { fn_start_texture_sheet(calculator, ids, randoms, &mut particlelocal.colorsanduvs.uv); }
        }
    });

    if performance.debug { performance.sys_start = (pi_time::Instant::now() - performance.time).as_micros() as u32; }
}

fn fn_emitter(
    emitter: &ParticleCalculatorShapeEmitter, startspeed: &ParticleCalculatorStartSpeed,
    locpos: &mut ParticleLocalPosition, directions: &mut ParticleDirection, ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom,
    tempvec3: &mut Vector3,
) {
    let emitter = &emitter.0;
    let newids = &ids.newids;
    // let activeids = &ids.actives;

    locpos.start(newids, directions, randoms, time, emitter, startspeed, tempvec3);
}

fn fn_start_lifetime(
    calculator: &ParticleCalculatorStartLifetime, 
    ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom, items: &mut ParticleAgeLifetime, diewaittimes: &mut ParticleDieWaitTime,
    trailmodifier: Option<&ParticleCalculatorTrail>, trails: Option<&mut ParticleTrail>
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    items.start(time, newids, calculator, randoms);

    if let (Some(ParticleCalculatorTrail(Some(trailmodifier))), Some(trails)) = (trailmodifier, trails) {
        trails.start(newids, &items, &mut diewaittimes.0, &randoms, time, trailmodifier);
    } else {
        diewaittimes.start(newids, &items, randoms, time, None);
    }
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

pub fn fn_start_rotation(
    calculator: &ParticleCalculatorStartRotation,
    ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom, items: &mut ParticleLocalRotation
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    items.start(newids, &randoms, time, calculator);
}

pub fn fn_start_color(
    calculator: &ParticleCalculatorStartColor,
    ids: &ParticleIDs, time: &ParticleSystemTime, randoms: &ParticleBaseRandom, items: &mut ParticleStartColor, colors: &mut ParticleColor
) {
    let calculator = &calculator.0;
    let newids = &ids.newids;
    items.start(newids, &mut colors.0, &randoms, time, calculator);
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
pub fn sys_over_lifetime(
    calculators: Query<&ParticleCalculatorStartModifiers>,
    calculators_overlifetime: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(
        (&ParticleIDs, &ParticleSystemTime, &ParticleStart, &ParticleEmitMatrix, &ParticleBaseRandom)
        , &mut ParticleVelocityAndForce
        , &mut ParticleLocal
        , (&mut ParticleOrbitOffset, &mut ParticleOrbitVelocity, &mut ParticleOrbitRadial)
    ), Changed<ParticleSystemModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    if performance.debug { performance.time = pi_time::Instant::now(); }

    particle_sys.iter_mut().for_each(|(
        (ids, time, particlestart, emitmatrixs, randoms)
        , mut particlevelocityforce
        , mut particlelocal
        , (mut items_orbitoff, mut items_orbit_velocity, mut items_orbitradial)
    )| {
        if time.running_delta_ms <= 0 { return; }

        let startcolors = &particlestart.color;
        let startsizes = &particlestart.scale;

        let ages = &particlestart.ages;
        if let Ok(calculator) = calculators.get(ids.calculator.as_ref().unwrap().0) {
            fn_gravity(&calculator.gravity, ages, ids, time, emitmatrixs, randoms, &mut particlevelocityforce.gravities);
        }

        if let Ok(calculator) = calculators_overlifetime.get(ids.calculator.as_ref().unwrap().0) {
            fn_force_over_life_time(&calculator.force, ids, time, ages, emitmatrixs, randoms, &mut particlevelocityforce.forces);
            fn_orbit_over_life_time(&calculator.orbitoffset, &calculator.orbitvelocity, &calculator.orbitradial, ids, ages, randoms, &mut items_orbitoff, &mut items_orbit_velocity, &mut items_orbitradial);
            if let Some(calculator) = &calculator.color { fn_color_over_life_time(calculator, ids, ages, randoms, startcolors, &mut particlelocal.colorsanduvs.color); }
            if let Some(calculator) = &calculator.rotation { fn_rotation_over_life_time(calculator, ids, time, ages, randoms, &mut particlelocal.rotation); }
            if let Some(calculator) = &calculator.size { fn_size_over_life_time(calculator, ids, time, ages, randoms, startsizes, &mut particlelocal.scalings); }
            if let Some(calculator) = &calculator.velocity { fn_velocity_over_life_time(calculator, ids, time, ages, randoms, &mut particlevelocityforce.velocity); }
            if let Some(calculator) = &calculator.speed { fn_speed_modifier_over_life_time(calculator, ids, time, ages, randoms, &mut particlevelocityforce.speedfector); }
            if let Some(calculator) = &calculator.limitvelocity { fn_limit_velocity_over_life_time(calculator, ids, time, ages, randoms, &mut particlevelocityforce.limitvelocityscalar); }
            if let Some(calculator) = &calculator.texturesheet { fn_texturesheet(calculator, ids, ages, randoms, &mut particlelocal.colorsanduvs.uv); }
        }
    });
    if performance.debug { performance.sys_over_life_time = (pi_time::Instant::now() - performance.time).as_micros() as u32; }
}

fn fn_gravity(calculator: &ParticleCalculatorGravity, ages: &ParticleAgeLifetime, ids: &ParticleIDs, time: &ParticleSystemTime, emitmatrixs: &ParticleEmitMatrix, randoms: &ParticleBaseRandom, items: &mut ParticleGravityFactor) {
    items.run(&ids.actives, &ages.0, emitmatrixs, &randoms.0, time, calculator);
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

pub fn fn_color_over_life_time(
    calculator: &ParticleCalculatorColorOverLifetime,
    ids: &ParticleIDs, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, startcolors: &ParticleStartColor, items: &mut ParticleColor,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, startcolors, randoms, calculator);
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

pub fn fn_size_over_life_time(
    calculator: &ParticleCalculatorSizeOverLifetime,
    ids: &ParticleIDs, _time: &ParticleSystemTime, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, startsizes: &ParticleStartScaling, items: &mut ParticleLocalScaling,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, &startsizes.0, randoms, calculator);
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

pub fn fn_speed_modifier_over_life_time(
    calculator: &ParticleCalculatorSpeedModifier,
    ids: &ParticleIDs, time: &ParticleSystemTime, ages: &ParticleAgeLifetime, randoms: &ParticleBaseRandom, items: &mut ParticleSpeedFactor,
) {
    let calculator = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    items.run(activeids, ages, randoms, time, calculator);
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
        &ParticleVelocityAndForce,
        &ParticleOrbitOffset, &ParticleOrbitVelocity, &ParticleOrbitRadial,
        &mut ParticleDirection, &mut ParticleLocal
    ), Changed<ParticleSystemModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    if performance.debug { performance.time = pi_time::Instant::now(); }
    let mut temp = Vector3::zeros();
    let mut orbit_center: Vector3 = Vector3::zeros();
    let mut orbit_direction: Vector3 = Vector3::zeros();

    particle_sys.iter_mut().for_each(
        |(
            ids, time,
            particlevelocityforce,
            orbitsoffset, orbitsvelocity, orbitsradial,
            mut direction, mut particlelocal
        )| {
            if time.running_delta_ms <= 0 { return; }
            let forces = &particlevelocityforce.forces;
            let limitscalars = &particlevelocityforce.limitvelocityscalar;
            let velocities = &particlevelocityforce.velocity;
            let gravities = &particlevelocityforce.gravities;
            let speedfactors = &particlevelocityforce.speedfector;

            if let Ok(calculator) = calculators.get(ids.calculator.as_ref().unwrap().0) {
                fn_direction(&calculator.shapeemitter, ids, time, velocities, gravities, forces, speedfactors, limitscalars, orbitsoffset, orbitsvelocity, orbitsradial, &mut direction, &mut particlelocal.position, &mut temp, &mut orbit_center, &mut orbit_direction);
                // let emitter = &calculator.0;
                // // let newids = &ids.newids;
                // let activeids = &ids.actives;
                // direction.run(activeids, forces, gravities, velocities, limitscalars, orbits, speedfactors, &mut positions, emitter, time);
            }
        }
    );
    if performance.debug { performance.sys_direction = (pi_time::Instant::now() - performance.time).as_micros() as u32; }
}
pub fn fn_direction(
    calculator: &ParticleCalculatorShapeEmitter,
    ids: &ParticleIDs, time: &ParticleSystemTime,
    velocities: &ParticleVelocity, gravities: &ParticleGravityFactor, forces: &ParticleForce, speedfactors: &ParticleSpeedFactor, limitscalars: &ParticleLimitVelocityScalar,
    orbitsoffset: &ParticleOrbitOffset, orbitsvelocity: &ParticleOrbitVelocity, orbitsradial: &ParticleOrbitRadial,
    direction: &mut ParticleDirection, positions: &mut ParticleLocalPosition,
    temp: &mut Vector3,
    orbit_center: &mut Vector3,
    orbit_direction: &mut Vector3,
) {
    let emitter = &calculator.0;
    // let newids = &ids.newids;
    let activeids = &ids.actives;
    direction.run(activeids, forces, gravities, velocities, limitscalars, orbitsvelocity, orbitsoffset, orbitsradial, speedfactors, positions, emitter, time, temp, orbit_center, orbit_direction);
}

// ========================== by speed
pub fn sys_by_speed(
    calculators: Query<&ParticleCalculatorOverLifetime>,
    mut particle_sys: Query<(
        &ParticleIDs, &ParticleSystemTime, &ParticleDirection, &ParticleBaseRandom
        , &mut ParticleLocal
    ), Changed<ParticleSystemModifyState>>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    if performance.debug { performance.time = pi_time::Instant::now(); }

    particle_sys.iter_mut().for_each(|(
        ids, time, directions, randoms
        , mut particlelocal
    )| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.as_ref().unwrap().0) {
            if let Some(calculator) = &calculator.colorbyspeed { fn_color_by_speed(calculator, ids, time, directions, randoms, &mut particlelocal.colorsanduvs.color); }
            
            if let Some(calculator) = &calculator.sizebyspeed { fn_size_by_speed(calculator, ids, time, directions, randoms, &mut particlelocal.scalings); }
            
            if let Some(calculator) = &calculator.rotationbyspeed { fn_rotation_by_speed(calculator, ids, time, directions, randoms, &mut particlelocal.rotation); }
        }
    });
    if performance.debug { performance.sys_by_speed = (pi_time::Instant::now() - performance.time).as_micros() as u32; }
}

pub fn fn_color_by_speed(
    calculator: &ParticleCalculatorColorBySpeed,
    ids: &ParticleIDs, _time: &ParticleSystemTime, directions: &ParticleDirection, randoms: &ParticleBaseRandom, items: &mut ParticleColor
) {
    let calculator = &calculator.0;
    let activeids = &ids.actives;
    items.speed(activeids, directions, randoms, calculator);
}

pub fn fn_size_by_speed(
    calculator: &ParticleCalculatorSizeBySpeed,
    ids: &ParticleIDs, _time: &ParticleSystemTime, directions: &ParticleDirection, randoms: &ParticleBaseRandom, items: &mut ParticleLocalScaling
) {
    let calculator = &calculator.0;
    let activeids = &ids.actives;
    items.speed(activeids, directions, randoms, calculator);
}

pub fn fn_rotation_by_speed(
    calculator: &ParticleCalculatorRotationBySpeed,
    ids: &ParticleIDs, time: &ParticleSystemTime, directions: &ParticleDirection, randoms: &ParticleBaseRandom, items: &mut ParticleLocalRotation
) {
    let calculator = &calculator.0;
    let activeids = &ids.actives;
    items.speed(activeids, directions, randoms, time, calculator);
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
    mut particle_sys: Query<
        (Entity, &ParticleAttributes, &mut ParticleSystemRunningState, &ParticleSystemTime, &ParticleIDs, &ParticleLocal, &ParticleDirection, &ParticleEmitMatrix),
    >,
    mut meshes: Query<(&GlobalEnable, &GeometryID, &ModelInstanceAttributes, &mut InstancedMeshTransparentSortCollection, &GlobalMatrix, &ModelMatIdxs)>,
    // mut meshrenderenables: Query<&mut RenderGeometryEable>,
    instanceinfos: Query<&InstancedInfoComp>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    if performance.debug { performance.time = pi_time::Instant::now(); }

    let mut count_particles = 0;

    let mut refwmatrix = Matrix::identity();
    let mut reflmatrix = Matrix::identity();
    let mut resultmatrix = Matrix::identity();
    let mut localmatrix = Matrix::identity();
    // let mut l_rotation = Rotation3::identity();
    let mut l_quaternion = SQuaternion::<Number>::identity();
    let v3zero = Vector3::zeros();
    // let v3one = Vector3::new(1., 1., 1.);
    let mut h = Vector4::zeros();
    let mut hh = Vector4::zeros();
    let mut emitposition = Vector3::zeros();
    let mut g_velocity = Vector3::zeros();
    let mut l_velocity = Vector3::zeros();
    let mut f_v = false;
    let mut f_lc = false;

    // let stripe = 16 + 4 + 4;
    let mut temp: [u8; 112] = [0; 112];
    particle_sys.iter_mut().for_each(
        |(
            entity, _attributes, mut state, _time, ids, particlelocal, directions, emitmatrixs
        )| {
            let particle_count = ids.actives.len();
            // log::warn!("sys_update_buffer A {:?}", particle_count);

            // if time.running_delta_ms <= 0 { return; }
            if let Ok((enable, idgeo, _instanceattributes, mut instancesort, gmatrix, matidxs)) = meshes.get_mut(entity) {

                if state.isrunning == false || particle_count == 0 {
                    // if let Ok(mut rendergeometry) = meshrenderenables.get_mut(entity) {
                    //     *rendergeometry = RenderGeometryEable(false);
                    //     return;
                    // }
                    instancesort.reset();
                    return;
                }

                if enable.0 == false { 
                    instancesort.reset();
                    return;
                }

                state.updatebuffer = state.waitframe >= state.update_buffer_interval_frame;
                if state.updatebuffer == false {
                    state.waitframe += 1;
                    return;
                }

                instancesort.reset();
                state.waitframe = 0;

                let positions = &particlelocal.position;
                let rotations = &particlelocal.rotation;
                let scalings = &particlelocal.scalings;
                let colorsanduvs = &particlelocal.colorsanduvs;
                
                let id_geo = idgeo.0;
                if let Ok(InstancedInfoComp(Some(instanceinfo))) = instanceinfos.get(id_geo) {
                    instancesort.sizeperinstance = instanceinfo.bytes_per_instance as u16;

                    count_particles += particle_count;

                    // log::warn!("sys_update_buffer B");
                    if let Ok(calculator) = calculators.get(ids.calculator.as_ref().unwrap().0) {
                        let renderalign = calculator.render_align();
                        let updatebuffer = renderalign.is_some();
                        // log::warn!("ActiveCount: {:?}", ids.actives.len());
                        f_v = false;
                        f_lc = false;
                        if let Some(renderalign) = renderalign {
                            f_lc = renderalign == ERenderAlignment::StretchedBillboard;
                            let calc_matrix = match renderalign {
                                ERenderAlignment::View                  => { calc_matrix_view  },
                                ERenderAlignment::World                 => { calc_matrix_world },
                                ERenderAlignment::Local                 => { calc_matrix_local },
                                ERenderAlignment::Facing                => { calc_matrix_facing    },
                                ERenderAlignment::Velocity              => { f_v = true; calc_matrix_velocity  },
                                ERenderAlignment::StretchedBillboard    => { f_v = true; calc_matrix_strentched    },
                                ERenderAlignment::HorizontalBillboard   => { calc_matrix_horizontal    },
                                ERenderAlignment::VerticalBillboard     => { calc_matrix_vertical  },
                            };

                            // unsafe { collect_float.set_len(ids.actives.len() * stripe); }

                            let mut index = 0;
                            ids.actives.iter().for_each(|idx| {
                                let scaling = scalings.get(*idx).unwrap();
                                let eulers = rotations.get(*idx).unwrap();
    
                                let translation = positions.get(*idx).unwrap();
                                // log::warn!("LOCAL: {:?}", translation);
    
                                let tx = translation.x + calculator.pivot.x;
                                let ty = translation.y + calculator.pivot.y;
                                let tz = translation.z + calculator.pivot.z;

                                let direction = directions.get(*idx).unwrap();
                                let emitmatrix = emitmatrixs.get(*idx).unwrap();
    
                                l_velocity.copy_from(&direction.value);
                                if f_v {
                                    h.x = direction.value.x; h.y = direction.value.y; h.z = direction.value.z; h.w = 0.;
                                    CoordinateSytem3::matrix4_mul_vector4(&emitmatrix.matrix, &h, &mut hh);
                                    g_velocity.x = hh.x; g_velocity.y = hh.y; g_velocity.z = hh.z;
                                    // CoordinateSytem3::transform_normal(&direction.value, &emitmatrix.matrix, &mut g_velocity);
                                }

                                h.x = tx; h.y = ty; h.z = tz; h.w = 1.;
                                CoordinateSytem3::matrix4_mul_vector4(&emitmatrix.matrix, &h, &mut hh);
                                emitposition.x = hh.x; emitposition.y = hh.y; emitposition.z = hh.z;
                                // CoordinateSytem3::transform_coordinates_floats(tx, ty, tz, &emitmatrix.matrix, &mut emitposition);

                                // emitposition.copy_from_slice(emitmatrix.matrix.fixed_view::<3, 1>(0, 3).as_slice());
                                let vlen = direction.length; // CoordinateSytem3::length(&direction.value);
                                // log::warn!("Velocity: {:?}", (g_velocity, direction.value));
                                // log::warn!("Translation: {:?}", emitposition);
                                // let matrix = emitmatrix.matrix.clone();

                                let matrix = if updatebuffer {
                                    // CoordinateSytem3::rotation_matrix_from_euler_angles_toref(eulers.x, eulers.y, eulers.z, &mut l_rotation);
                                    pi_scene_shell::prelude::quaternion_from_euler_angles(eulers.x, eulers.y, eulers.z, &mut l_quaternion);
                                    l_quaternion.normalize_mut();
                                    refwmatrix.copy_from(&emitmatrix.matrix);
                                    calc_matrix(
                                        &emitposition, &emitmatrix.scaling, &emitmatrix.rotation, &g_velocity,
                                        &v3zero, &scaling, &l_quaternion, &eulers,
                                        &mut refwmatrix, &mut reflmatrix, &mut resultmatrix
                                    );

                                    if f_lc {
                                        calc_local_strentched_call(&scaling, &l_velocity, calculator.stretched_length_scale, calculator.stretched_velocity_scale * vlen, &mut refwmatrix, &mut reflmatrix, &mut localmatrix);
                                        CoordinateSytem3::mul_to(&resultmatrix, &localmatrix, &mut refwmatrix);
                                        // refwmatrix.copy_from(&resultmatrix);
                                        &refwmatrix
                                    } else {
                                        &resultmatrix
                                    }
                                } else {
                                    // let mut matrix = Matrix::identity();
                                    // CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &emitmatrix.rotation, &emitposition, &mut matrix);
                                    // let mut local = Matrix::identity();
                                    // CoordinateSytem3::rotation_matrix_from_euler_angles_toref(eulers.x, eulers.y, eulers.z, &mut l_rotation);
                                    pi_scene_shell::prelude::quaternion_from_euler_angles(eulers.x, eulers.y, eulers.z, &mut l_quaternion);
                                    l_quaternion.normalize_mut();
                                    pi_scene_shell::prelude::matrix4_compose_quaternion(scaling, &l_quaternion, translation, &mut reflmatrix);
                                    // log::warn!("MAREIX: {:?}", matrix);
                                    // log::warn!("LOCAL: {:?}", local);
                                    CoordinateSytem3::mul_to(&emitmatrix.matrix, &reflmatrix, &mut resultmatrix);
                                    // emitmatrix.matrix.mul_to(&reflmatrix, &mut resultmatrix);
                                    &resultmatrix
                                };
    
                                let color = colorsanduvs.color.0.get(*idx).unwrap();
                                let uv = colorsanduvs.uv.0.get(*idx).unwrap();

                                // 获取粒子的网格实例化属性写入顶点Buffer
                                {
                                    // log::warn!("LOCAL: {:?}", ([uv.uscale, uv.vscale, uv.uoffset, uv.voffset], color));
                                    let m = matrix.as_slice();
                                    temp.as_mut_slice()[0..64  ].copy_from_slice(bytemuck::cast_slice(m));
                                    temp.as_mut_slice()[64..80 ].copy_from_slice(bytemuck::cast_slice(&matidxs.0));
                                    temp.as_mut_slice()[80..96 ].copy_from_slice(bytemuck::cast_slice(color.as_slice()));
                                    temp.as_mut_slice()[96..112].copy_from_slice(bytemuck::cast_slice(&uv.data));
                                    unsafe_vec_append_slice(&mut instancesort.data, bytemuck::cast_slice(temp.as_slice()));
                                }

                                index += 1;
                            });

                            // log::error!("Particel: {:?}", (ids.actives.len(), index));
                            // bytemuck::cast_slice(&collect_float.as_slice()[0..(index * stripe)]).iter().for_each(|v| { instancesort.data.push(*v); });
                            instancesort.ranges.push((0, Range { start: 0, end: index as u32 }, gmatrix.xyz()));
                            instancesort.count = index;
                        }
                    }
                }
            }
        }
    );

    performance.particles = count_particles as u32;

    if performance.debug { performance.sys_update_buffer = (pi_time::Instant::now() - performance.time).as_micros() as u32; }
    // log::error!("Particle Sytem: UpdateBuffer {:?} ms, direction {:?}, byspeed {:?}, overlife {:?}", ((pi_time::Instant::now() - time0).as_millis() as u32, count_particles), performance.sys_direction, performance.sys_by_speed, (performance.sys_over_life_time, performance.sys_emission, performance.sys_emitmatrix, performance.sys_ids, performance.sys_prewarm, performance.sys_start));
}

pub fn sys_update_buffer_trail(
    trailmodifiers: Query<&ParticleCalculatorTrail>,
    mut particle_sys: Query<
        (&ParticleSystemRunningState, &ParticleSystemTime, &ParticleIDs, &ParticleEmitMatrix, &ParticleBaseRandom, &ParticleLocal, &ParticleDirection, &ParticleTrailMesh, &mut ParticleTrail),
    >,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut meshes: Query<&mut RenderGeometryEable>,
    mut trailbuffer: ResMut<ResParticleTrailBuffer>,
    queue: Res<PiRenderQueue>,
    mut performance: ResMut<ParticleSystemPerformance>,
) {
    if performance.debug { performance.time = pi_time::Instant::now(); }

    if performance.update_buffer {
        if let Some(trailbuffer) = &mut trailbuffer.0 {
            particle_sys.iter_mut().for_each(
                |(
                    state, time, ids, emitmatrixs, randoms, particlelocal, directions, trailmesh, mut trails
                )| {
                    let colors = &particlelocal.colorsanduvs;
                    let positions = &particlelocal.position;
                    let scalings = &particlelocal.scalings;
                    let rotations = &particlelocal.rotation;

                    // log::warn!("Trail Update: 00");
                    if let Ok(mut geometry) = geometries.get_mut(trailmesh.geo){
                        if state.isrunning == false {
                            if let Ok(mut rendergeometry) = meshes.get_mut(trailmesh.mesh) {
                                *rendergeometry = RenderGeometryEable(false);
                            }
                            return;
                        }
                        if state.updatebuffer == false { return; }
                        if let Ok(ParticleCalculatorTrail(Some(trailmodifier))) = trailmodifiers.get(ids.calculator.as_ref().unwrap().0) {
                            let newids = &ids.newids;
                            trails.run_new(newids, randoms, &colors.color.0, positions, scalings, rotations, emitmatrixs, directions, &trailmodifier);

                            let activeids = [ids.actives.clone(), ids.dies.clone()].concat();
                            // log::warn!("Trail Update: {:?}", activeids.len());
                            trails.run(&activeids, randoms, &colors.color.0, positions, scalings, rotations, emitmatrixs, time, &trailmodifier);

                            let mut start = u32::MAX;
                            let mut end = 0;
                            let trailworldspace = trailmodifier.use_world_space;
                            activeids.iter().for_each(|idx| {
                                let item = trails.pathlist.get_mut(*idx).unwrap();
                                let parentmatrix = &emitmatrixs.get(*idx).unwrap().matrix;

                                // log::warn!("Trail: {:?}, {:?}", age, flag);
                                if item.3 {
                                    let (istart, iend) = trailbuffer.collect(&item, trailworldspace, parentmatrix);
                                    start = istart.min(start);
                                    end = iend.max(end);
                                }
                            });

                            if let Some(geometry) = &mut geometry.0 {
                                if start < end {
                                    geometry.update_vertices(0, EVerticesBufferUsage::EVBRange(Share::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), start, end))));
                                } else {
                                    geometry.update_vertices(0, EVerticesBufferUsage::EVBRange(Share::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                                }
                            }
                        } else {
                            if let Some(geometry) = &mut geometry.0 {
                                geometry.update_vertices(0, EVerticesBufferUsage::EVBRange(Share::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                            }
                        }
                    }
                }
            );
            trailbuffer.after_collect(&queue);
        }   
    }
    if performance.debug { performance.sys_update_buffer_trail = (pi_time::Instant::now() - performance.time).as_micros() as u32; }
}

pub fn sys_dispose_about_particle_system(
    particles: Query<(Entity, &DisposeReady, &ParticleTrailMesh), Changed<DisposeReady>>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecan: Query<&mut DisposeCan>,
) {
    particles.iter().for_each(|(entity, state, trailmesh)| {
        if state.0 == false { return; }

        disposereadylist.push(OpsDisposeReadyForRef::ops(trailmesh.mesh));
        if let Ok(mut dispose) = disposecan.get_mut(entity) { dispose.0 = true; }
    });
}
