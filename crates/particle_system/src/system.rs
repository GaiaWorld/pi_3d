
use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::{*, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolVector3}};

use crate::{base::*, tools::{Random, Velocity}};

/// 系统的启动
pub fn sys_emission(
    scenes: Query<&SceneTime>,
    calculators: Query<(&ParticleCalculatorBase, &ParticleCalculatorEmission)>,
    mut particle_sys: Query<(&SceneID, &DisposeReady, &ParticleState, &mut ParticleRandom, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleSystemEmission, &mut ParticleBaseRandom)>,
) {
    particle_sys.iter_mut().for_each(|(idscene, disposestate, state, mut random, mut ids, mut particlesystime, mut emission, mut randoms)| {
        if let (Ok(scenetime), Ok((base, calcemission))) = (scenes.get(idscene.0), calculators.get(ids.calculator.0)) {
            let delta_ms = scenetime.delta_ms() as u32;

            if state.playing && disposestate.0 == false {
                particlesystime.run(delta_ms, 1000, base.duration);
            } else {
                particlesystime.run(0, 1000, base.duration);
            }

            // log::warn!("Emission: {:?}, {:?}, ", delta_ms, particlesystime.running_delta_ms);

            // 间隔时间到达帧运行间隔
            if particlesystime.running_delta_ms > 0 {
                let rate_over_time = calcemission.rateovertime.interpolate(particlesystime.emission_progress, random.random()) as usize;
                // log::warn!("Emission Rate: {:?}, ", rate_over_time);
                emission.start(
                    base.looping, base.duration,
                    &particlesystime, rate_over_time,
                    &calcemission.bursts,
                    &mut ids
                );

                let newids = &ids.newids;
                let activeids = &ids.actives;

                randoms.run(newids, activeids, &mut random);
            }
        }
    });
}

pub fn sys_emitmatrix(
    calculators: Query<&ParticleCalculatorBase>,
    mut particle_sys: Query<(&LocalScaling, &LocalPosition, &WorldMatrix, &mut GlobalTransform, &ParticleIDs, &ParticleSystemTime, &mut ParticleEmitMatrix)>,
) {
    particle_sys.iter_mut().for_each(|(local_scaling, localpos, world_matrix, mut transform, ids, time, mut emitmatrix)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(base) = calculators.get(ids.calculator.0) {
            let newids = &ids.newids;
            let activeids = &ids.actives;
            let global_rotation = transform.rotation().clone();
            let global_scaling = transform.scaling().clone();
            let global_position = transform.position();
            // log::warn!("Position: {:?} {:?}", &localpos.0, global_position);
            emitmatrix.emit(newids, activeids, &base.simulation_space, &base.scaling_space, &world_matrix.0, global_position, &global_rotation, &global_scaling, &local_scaling.0);
        }
    });
}

pub fn sys_emitter(
    calculators: Query<(&ParticleCalculatorShapeEmitter, &ParticleCalculatorStartSpeed)>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleLocalPosition, &mut ParticleDirection)>,
) {
    let time = pi_time::Instant::now();

    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut locpos, mut directions)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok((emitter, startspeed)) = calculators.get(ids.calculator.0) {
            let emitter = &emitter.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;

            locpos.start(newids, &mut directions, randoms, time, emitter, startspeed);
        }
    });
    
    let time1 = pi_time::Instant::now();
    // log::warn!("emitter: {:?}", time1 - time);
}

pub fn sys_start_lifetime(
    calculators: Query<&ParticleCalculatorStartLifetime>,
    mut particle_sys: Query<(Entity, &ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleAgeLifetime, &mut ParticleDieWaitTime)>,
    calculators_trail: Query<&ParticleCalculatorTrail>,
    mut particle_sys_trail: Query<&mut ParticleTrail>,
) {
    particle_sys.iter_mut().for_each(|(entity, ids, time, randoms, mut items, mut diewaittimes)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            items.start(time, newids, calculator, randoms);

            if let (Ok(trailmodifier), Ok(mut trails)) = (calculators_trail.get(ids.calculator.0), particle_sys_trail.get_mut(entity)) {
                trails.start(newids, &items, &mut diewaittimes.0, &randoms, time, &trailmodifier.0);
            } else {
                diewaittimes.start(newids, &items, randoms, time, None);
            }
        }
    });
}

pub fn sys_start_size(
    calculators: Query<&ParticleCalculatorStartSize>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleStartScaling, &mut ParticleLocalScaling)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items, mut localscalings)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.start(newids, &mut localscalings, &randoms, time, calculator, );
        }
    });
}

pub fn sys_start_rotation(
    calculators: Query<&ParticleCalculatorStartRotation>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleLocalRotation)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            items.start(newids, &randoms, time, calculator);
        }
    });
}

pub fn sys_start_color(
    calculators: Query<&ParticleCalculatorStartColor>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleStartColor, &mut ParticleColor)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items, mut colors)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            items.start(newids, &mut colors, &randoms, time, calculator);
        }
    });
}

pub fn sys_start_texture_sheet(
    calculators: Query<&ParticleCalculatorTextureSheet>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleUV)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            items.start(newids, &randoms, calculator);
        }
    });
}
/// =================================== over life time
pub fn sys_color_over_life_time(
    calculators: Query<&ParticleCalculatorColorOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &ParticleStartColor, &mut ParticleColor)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, startcolors, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.run(activeids, ages, startcolors, randoms, calculator);
        }
    });
}

pub fn sys_rotation_over_life_time(
    calculators: Query<&ParticleCalculatorRotationOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleLocalRotation)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.run(activeids, ages, randoms, time, calculator);
        }
    });
}


pub fn sys_size_over_life_time(
    calculators: Query<&ParticleCalculatorSizeOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleLocalScaling)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.run(activeids, ages, randoms, time, calculator);
        }
    });
}


pub fn sys_velocity_over_life_time(
    calculators: Query<&ParticleCalculatorVelocityOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleVelocity)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.run(activeids, ages, randoms, time, calculator);
        }
    });
}

pub fn sys_orbit_over_life_time(
    offsets: Query<&ParticleCalculatorOrbitOffset>,
    velocitys: Query<&ParticleCalculatorOrbitVelocity>,
    radials: Query<&ParticleCalculatorOrbitRadial>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleOrbitVelocity)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        let offset = offsets.get(ids.calculator.0);
        let velocity = velocitys.get(ids.calculator.0);
        let radial = radials.get(ids.calculator.0);

        let newids = &ids.newids;
        let activeids = &ids.actives;
        items.run(activeids, ages, randoms, time, offset, velocity, radial);
    });
}

pub fn sys_speed_modifier_over_life_time(
    calculators: Query<&ParticleCalculatorSpeedModifier>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleSpeedFactor)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.run(activeids, ages, randoms, time, calculator);
        }
    });
}

pub fn sys_limit_velocity_over_life_time(
    calculators: Query<&ParticleCalculatorLimitVelocityOverLifetime>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleLimitVelocityScalar)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, ages, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.run(activeids, ages, randoms, time, calculator);
        }
    });
}

pub fn sys_direction(
    calculators: Query<&ParticleCalculatorShapeEmitter>,
    mut particle_sys: Query<(
        &ParticleIDs, &ParticleSystemTime, &ParticleEmitMatrix,
        &ParticleVelocity, &ParticleGravityFactor, &ParticleForce, &ParticleOrbitVelocity, &ParticleSpeedFactor, &ParticleLimitVelocityScalar,
        &mut ParticleDirection, &mut ParticleLocalPosition
    )>,
) {
    particle_sys.iter_mut().for_each(
        |(
            ids, time, emitmatrixs,
            velocities, gravities, forces, orbits, speedfactors, limitscalars,
            mut direction, mut positions
        )| {
            if time.running_delta_ms <= 0 { return; }

            if let Ok(calculator) = calculators.get(ids.calculator.0) {
                let emitter = &calculator.0;
                let newids = &ids.newids;
                let activeids = &ids.actives;
                direction.run(activeids, forces, gravities, velocities, limitscalars, orbits, speedfactors, emitmatrixs, &mut positions, emitter, time);
            }
        }
    );
}

// 
pub fn sys_color_by_speed(
    calculators: Query<&ParticleCalculatorColorBySpeed>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleDirection, &ParticleBaseRandom, &mut ParticleColor)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, directions, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let activeids = &ids.actives;
            items.speed(activeids, directions, randoms, calculator);
        }
    });
}

// 
pub fn sys_size_by_speed(
    calculators: Query<&ParticleCalculatorSizeBySpeed>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleDirection, &ParticleBaseRandom, &mut ParticleLocalScaling)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, directions, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let activeids = &ids.actives;
            items.speed(activeids, directions, randoms, calculator);
        }
    });
}

// 
pub fn sys_rotation_by_speed(
    calculators: Query<&ParticleCalculatorRotationBySpeed>,
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleDirection, &ParticleBaseRandom, &mut ParticleLocalRotation)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, directions, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let activeids = &ids.actives;
            items.speed(activeids, directions, randoms, time, calculator);
        }
    });
}

pub fn sys_ids(
    mut particle_sys: Query<(&mut ParticleIDs, &ParticleAgeLifetime, &ParticleSystemTime, &ParticleDieWaitTime)>,
) {
    particle_sys.iter_mut().for_each(|(mut ids, ages, time, diewaittimes)| {
        if time.running_delta_ms <= 0 { return; }

        ids.newids.clear();

        let mut items = [ids.actives.clone(), ids.dies.clone()].concat();
        ids.actives.clear();
        ids.dies.clear();
        items.drain(..).for_each(|idx| {
            let age = ages.get(idx).unwrap();
            let diewait = diewaittimes.0.get(idx).unwrap();
            // log::warn!("Age: {:?}, Lifetime: {:?}", age.age, age.lifetime);
            if age.age <= age.lifetime {
                ids.actives.push(idx);
            } else if age.age < age.lifetime + diewait {
                ids.dies.push(idx);
            } else {
                ids.unactives.push(idx);
            }
        });

        // log::warn!("actives: {:?}", ids.actives);
    });
}

pub fn sys_texturesheet(
    texturesheets: Query<&ParticleCalculatorTextureSheet>,
    mut particle_sys: Query<
        (&ParticleIDs, &ParticleAgeLifetime, &ParticleBaseRandom, &mut ParticleUV),
    >,
) {
    particle_sys.iter_mut().for_each(
        |(
            ids, ages, baserandoms,
            mut uvs
        )| {
            if let Ok(texturesheet) = texturesheets.get(ids.calculator.0) {
                let activeids = &ids.actives;
                uvs.run(activeids, ages, baserandoms, &texturesheet.0);
            }
        }
    );
}

pub fn sys_update_buffer(
    calculators: Query<&ParticleCalculatorBase>,
    particle_sys: Query<
        (Entity, &ParticleState, &ParticleSystemTime, &ParticleIDs, &ParticleLocalScaling, &ParticleLocalRotation, &ParticleLocalPosition, &ParticleDirection, &ParticleEmitMatrix, &ParticleColor, &ParticleUV),
    >,
    meshes: Query<(&GlobalEnable, &GeometryID)>,
    mut meshrenderenables: Query<&mut RenderGeometryEable>,
    mut worldmatrixbuffers: Query<&mut InstanceBufferWorldMatrix>,
    mut colorbuffers: Query<&mut InstanceBufferColor>,
    mut uvbuffers: Query<&mut InstanceBufferTillOff>,
    mut slots: (
        Query<&mut AssetResVBSlot01>,
        Query<&mut AssetResVBSlot02>,
        Query<&mut AssetResVBSlot03>,
        Query<&mut AssetResVBSlot04>,
        Query<&mut AssetResVBSlot05>,
        Query<&mut AssetResVBSlot06>,
        Query<&mut AssetResVBSlot07>,
        Query<&mut AssetResVBSlot08>,
        Query<&mut AssetResVBSlot09>,
        Query<&mut AssetResVBSlot10>,
        Query<&mut AssetResVBSlot11>,
        Query<&mut AssetResVBSlot12>,
        Query<&mut AssetResVBSlot13>,
        Query<&mut AssetResVBSlot14>,
        Query<&mut AssetResVBSlot15>,
        Query<&mut AssetResVBSlot16>,
    ),
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    let mut ptime = pi_time::Instant::now();
    let mut ptime1 = pi_time::Instant::now();

    // log::warn!("ParticleBuffer: ");
    particle_sys.iter().for_each(
        |(
            entity, state, time, ids, scalings, rotations, positions, directions, emitmatrixs,
            colors, uvs
        )| {
            if state.playing == false {
                if let Ok(mut rendergeometry) = meshrenderenables.get_mut(entity) {
                    *rendergeometry = RenderGeometryEable(false);
                    return;
                }
            }

            if time.running_delta_ms <= 0 { return; }

            let length = ids.actives.len();

            // log::warn!("sys_update_buffer");
            if let Ok(calculator) = calculators.get(ids.calculator.0) {
                let mut datamatrix: Vec<f32> = Vec::with_capacity(length * 16);
                let mut datacolors: Vec<f32> = Vec::with_capacity(length * 4);
                let mut datauvs: Vec<f32> = Vec::with_capacity(length * 4);
                
                let renderalign = calculator.render_align();
                let updatebuffer = renderalign.is_some();
                // log::warn!("ActiveCount: {:?}", length);

                let mut emitposition = Vector3::zeros();
                let zero = Vector3::zeros();

                ids.actives.iter().for_each(|idx| {
                    let scaling = scalings.get(*idx).unwrap();
                    let eulers = rotations.get(*idx).unwrap();

                    let mut translation = positions.get(*idx).unwrap().clone();
                    // log::warn!("LOCAL: {:?}", translation);

                    translation = translation + calculator.pivot.clone();

                    let direction = directions.get(*idx).unwrap();
                    let emitmatrix = emitmatrixs.get(*idx).unwrap();
                    let color = colors.get(*idx).unwrap();
                    let uv = uvs.get(*idx).unwrap();
                    let mut g_velocity = Vector3::zeros();

                    CoordinateSytem3::transform_normal(&direction.value, &emitmatrix.matrix, &mut g_velocity);
                    CoordinateSytem3::transform_coordinates(&translation, &emitmatrix.matrix, &mut emitposition);
                    // emitposition.copy_from_slice(emitmatrix.matrix.fixed_view::<3, 1>(0, 3).as_slice());
                    let vlen = CoordinateSytem3::length(&g_velocity);
                    // log::warn!("Velocity: {:?}", g_velocity);

                    let matrix = if let Some(renderalign) = renderalign {
                        // let mut matrix = match renderalign {
                        //     ERenderAlignment::Velocity => {
                        //         let rotation = renderalign.calc_rotation(&emitmatrix.rotation, emitmatrix.eulers, &g_velocity);
                        //         let mut matrix = Matrix::identity();
                        //         CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &rotation, &emitposition, &mut matrix);
                        //         matrix
                        //     },
                        //     ERenderAlignment::StretchedBillboard => {
                        //         let rotation = renderalign.calc_rotation(&emitmatrix.rotation, emitmatrix.eulers, &g_velocity);
                        //         let mut matrix = Matrix::identity();
                        //         CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &rotation, &emitposition, &mut matrix);
                        //         matrix
                        //     }
                        //     _ => {
                        //         let mut matrix = Matrix::identity();
                        //         CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &emitmatrix.rotation, &emitposition, &mut matrix);
                        //         matrix
                        //     }
                        // };
                        let rotation = renderalign.calc_rotation(&emitmatrix.rotation, emitmatrix.eulers, &g_velocity);
                        // log::warn!("rotation : {:?}", rotation);
                        let mut matrix = Matrix::identity();
                        CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &rotation, &emitposition, &mut matrix);
                        
                        let mut local = Matrix::identity();
                        CoordinateSytem3::matrix4_compose_euler_angle(scaling, eulers, &zero, &mut local);
                        // log::warn!("a MAREIX: {:?}", matrix);
                        matrix = matrix * local;
            
                        if let Some(local) = renderalign.calc_local(&g_velocity, calculator.stretched_length_scale, calculator.stretched_velocity_scale * vlen) {
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

                    if updatebuffer {
                        // log::warn!("MAREIX: {:?}", matrix);
                        // log::warn!("Color: {:?}", color);
                        // log::warn!("UV: {:?}", uv);
                        matrix.as_slice().iter().for_each(|v| { datamatrix.push(*v); });
                        color.as_slice().iter().for_each(|v| { datacolors.push(*v); });
                        datauvs.push(uv.uscale);datauvs.push(uv.vscale);datauvs.push(uv.uoffset);datauvs.push(uv.voffset);
                    }
                });

                if updatebuffer && length == 0 {
                    let mut matrix = Matrix::identity();matrix.append_scaling_mut(0.000001); matrix.append_translation_mut(&Vector3::new(0., -9999999.0, 0.));
                    datamatrix = matrix.as_slice().to_vec();
                    datacolors = Vector4::zeros().as_slice().to_vec();
                    datauvs = Vector4::zeros().as_slice().to_vec();
                }
                
                // ptime1 = pi_time::Instant::now();
                // log::warn!("update_buffer Calc: {:?}", ptime1 - ptime);
                // ptime = ptime1;

                if updatebuffer {
                    if let Ok((enable, idgeo)) = meshes.get(entity) {
                        let id_geo = idgeo.0;
                        if enable.0 {
                            if let Ok(mut buffer) = worldmatrixbuffers.get_mut(id_geo) {
                                // log::warn!("sys_update_buffer A");
                                let data = bytemuck::pod_collect_to_vec(&datamatrix);
                                instance_buffer_update::<InstanceBufferWorldMatrix>(
                                    data, id_geo,
                                    &mut buffer,
                                    &mut slots, &mut allocator,
                                    &device, &queue
                                );
                            }
                            if let Ok(mut buffer) = colorbuffers.get_mut(id_geo) {
                                // log::warn!("sys_update_buffer B");
                                let data = bytemuck::pod_collect_to_vec(&datacolors);
                                instance_buffer_update::<InstanceBufferColor>(
                                    data, id_geo,
                                    &mut buffer,
                                    &mut slots, &mut allocator,
                                    &device, &queue
                                );
                            }
                            if let Ok(mut buffer) = uvbuffers.get_mut(id_geo) {
                                // log::warn!("sys_update_buffer C");
                                let data = bytemuck::pod_collect_to_vec(&datauvs);
                                instance_buffer_update::<InstanceBufferTillOff>(
                                    data, id_geo,
                                    &mut buffer,
                                    &mut slots, &mut allocator,
                                    &device, &queue
                                );
                            }
                        }
                    }
                }
            }
        }
    );

    ptime1 = pi_time::Instant::now();
    // log::warn!("update_buffer: {:?}", ptime1 - ptime);
    // log::warn!("ParticleBuffer: End");
}

pub fn sys_update_buffer_trail(
    trailmodifiers: Query<&ParticleCalculatorTrail>,
    mut particle_sys: Query<
        (&ParticleState, &ParticleSystemTime, &ParticleIDs, &ParticleEmitMatrix, &ParticleBaseRandom, &ParticleColor, &ParticleLocalPosition, &ParticleLocalScaling, &ParticleLocalRotation, &ParticleDirection, &ParticleTrailMesh, &mut ParticleTrail),
    >,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut meshes: Query<&mut RenderGeometryEable>,
    mut trailbuffer: ResMut<ResParticleTrailBuffer>,
    queue: Res<PiRenderQueue>,
) {
    if let Some(trailbuffer) = &mut trailbuffer.0 {
        particle_sys.iter_mut().for_each(
            |(
                state, time, ids, emitmatrixs, randoms, colors, positions, scalings, rotations, directions, trailmesh, mut trails
            )| {
                // log::warn!("Trail Update: 00");
                if let Ok(mut geometry) = geometries.get_mut(trailmesh.geo){
                    if state.playing == false {
                        if let Ok(mut rendergeometry) = meshes.get_mut(trailmesh.mesh) {
                            *rendergeometry = RenderGeometryEable(false);
                        }
                        return;
                    }
                    if let Ok(trailmodifier) = trailmodifiers.get(ids.calculator.0) {
                        let newids = &ids.newids;
                        trails.run_new(newids, randoms, colors, positions, scalings, rotations, emitmatrixs, directions, &trailmodifier.0);

                        let activeids = [ids.actives.clone(), ids.dies.clone()].concat();
                        // log::warn!("Trail Update: {:?}", activeids.len());
                        let (start, end) = trails.run(&activeids, randoms, colors, positions, scalings, rotations, emitmatrixs, time, &trailmodifier.0, trailbuffer);
                    
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
    }
}

pub fn sys_dispose_about_particle_system(
    particles: Query<(Entity, &DisposeReady, &ParticleTrailMesh), Changed<DisposeReady>>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    particles.iter().for_each(|(entity, state, trailmesh)| {
        if state.0 == false { return; }

        disposereadylist.push(OpsDisposeReady::ops(trailmesh.mesh));
        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
