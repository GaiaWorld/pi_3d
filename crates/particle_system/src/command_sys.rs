
use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::*;

use crate::{command::*, base::*, extend::format};

pub fn sys_create_particle_calculator(
    mut cmds: ResMut<ActionListCPUParticleCalculator>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleCalculator(entity, cfg, _)| {
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
    calculators: Query<&ParticleCalculatorBase>,
    trailmodifiers: Query<&ParticleCalculatorTrail>,
    trailbuffer: Res<ResParticleTrailBuffer>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut disposeready: ResMut<ActionListDisposeReadyForRef>,
    mut meshes: ResMut<ActionListMeshRenderAlignment>,
    mut performance: ResMut<ParticleSystemPerformance>,
    lightlimit: Res<ModelLightLimit>,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleSystem(id_scene, entity, trailmesh, trailgeo, calculator, count)| {
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            // log::warn!("create_cpu_partilce_system CANNT");
            disposeready.push(OpsDisposeReadyForRef::ops(entity));
            disposeready.push(OpsDisposeReadyForRef::ops(trailmesh));
            disposeready.push(OpsDisposeReadyForRef::ops(trailgeo));
            return;
        };

        let idcalculator = calculator.0;
        if let Ok(base) = calculators.get(idcalculator) {
            // log::warn!("create_cpu_partilce_system");
            let maxcount = base.maxcount;
            performance.maxparticles = (performance.maxparticles.max(maxcount as u32) / 64 + 1) * 64;

            let mut vec_vec3_arr: Vec<Vec<Vector3>> = Vec::with_capacity(maxcount);
            for _ in 0..maxcount {
                vec_vec3_arr.push(vec![]);
            }
            if let Some(val) = base.render_align() {
                meshes.push(OpsMeshRenderAlignment::ops(entity, val));
            }

            entitycmd
                .insert(ParticleActive(true))
                .insert(ParticleRunningState(false))
                .insert(ParticleModifyState(false))
                .insert(ParticleRandom::new(0))
                .insert(ParticleSystemTime::new(performance.frame_time_ms))
                .insert(ParticleSystemEmission::new())
                .insert(ParticleIDs::new(calculator, maxcount))
                .insert(ParticleBaseRandom::new(maxcount))
                .insert(ParticleAgeLifetime::new(maxcount))
                .insert(ParticleDieWaitTime::new(maxcount))
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
                .insert(ParticleTrailMesh::new(trailmesh, trailgeo))
                ;
            if let (Ok(_), Some(trailbuffer)) = (trailmodifiers.get(idcalculator), &trailbuffer.0) {
                // log::warn!("Trail Init: ");
                // if trails.contains(entity) == false {
                    let id_mesh = trailmesh;
                    let id_geo = trailgeo;
                    ActionMesh::init(&mut commands, id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0);
        
                    if let Some(mut cmd) = commands.get_entity(id_mesh) {
                        // log::warn!("Mesh Ok");
                        // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                        cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                        cmd.insert(CCullMode(CullMode::Off));
                        cmd.insert(GeometryID(id_geo));
                    }
                    if let Some(mut cmd) = commands.get_entity(id_geo) {
                        // log::warn!("Geometry Ok");
                        let vertex_desc = vec![trailbuffer.buffer_desc()];
                        ActionGeometry::init(&mut cmd, &vertex_desc, None, id_mesh);

                        let mut verticescode = EVerticeExtendCodeComp::default();
                        verticescode.0.0 += EVerticeExtendCode::TRIAL_BILLBOARD;
                        let slot = AssetDescVBSlot01::from(vertex_desc[0].clone());
                        let geo_desc = GeometryDesc { list: vertex_desc };
                        let buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                        
                        cmd
                            .insert(geo_desc)
                            .insert(slot)
                            .insert(buffer)
                            .insert(verticescode)
                            ;
                    }
                // }
                
                commands.entity(entity).insert(ParticleTrail::new(maxcount));
            }
        } else if count < 2 {
            // log::warn!("create_cpu_partilce_system FAIL");
            cmds.push(OpsCPUParticleSystem(id_scene, entity, trailmesh, trailgeo, calculator, count + 1));
        } else {
            disposeready.push(OpsDisposeReadyForRef::ops(entity));
            disposeready.push(OpsDisposeReadyForRef::ops(trailmesh));
            disposeready.push(OpsDisposeReadyForRef::ops(trailgeo));
        }

    });
}

pub fn sys_act_partilce_system_state(
    mut cmds: ResMut<ActionListCPUParticleSystemState>,
    mut items: Query<(&mut ParticleActive, &mut ParticleSystemTime)>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsCPUParticleSystemState::Start(entity, count) => {
                if let Ok((mut active, _)) = items.get_mut(entity) {
                    active.0 = true;
                } else if count < 2 {
                    cmds.push(OpsCPUParticleSystemState::Start(entity, count + 1));
                }
            },
            OpsCPUParticleSystemState::TimeScale(entity, timescale, count) => {
                if let Ok((_, mut time)) = items.get_mut(entity) {
                    time.time_scale = timescale;
                } else if count < 2 {
                    cmds.push(OpsCPUParticleSystemState::TimeScale(entity, timescale, count + 1));
                }
            },
            OpsCPUParticleSystemState::Stop(entity, count) => {
                if let Ok((mut active, _)) = items.get_mut(entity) {
                    active.0 = false;
                } else if count < 2 {
                    cmds.push(OpsCPUParticleSystemState::Stop(entity, count + 1));
                }
            },
        }
    });
}

pub fn sys_act_particle_system_trail_material(
    mut cmds: ResMut<ActionListCPUParticleSystemTrailMaterial>,
    items: Query<&ParticleTrailMesh>,
    mut actions: ResMut<ActionListMaterialUse>,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count)| {
        if let Ok(trail) = items.get(entity) {
            actions.push(OpsMaterialUse::Use(trail.mesh, idmat, pass));
        } else if count < 8 {
            cmds.push(OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count + 1))
        }
    });
}
