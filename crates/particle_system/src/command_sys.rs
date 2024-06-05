use std::{hash::Hasher, sync::Arc};

use pi_scene_shell::prelude::*;
use pi_scene_context::{geometry::instance::instanced_buffer::InstancedInfoComp, prelude::*};

use crate::{command::*, base::*, extend::format};
pub type ParticleBundle = (
(
    ParticleAttributes,
    ParticleSystemActive,
    ParticleSystemRunningState,
    ParticleSystemModifyState,
    ParticleRandom,
    ParticleSystemTime,
    ParticleSystemEmission,
    ParticleIDs,
),
(
    ParticleBaseRandom,
    ParticleAgeLifetime,
    ParticleDieWaitTime,
    ParticleStartColor,
    ParticleStartScaling,
    ParticleLocalPosition,
    ParticleLocalRotation,
),
(
    ParticleLocalScaling,
    ParticleColorAndUV,
    ParticleEmitMatrix,
    ParticleGravityFactor,
    ParticleForce,
    ParticleVelocity,
    ParticleSpeedFactor,
),
(
    ParticleOrbitVelocity,
    ParticleOrbitOffset,
    ParticleOrbitRadial,
    ParticleLimitVelocityScalar,
    ParticleDirection,
    ParticleCustomV4,
    ParticleTrailMesh,
)
);

pub fn sys_create_particle_calculator(
    mut cmds: ResMut<ActionListCPUParticleCalculator>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleCalculator(entity, cfg)| {
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else { return; };

        // log::warn!("particle_calculator");
        let bundle = format(&cfg);
        entitycmd.insert(bundle);
    });
}

pub fn sys_create_cpu_partilce_system(
    mut cmds: ResMut<ActionListCPUParticleSystem>,
    mut commands: Commands,
    calculators: Query<(&ParticleCalculatorBase, &ParticleCalculatorStartModifiers, &ParticleCalculatorOverLifetime)>,
    trailmodifiers: Query<&ParticleCalculatorTrail>,
    trailbuffer: Res<ResParticleTrailBuffer>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut disposeready: ResMut<ActionListDisposeReadyForRef>,
    mut meshes: ResMut<ActionListMeshStateModify>,
    mut performance: ResMut<ParticleSystemPerformance>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
    mut meshprimitivestate: ResMut<ActionListPrimitiveState>,
    // mut cmdps: Alter<(), (), ParticleBundle, ()>,
    // mut altermodel: Alter<(), (), BundleModel, ()>,
    // mut altergeo: Alter<(), (), BundleGeometry, ()>,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleSystem(id_scene, entity, trailmesh, trailgeo, calculator, attributes, count)| {
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
        if let Ok((
            base, startmodifiers, overlifetime
        )) = calculators.get(idcalculator) {
            // log::warn!("create_cpu_partilce_system");
            let maxcount = base.maxcount;
            performance.maxparticles = (performance.maxparticles.max(maxcount as u32) / 64 + 1) * 64;

            if let Some(val) = base.render_align() {
                meshes.push(OpsMeshStateModify::ops(entity, EMeshStateModify::Alignment(val)));
            }

            let bundle = (
                (
                    attributes,
                    ParticleSystemActive(true),
                    ParticleSystemRunningState(false),
                    ParticleSystemModifyState,
                    ParticleRandom::new(0),
                    ParticleSystemTime::new(performance.frame_time_ms),
                    ParticleSystemEmission::new(),
                    ParticleIDs::new(calculator, maxcount),
                ),
                (
                    ParticleBaseRandom::new(maxcount),
                    ParticleAgeLifetime::new(maxcount),
                    ParticleDieWaitTime::new(maxcount),
                    ParticleStartColor::new(maxcount),
                    ParticleStartScaling::new(maxcount),
                    ParticleLocalPosition::new(maxcount),
                    ParticleLocalRotation::new(maxcount),
                ),
                (
                    ParticleLocalScaling::new(maxcount),
                    ParticleColorAndUV::new(maxcount),
                    ParticleEmitMatrix::new(maxcount, &base.scaling_space, &base.simulation_space),
                    ParticleGravityFactor::new(maxcount, &startmodifiers.gravity, &base.simulation_space),
                    ParticleForce::new(maxcount, overlifetime.force.0.is_local_space, overlifetime.force.0.translation_interpolate.constant()),
                    ParticleVelocity::new(maxcount),
                    ParticleSpeedFactor::new(maxcount),
                ),
                (
                    ParticleOrbitVelocity::new(maxcount, &overlifetime.orbitvelocity),
                    ParticleOrbitOffset::new(maxcount, &overlifetime.orbitoffset),
                    ParticleOrbitRadial::new(maxcount, &overlifetime.orbitradial),
                    ParticleLimitVelocityScalar::new(maxcount),
                    ParticleDirection::new(maxcount),
                    ParticleCustomV4::new(maxcount),
                    ParticleTrailMesh::new(trailmesh, trailgeo),
                )
            );
            entitycmd.insert(bundle);
            // cmdps.alter( entity, bundle);
            if let (Ok(ParticleCalculatorTrail(Some(_))), Some(trailbuffer)) = (trailmodifiers.get(idcalculator), &trailbuffer.0) {
                // log::warn!("Trail Init: ");
                // if trails.contains(entity) == false {
                    let id_mesh = trailmesh;
                    let id_geo = trailgeo;
                    ActionMesh::init(&mut commands, id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0, &commonbindmodel);
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

                    if let Some(mut cmd) = commands.get_entity(id_mesh) {
                        // log::warn!("Mesh Ok");
                        // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                        // cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                        // cmd.insert(CCullMode(CullMode::Off));
                        let bundle = (
                            GeometryID(id_geo),
                            ModelStatic,
                            // 显式重置为默认
                           commonbindmodel.0.clone(),
                        );
                        cmd.insert(bundle);
                    }
                    if let Some(mut geocommands) = commands.get_entity(id_geo) {
                        // log::warn!("Geometry Ok");
                        let vertex_desc = vec![trailbuffer.buffer_desc_billboard()];
                        let (comp1, comp2, comp3, comp4, comp5, comp6) = ActionGeometry::init(&vertex_desc, None, id_mesh);

                        // let mut verticescode = EVerticeExtendCodeComp::default();
                        // verticescode.0.0 += EVerticeExtendCode::TRIAL_BILLBOARD;
                        let slot = AssetDescVBSlot::from(vertex_desc[0].clone());
                        let geo_desc = GeometryDesc { list: vertex_desc };
                        let buffer = AssetResVBSlot::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                        
                        let mut desclist = AssetDescVBSlots::default();
                        let mut keyslist = LoadedKeyVBSlots::default();
                        let mut datalist = AssetResVBSlots::default();
                        keyslist[0] = Some(slot.key().clone());
                        desclist[0] = Some(slot);
                        datalist[0] = Some(buffer);

                        let mut hasher = DefaultHasher::default();
                        geo_desc.hash_resource(&mut hasher);
                        let bundle: BundleGeometry = (
                            comp1,
                            geo_desc,
                            (comp2, comp3, comp4, comp5, comp6, desclist, keyslist, datalist),
                            AssetResBufferIndicesComp(None),
                            InstancedInfoComp(None),
                            GeometryResourceHash(hasher.finish()),
                        );
                        geocommands.insert(bundle);
                        // altergeo.alter(id_geo, bundle);
                    }
                // }
                
                commands.entity(entity).insert((ParticleTrail::new(maxcount), ));
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
    mut trail_cmds: ResMut<ActionListCPUParticleSystemTrailMaterial>,
    trail_items: Query<&ParticleTrailMesh>,
    mut actions: ResMut<ActionListMaterialUse>,
) {
    trail_cmds.drain().drain(..).for_each(|OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count)| {
        if let Ok(trail) = trail_items.get(entity) {
            actions.push(OpsMaterialUse::Use(trail.mesh, idmat, pass));
        } else if count < 8 {
            trail_cmds.push(OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count + 1))
        }
    });
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

// pub fn sys_act_particle_system_trail_material(
//     mut trail_cmds: ResMut<ActionListCPUParticleSystemTrailMaterial>,
//     trail_items: Query<&ParticleTrailMesh>,
//     mut actions: ResMut<ActionListMaterialUse>,
// ) {
//     trail_cmds.drain().drain(..).for_each(|OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count)| {
//         if let Ok(trail) = trail_items.get(entity) {
//             actions.push(OpsMaterialUse::Use(trail.mesh, idmat, pass));
//         } else if count < 8 {
//             trail_cmds.push(OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count + 1))
//         }
//     });
// }
