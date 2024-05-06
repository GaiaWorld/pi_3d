use std::{hash::Hasher, sync::Arc};

use pi_scene_shell::prelude::*;
use pi_scene_context::{geometry::instance::{types::InstanceAttributeAnimated, DirtyInstanceSourceForSingleBuffer}, prelude::*, transforms::command_sys::ActionTransformNodeBundle};

use crate::{command::*, base::*, extend::format};

pub fn sys_create_particle_calculator(
    mut cmds: ResMut<ActionListCPUParticleCalculator>,
    // mut commands: Commands,
    // cmds: &mut EntityCommands, 
    mut alter1: Alter<(), (), (ParticleCalculatorBase,)>,
    mut alter2: Alter<(), (), (ParticleCalculatorStartModifiers,)>,
    mut alter3: Alter<(), (), (ParticleCalculatorOverLifetime,)>,
    mut alter4: Alter<(), (), (ParticleCalculatorCustomV4,)>,
    mut alter5: Alter<(), (), (ParticleCalculatorTrail,)>,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleCalculator(entity, cfg)| {
        /* let mut entitycmd = */ if /* let Some(cmd) = */ alter1.get(entity).is_err() {
        /*     cmd
        } else { */ return; };

        // log::warn!("particle_calculator");
        format(entity, &mut alter1, &mut alter2, &mut alter3, &mut alter4, &mut alter5, &cfg);
    });
}

pub type CpuPartilceBundle = (
    ParticleAttributes,
    ParticleSystemActive,
    ParticleSystemRunningState,
    ParticleSystemModifyState,
    ParticleRandom,
    ParticleSystemTime,
    ParticleSystemEmission,
    ParticleIDs,
    ParticleBaseRandom,
    ParticleAgeLifetime,
    ParticleDieWaitTime,
    ParticleStartColor,
    ParticleStartScaling,
    ParticleLocalPosition,
    ParticleLocalRotation,
    ParticleLocalScaling,
    ParticleColorAndUV,
    ParticleEmitMatrix,
    ParticleGravityFactor,
    ParticleForce,
    ParticleVelocity,
    ParticleSpeedFactor,
    ParticleOrbitVelocity,
    ParticleOrbitOffset,
    ParticleOrbitRadial,
    ParticleLimitVelocityScalar,
    ParticleDirection,
    ParticleCustomV4,
    ParticleTrailMesh,
);
pub fn sys_create_cpu_partilce_system(
    mut cmds: ResMut<ActionListCPUParticleSystem>,
    // mut commands: Commands,
    mut insert: Insert<()>,
    mut alter1: Alter<(), (), (DisposeReady, DisposeCan),>,
    mut alter2: Alter<(), (), (SceneID,)>,
    mut alter3: Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable)>,
    mut alter4: Alter<(), (), ActionTransformNodeBundle>,
    // mut alter5: Alter<(), (), (InstanceSourceRefs, DirtyInstanceSourceRefs, DirtyInstanceSourceForSingleBuffer)>,
    mut alter6: Alter<(), (), (InstanceSourceRefs, DirtyInstanceSourceRefs, DirtyInstanceSourceForSingleBuffer)>,
    mut alter7: Alter<(), (), (TargetAnimatorableIsRunning, InstanceAttributeAnimated)>,
    mut alter8: Alter<(), (), (BindModel, ModelStatic)>,
    mut alter9: Alter<(), (), (BindModel,)>,
    mut alter10: Alter<(), (), ActionMeshInitBundle>,
    mut alter11: Alter<(), (), (DisposeReady, DisposeCan)>,
    mut alter12: Alter<(), (), ActionPassObjectInitBundle>, 
    mut alter13: Alter<(), (), (PassTag,), ()>,
    mut alter14: Alter<(), (), ActionMeshBundle>,
    mut alter15: Alter<(), (), (PassIDs,), ()>,
    mut alter16: Alter<(), (), (DisposeReady, DisposeCan), ()>,
    mut alter17: Alter<(), (), (VertexBufferLayoutsComp, MeshID, RenderGeometryComp), ()>,
    mut alter18: Alter<(), (), (IndicesBufferDesc,), ()>,
    mut alter19: Alter<(), (), (), (IndicesBufferDesc, )>,
    mut alter20: Alter<(), (), CpuPartilceBundle, ()>,
    mut alter21: Alter<(), (), (GeometryID, ModelStatic, BindModel), ()>,
    mut alter22: Alter<(), (), (GeometryResourceHash, GeometryDesc, AssetDescVBSlot01, AssetResVBSlot01), ()>,
    mut alter23: Alter<(), (), (ParticleTrail,), ()>,
    calculators: Query<(&ParticleCalculatorBase, &ParticleCalculatorStartModifiers, &ParticleCalculatorOverLifetime)>,
    trailmodifiers: Query<&ParticleCalculatorTrail>,
    trailbuffer: Res<ResParticleTrailBuffer>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut disposeready: ResMut<ActionListDisposeReadyForRef>,
    mut meshes: ResMut<ActionListMeshRenderAlignment>,
    mut performance: ResMut<ParticleSystemPerformance>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
    mut meshprimitivestate: ResMut<ActionListPrimitiveState>,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleSystem(id_scene, entity, trailmesh, trailgeo, calculator, attributes, count)| {
        /* let mut entitycmd =  */if  alter20.get(entity).is_err() {
      /*       cmd
        } else { */
            // log::warn!("create_cpu_partilce_system CANNT");
            disposeready.push(OpsDisposeReadyForRef::ops(entity));
            disposeready.push(OpsDisposeReadyForRef::ops(trailmesh));
            disposeready.push(OpsDisposeReadyForRef::ops(trailgeo));
            return;
        };

        let idcalculator = calculator.0;
        if let Ok((
            base, startmodifiers, overlifetime
        )) = calculators.get(idcalculator) {
            // log::warn!("create_cpu_partilce_system");
            let maxcount = base.maxcount;
            performance.maxparticles = (performance.maxparticles.max(maxcount as u32) / 64 + 1) * 64;

            if let Some(val) = base.render_align() {
                meshes.push(OpsMeshRenderAlignment::ops(entity, val));
            }

            alter20.alter(entity, 
                (attributes,
                ParticleSystemActive(true),
                ParticleSystemRunningState(false),
                ParticleSystemModifyState,
                ParticleRandom::new(0),
                ParticleSystemTime::new(performance.frame_time_ms),
                ParticleSystemEmission::new(),
                ParticleIDs::new(calculator, maxcount),
                ParticleBaseRandom::new(maxcount),
                ParticleAgeLifetime::new(maxcount),
                ParticleDieWaitTime::new(maxcount),
                ParticleStartColor::new(maxcount),
                ParticleStartScaling::new(maxcount),
                ParticleLocalPosition::new(maxcount),
                ParticleLocalRotation::new(maxcount),
                ParticleLocalScaling::new(maxcount),
                ParticleColorAndUV::new(maxcount),
                ParticleEmitMatrix::new(maxcount, &base.scaling_space, &base.simulation_space),
                ParticleGravityFactor::new(maxcount, &startmodifiers.gravity, &base.simulation_space),
                ParticleForce::new(maxcount, overlifetime.force.0.is_local_space, overlifetime.force.0.translation_interpolate.constant()),
                ParticleVelocity::new(maxcount),
                ParticleSpeedFactor::new(maxcount),
                ParticleOrbitVelocity::new(maxcount, &overlifetime.orbitvelocity),
                ParticleOrbitOffset::new(maxcount, &overlifetime.orbitoffset),
                ParticleOrbitRadial::new(maxcount, &overlifetime.orbitradial),
                ParticleLimitVelocityScalar::new(maxcount),
                ParticleDirection::new(maxcount),
                ParticleCustomV4::new(maxcount),
                ParticleTrailMesh::new(trailmesh, trailgeo)),
            );
            if let (Ok(_), Some(trailbuffer)) = (trailmodifiers.get(idcalculator), &trailbuffer.0) {
                // log::warn!("Trail Init: ");
                // if trails.contains(entity) == false {
                    let id_mesh = trailmesh;
                    let id_geo = trailgeo;
                    ActionMesh::init(&mut insert, &mut alter1, &mut alter2, &mut alter3, &mut alter4, &mut alter6, &mut alter7, &mut alter8, &mut alter9, &mut alter10, &mut alter11, &mut alter12, &mut alter13, &mut alter14, &mut alter15,  id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0, &commonbindmodel);
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_01, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_02, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_03, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_04, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_05, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_06, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_07, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_08, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_01, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_02, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_03, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_04, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_05, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_06, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_07, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_08, EPrimitiveState::CCullMode(CullMode::Off)));

                    if /* let Some(mut cmd) =  */alter21.get(id_mesh).is_ok() {
                        // log::warn!("Mesh Ok");
                        // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                        // cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                        // cmd.insert(CCullMode(CullMode::Off));
                        // cmd.insert(GeometryID(id_geo));
                        // cmd.insert(ModelStatic);
                        // // 显式重置为默认
                        // cmd.insert(commonbindmodel.0.clone());
                        alter21.alter(id_mesh, (GeometryID(id_geo), ModelStatic, commonbindmodel.0.clone()));
                    }
                    if /* let Some(mut cmd) = */ alter22.get(id_geo).is_ok() {
                        // log::warn!("Geometry Ok");
                        let vertex_desc = vec![trailbuffer.buffer_desc_billboard()];
                        ActionGeometry::init(id_geo, &mut alter16, &mut alter17, &mut alter18, &mut alter19, &vertex_desc, None, id_mesh);

                        // let mut verticescode = EVerticeExtendCodeComp::default();
                        // verticescode.0.0 += EVerticeExtendCode::TRIAL_BILLBOARD;
                        let slot = AssetDescVBSlot01::from(vertex_desc[0].clone());
                        let geo_desc = GeometryDesc { list: vertex_desc };
                        let buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                        
                        let mut hasher = DefaultHasher::default();
                        geo_desc.hash_resource(&mut hasher);
                        // cmd.insert(GeometryResourceHash(hasher.finish()));

                        // cmd
                        //     .insert(geo_desc)
                        //     .insert(slot)
                        //     .insert(buffer)
                        //     // .insert(verticescode)
                        //     ;
                            alter22.alter(id_geo, (GeometryResourceHash(hasher.finish()), geo_desc, slot, buffer ));
                    }
                // }
                
                alter23.alter(entity, (ParticleTrail::new(maxcount), ));
            }
        } else if count < 2 {
            // log::warn!("create_cpu_partilce_system FAIL");
            cmds.push(OpsCPUParticleSystem(id_scene, entity, trailmesh, trailgeo, calculator, attributes, count + 1));
        } else {
            disposeready.push(OpsDisposeReadyForRef::ops(entity));
            disposeready.push(OpsDisposeReadyForRef::ops(trailmesh));
            disposeready.push(OpsDisposeReadyForRef::ops(trailgeo));
        }

    });
}

pub fn sys_act_partilce_system_state(
    mut cmds: ResMut<ActionListCPUParticleSystemState>,
    mut items: Query<(&mut ParticleSystemActive, &mut ParticleSystemTime)>,
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
