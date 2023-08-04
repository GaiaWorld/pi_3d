
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::{*, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolVector3}};

use crate::{base::*, interpolation::IInterpolation, tools::{Random, Velocity}};

/// 系统的启动
pub fn sys_emission(
    scenes: Query<&SceneTime>,
    calculators: Query<(&ParticleCalculatorBase, &ParticleCalculatorEmission)>,
    mut particle_sys: Query<(&SceneID, &mut ParticleRandom, &mut ParticleIDs, &mut ParticleSystemTime, &mut ParticleSystemEmission, &mut ParticleBaseRandom, &mut ParticleAgeLifetime)>,
) {
    particle_sys.iter_mut().for_each(|(idscene, mut random, mut ids, mut particlesystime, mut emission, mut randoms, mut agelifetime)| {
        if let (Ok(scenetime), Ok((base, calcemission))) = (scenes.get(idscene.0), calculators.get(ids.calculator.0)) {
            let delta_ms = scenetime.delta_ms() as u32;

            particlesystime.run(delta_ms, 1000, base.duration);
            
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
    mut particle_sys: Query<(&LocalScaling, &WorldMatrix, &mut GlobalTransform, &ParticleIDs, &ParticleSystemTime, &mut ParticleEmitMatrix)>,
) {
    particle_sys.iter_mut().for_each(|(local_scaling, world_matrix, mut transform, ids, time, mut emitmatrix)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(base) = calculators.get(ids.calculator.0) {
            let newids = &ids.newids;
            let activeids = &ids.actives;
            let global_rotation = transform.rotation().clone();
            let global_scaling = transform.scaling().clone();
            let global_position = transform.position();
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
    mut particle_sys: Query<(&ParticleIDs, &ParticleSystemTime, &ParticleBaseRandom, &mut ParticleAgeLifetime)>,
) {
    particle_sys.iter_mut().for_each(|(ids, time, randoms, mut items)| {
        if time.running_delta_ms <= 0 { return; }

        if let Ok(calculator) = calculators.get(ids.calculator.0) {
            let calculator = &calculator.0;
            let newids = &ids.newids;
            let activeids = &ids.actives;
            items.start(time, newids, calculator, randoms);
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
            let activeids = &ids.actives;
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
            let activeids = &ids.actives;
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
            let activeids = &ids.actives;
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
    mut particle_sys: Query<(&mut ParticleIDs, &ParticleAgeLifetime, &ParticleSystemTime)>,
) {
    particle_sys.iter_mut().for_each(|(mut ids, ages, time)| {
        if time.running_delta_ms <= 0 { return; }

        ids.newids.clear();

        let mut actives = ids.actives.clone();
        ids.actives.clear();
        actives.drain(..).for_each(|idx| {
            let age = ages.get(idx).unwrap();
            // log::warn!("Age: {:?}, Lifetime: {:?}", age.age, age.lifetime);
            if age.age <= age.lifetime {
                ids.actives.push(idx);
            } else {
                ids.unactives.push(idx);
            }
        });

        // log::warn!("actives: {:?}", ids.actives);
    });
}

pub fn sys_update_buffer(
    calculators: Query<&ParticleCalculatorBase>,
    texturesheets: Query<&ParticleCalculatorTextureSheet>,
    particle_sys: Query<
        (Entity, &ParticleSystemTime, &ParticleIDs, &ParticleLocalScaling, &ParticleLocalRotation, &ParticleLocalPosition, &ParticleDirection, &ParticleEmitMatrix, &ParticleColor, &ParticleUV),
    >,
    meshes: Query<(&GlobalEnable, &GeometryID)>,
    mut worldmatrixbuffers: Query<&mut InstanceBufferWorldMatrix>,
    mut colorbuffers: Query<&mut InstanceBufferColor>,
    mut uvbuffers: Query<&mut InstanceBufferTillOff>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
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
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    let mut ptime = pi_time::Instant::now();
    let mut ptime1 = pi_time::Instant::now();

    particle_sys.iter().for_each(
        |(
            entity, time, ids, scalings, rotations, positions, directions, emitmatrixs,
            colors, uvs
        )| {
            if time.running_delta_ms <= 0 { return; }

            let length = ids.actives.len();

            // log::warn!("sys_update_buffer");
            if let Ok(calculator) = calculators.get(ids.calculator.0) {
                let mut datamatrix: Vec<f32> = Vec::with_capacity(length * 16);
                let mut datacolors: Vec<f32> = Vec::with_capacity(length * 4);
                let mut datauvs: Vec<f32> = Vec::with_capacity(length * 4);
                let mut pose = Matrix::identity();
                
                let renderalign = calculator.render_align();
                let updatebuffer = renderalign.is_some();
                // log::warn!("ActiveCount: {:?}", length);

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

                    CoordinateSytem3::transform_coordinates(&direction.value, &emitmatrix.pose, &mut g_velocity);

                    let matrix = if let Some(renderalign) = renderalign {
                        let mut matrix = match renderalign {
                            ERenderAlignment::Velocity => {
                                let rotation = renderalign.calc_rotation(&emitmatrix.rotation, emitmatrix.eulers, &g_velocity);
                                let mut matrix = Matrix::identity();
                                CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &rotation, &emitmatrix.position, &mut matrix);
                                matrix
                            },
                            ERenderAlignment::StretchedBillboard => {
                                let rotation = renderalign.calc_rotation(&emitmatrix.rotation, emitmatrix.eulers, &g_velocity);
                                let mut matrix = Matrix::identity();
                                CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &rotation, &emitmatrix.position, &mut matrix);
                                matrix
                            }
                            _ => {
                                let matrix = emitmatrix.pose.append_translation(&emitmatrix.position);
                                matrix
                            }
                        };
                        
                        let mut local = Matrix::identity();
                        CoordinateSytem3::matrix4_compose_euler_angle(scaling, eulers, &translation, &mut local);
                        // log::warn!("a MAREIX: {:?}", matrix);
            
                        matrix = matrix * local;
            
                        if let Some(local) = renderalign.calc_local(&g_velocity) {
                            matrix = matrix * local;
                        }
                        matrix
                    } else {
                        let mut matrix = Matrix::identity();
                        CoordinateSytem3::matrix4_compose_rotation(&emitmatrix.scaling, &emitmatrix.rotation, &emitmatrix.position, &mut matrix);
                        let mut local = Matrix::identity();
                        CoordinateSytem3::matrix4_compose_euler_angle(scaling, eulers, &translation, &mut local);
                        // log::warn!("MAREIX: {:?}", matrix);
                        // log::warn!("LOCAL: {:?}", local);
                        matrix = matrix * local;
                        matrix
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
                    datamatrix = Matrix::identity().as_slice().to_vec();
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
                                    &mut buffer, &mut geoloader, &mut vb_data_map,
                                    &mut slots, &mut allocator, &asset_mgr,
                                    &device, &queue
                                );
                            }
                            if let Ok(mut buffer) = colorbuffers.get_mut(id_geo) {
                                // log::warn!("sys_update_buffer B");
                                let data = bytemuck::pod_collect_to_vec(&datacolors);
                                instance_buffer_update::<InstanceBufferColor>(
                                    data, id_geo,
                                    &mut buffer, &mut geoloader, &mut vb_data_map,
                                    &mut slots, &mut allocator, &asset_mgr,
                                    &device, &queue
                                );
                            }
                            if let Ok(mut buffer) = uvbuffers.get_mut(id_geo) {
                                // log::warn!("sys_update_buffer C");
                                let data = bytemuck::pod_collect_to_vec(&datauvs);
                                instance_buffer_update::<InstanceBufferTillOff>(
                                    data, id_geo,
                                    &mut buffer, &mut geoloader, &mut vb_data_map,
                                    &mut slots, &mut allocator, &asset_mgr,
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
}