// use std::{hash::Hasher, sync::Arc};

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
    ParticleEmitMatrix,
),
(
    ParticleBaseRandom,
    ParticleDieWaitTime,
    ParticleStart,
    ParticleLocal,
    ParticleVelocityAndForce,
),
(
    ParticleOrbitVelocity,
    ParticleOrbitOffset,
    ParticleOrbitRadial,
    ParticleDirection,
    ParticleCustomV4,
    ParticleTrailMesh,
)
);

pub fn sys_create_particle_calculator(
    mut cmds: ResMut<ActionListCPUParticleCalculator>,
    mut commands: Commands,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_create_particle_calculator"));
    cmds.drain().for_each(|OpsCPUParticleCalculator(entity, cfg)| {
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
    mut psperformance: ResMut<ParticleSystemPerformance>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
    mut meshprimitivestate: ResMut<ActionListRenderState>,
    mut cmdps: Alter<(), (), ParticleBundle, ()>,
    mut altermodel: Alter<(), (), (BundleModel, BindModel, ModelBindDefines, ModelMatIdxs, PassIDs, ModelStatic), ()>,
    
    mut passinsert: Insert<(BundleEntity, PassObjInitBundle, PassTag)>,
    mut altergeo: Alter<(), (), BundleGeometry, ()>,
    engineopt: Res<EngineCustomPlugins>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_create_cpu_partilce_system"));
    cmds.drain().for_each(|OpsCPUParticleSystem(id_scene, entity, trailmesh, trailgeo, calculator, attributes, update_buffer_interval_frame)| {
        let mut _entitycmd = if let Some(cmd) = commands.get_entity(entity) {
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
            psperformance.maxparticles = (psperformance.maxparticles.max(maxcount as u32) / 64 + 1) * 64;

            if let Some(val) = base.render_align() {
                meshes.push(OpsMeshStateModify::ops(entity, EMeshStateModify::Alignment(val)));
            }

            let bundle = (
                (
                    attributes,
                    ParticleSystemActive(true),
                    ParticleSystemRunningState { isrunning: false, deltatime: 0, update_buffer_interval_frame, waitframe: 0, updatebuffer: false },
                    ParticleSystemModifyState,
                    ParticleRandom::new(0),
                    ParticleSystemTime::new(psperformance.frame_time_ms),
                    ParticleSystemEmission::new(),
                    ParticleIDs::new(calculator, maxcount),
                    ParticleEmitMatrix::new(maxcount, &base.scaling_space, &base.simulation_space),
                ),
                (
                    ParticleBaseRandom::new(maxcount),
                    ParticleDieWaitTime::new(maxcount),
                    ParticleStart::new(maxcount),
                    ParticleLocal::new(maxcount),
                    ParticleVelocityAndForce::new(maxcount, overlifetime.force.0.is_local_space, overlifetime.force.0.translation_interpolate.constant(), &startmodifiers.gravity, &base.simulation_space),
                ),
                (
                    ParticleOrbitVelocity::new(maxcount, &overlifetime.orbitvelocity),
                    ParticleOrbitOffset::new(maxcount, &overlifetime.orbitoffset),
                    ParticleOrbitRadial::new(maxcount, &overlifetime.orbitradial),
                    ParticleDirection::new(maxcount),
                    ParticleCustomV4::new(maxcount),
                    ParticleTrailMesh::new(trailmesh, trailgeo),
                )
            );
            // entitycmd.insert(bundle);
            let _ = cmdps.alter( entity, bundle);
            if let (Ok(ParticleCalculatorTrail(Some(_))), Some(trailbuffer)) = (trailmodifiers.get(idcalculator), &trailbuffer.0) {
                // log::warn!("Trail Init: ");
                // if trails.contains(entity) == false {
                    let id_mesh = trailmesh;
                    let id_geo = trailgeo;
                    ActionMesh::init(
                        id_mesh, &mut commands, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0, &commonbindmodel,
                        &mut altermodel, &mut passinsert, &engineopt
                    );
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_01, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_02, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_03, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_04, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_05, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_06, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_07, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_08, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_01, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_02, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_03, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_04, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_05, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_06, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_07, EPrimitiveState::CCullMode(CullMode::Off)));
                    meshprimitivestate.push(OpsRenderState::primitive_state(id_mesh, PassTag::PASS_TAG_08, EPrimitiveState::CCullMode(CullMode::Off)));

                    if let Some(mut cmd) = commands.get_entity(id_mesh) {
                        // log::warn!("Mesh Ok");
                        // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                        // cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                        // cmd.insert(CCullMode(CullMode::Off));
                        let bundle = (
                            GeometryID(id_geo),
                            // ModelStatic,
                            // 显式重置为默认
                           commonbindmodel.0.clone(),
                           ModelBindDefines::default()
                        );
                        cmd.insert(bundle);
                    }
                    if let Some(mut _geocommands) = commands.get_entity(id_geo) {
                        // log::warn!("Geometry Ok");
                        let vertex_desc = vec![trailbuffer.buffer_desc_billboard()];
                        let (comp1, comp2, comp3, comp4, comp5, comp6) = ActionGeometry::init(&vertex_desc, None, id_mesh);

                        // let mut verticescode = EVerticeExtendCodeComp::default();
                        // verticescode.0.0 += EVerticeExtendCode::TRIAL_BILLBOARD;
                        let slot = AssetDescVBSlot::from(vertex_desc[0].clone());
                        let geo_desc = GeometryDesc { list: vertex_desc };
                        let buffer = AssetResVBSlot::from(EVerticesBufferUsage::EVBRange(Share::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                        
                        let mut desclist = AssetDescVBSlots::default();
                        let mut keyslist = LoadedKeyVBSlots::default();
                        let mut datalist = AssetResVBSlots::default();
                        keyslist[0] = Some(slot.key().clone());
                        desclist[0] = Some(slot);
                        datalist[0] = Some(buffer);

                        let bundle: BundleGeometry = (
                            comp1,
                            geo_desc,
                            (comp2, comp3, comp4, comp5, comp6, desclist, keyslist, datalist),
                            AssetResBufferIndicesComp(None),
                            InstancedInfoComp(None),
                            FlagGeometryDirty,
                        );
                        // geocommands.insert(bundle);
                        let _ = altergeo.alter(id_geo, bundle);
                    }
                // }
                
                commands.entity(entity).insert((ParticleTrail::new(maxcount), ));
            }
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
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_partilce_system_state"));
    trail_cmds.drain().for_each(|OpsCPUParticleSystemTrailMaterial(entity, idmat, pass)| {
        if let Ok(trail) = trail_items.get(entity) {
            actions.push(OpsMaterialUse::Use(trail.mesh, idmat, pass));
        }
    });
    cmds.drain().for_each(|OpsCPUParticleSystemState(entity, cmd)| {
        match cmd {
            ECPUParticleSystemState::Start() => {
                if let Ok((mut active, _)) = items.get_mut(entity) {
                    active.0 = true;
                }
            },
            ECPUParticleSystemState::TimeScale( timescale) => {
                if let Ok((_, mut time)) = items.get_mut(entity) {
                    time.time_scale = timescale;
                }
            },
            ECPUParticleSystemState::Stop() => {
                if let Ok((mut active, _)) = items.get_mut(entity) {
                    active.0 = false;
                }
            },
        }
    });
}
