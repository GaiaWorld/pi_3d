use std::{hash::Hasher, sync::Arc};

use pi_scene_shell::prelude::*;
use pi_scene_context::{geometry::instance::{types::InstanceAttributeAnimated, DirtyInstanceSourceForSingleBuffer}, pass::pi_world::editor::EntityEditor, prelude::*, transforms::command_sys::ActionTransformNodeBundle};

use crate::{command::*, base::*, extend::format};

pub fn sys_create_particle_calculator(
    mut cmds: ResMut<ActionListCPUParticleCalculator>,
    // mut commands: Commands,
    // cmds: &mut EntityCommands, 
    mut editor: EntityEditor,
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleCalculator(entity, cfg)| {
        /* let mut entitycmd = */ if /* let Some(cmd) = */ !editor.contains_entity(entity) {
        /*     cmd
        } else { */ return; };

        // log::warn!("particle_calculator");
        format(entity, &mut editor, &cfg);
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
    mut editor: EntityEditor,
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
) {
    cmds.drain().drain(..).for_each(|OpsCPUParticleSystem(id_scene, entity, trailmesh, trailgeo, calculator, attributes, count)| {
        /* let mut entitycmd =  */if !editor.contains_entity(entity) {
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
                meshes.push(OpsMeshStateModify::ops(entity, EMeshStateModify::Alignment(val)));
            }

            let components = [
                editor.init_component::<ParticleAttributes>(),
                editor.init_component::<ParticleSystemActive>(),
                editor.init_component::<ParticleSystemRunningState>(),
                editor.init_component::<ParticleSystemModifyState>(),
                editor.init_component::<ParticleRandom>(),
                editor.init_component::<ParticleSystemTime>(),
                editor.init_component::<ParticleSystemEmission>(),
                editor.init_component::<ParticleIDs>(),
                editor.init_component::<ParticleBaseRandom>(),
                editor.init_component::<ParticleAgeLifetime>(),
                editor.init_component::<ParticleDieWaitTime>(),
                editor.init_component::<ParticleStartColor>(),
                editor.init_component::<ParticleStartScaling>(),
                editor.init_component::<ParticleLocalPosition>(),
                editor.init_component::<ParticleLocalRotation>(),
                editor.init_component::<ParticleLocalScaling>(),
                editor.init_component::<ParticleColorAndUV>(),
                editor.init_component::<ParticleEmitMatrix>(),
                editor.init_component::<ParticleGravityFactor>(),
                editor.init_component::<ParticleForce>(),
                editor.init_component::<ParticleVelocity>(),
                editor.init_component::<ParticleSpeedFactor>(),
                editor.init_component::<ParticleOrbitVelocity>(),
                editor.init_component::<ParticleOrbitOffset>(),
                editor.init_component::<ParticleOrbitRadial>(),
                editor.init_component::<ParticleLimitVelocityScalar>(),
                editor.init_component::<ParticleDirection>(),
                editor.init_component::<ParticleCustomV4>(),
                editor.init_component::<ParticleTrailMesh>(),
            ];
            editor.add_components(entity, &components);

            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = attributes;
         
            *editor.get_component_unchecked_mut_by_id(entity, components[0])  =    ParticleSystemActive(true);
            *editor.get_component_unchecked_mut_by_id(entity, components[1])  =    ParticleSystemRunningState(false);
            // *editor.get_component_unchecked_mut_by_id(entity, components[2])  =    ParticleSystemModifyState;
            *editor.get_component_unchecked_mut_by_id(entity, components[3])  =    ParticleRandom::new(0);
            *editor.get_component_unchecked_mut_by_id(entity, components[4])  =    ParticleSystemTime::new(performance.frame_time_ms);
            // *editor.get_component_unchecked_mut_by_id(entity, components[5])  =    ParticleSystemEmission::new();
            *editor.get_component_unchecked_mut_by_id(entity, components[6]) =    ParticleIDs::new(calculator, maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[7]) =    ParticleBaseRandom::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[8]) =    ParticleAgeLifetime::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[9]) =    ParticleDieWaitTime::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[10]) =    ParticleStartColor::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[11]) =    ParticleStartScaling::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[12]) =    ParticleLocalPosition::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[13]) =    ParticleLocalRotation::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[14]) =    ParticleLocalScaling::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[15]) =    ParticleColorAndUV::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[16]) =    ParticleEmitMatrix::new(maxcount, &base.scaling_space, &base.simulation_space);
            *editor.get_component_unchecked_mut_by_id(entity, components[17]) =    ParticleGravityFactor::new(maxcount, &startmodifiers.gravity, &base.simulation_space);
            *editor.get_component_unchecked_mut_by_id(entity, components[18]) =    ParticleForce::new(maxcount, overlifetime.force.0.is_local_space, overlifetime.force.0.translation_interpolate.constant());
            *editor.get_component_unchecked_mut_by_id(entity, components[19]) =    ParticleVelocity::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[20]) =    ParticleSpeedFactor::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[21]) =    ParticleOrbitVelocity::new(maxcount, &overlifetime.orbitvelocity);
            *editor.get_component_unchecked_mut_by_id(entity, components[22]) =    ParticleOrbitOffset::new(maxcount, &overlifetime.orbitoffset);
            *editor.get_component_unchecked_mut_by_id(entity, components[23]) =    ParticleOrbitRadial::new(maxcount, &overlifetime.orbitradial);
            *editor.get_component_unchecked_mut_by_id(entity, components[24]) =    ParticleLimitVelocityScalar::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[25]) =    ParticleDirection::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[26]) =    ParticleCustomV4::new(maxcount);
            *editor.get_component_unchecked_mut_by_id(entity, components[27]) =    ParticleTrailMesh::new(trailmesh, trailgeo);
            
            if let (Ok(_), Some(trailbuffer)) = (trailmodifiers.get(idcalculator), &trailbuffer.0) {
                // log::warn!("Trail Init: ");
                // if trails.contains(entity) == false {
                    let id_mesh = trailmesh;
                    let id_geo = trailgeo;
                    ActionMesh::init( &mut editor,  id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0, &commonbindmodel);
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

                //     if /* let Some(mut cmd) =  */alter21.get(id_mesh).is_ok() {
                //         // log::warn!("Mesh Ok");
                //         // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                //         // cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                //         // cmd.insert(CCullMode(CullMode::Off));
                //         // cmd.insert(GeometryID(id_geo));
                //         // cmd.insert(ModelStatic);
                //         // // 显式重置为默认
                //         // cmd.insert(commonbindmodel.0.clone());
                //         alter21.alter(id_mesh, (GeometryID(id_geo), ModelStatic, commonbindmodel.0.clone()));
                //     }
                //     if /* let Some(mut cmd) = */ alter22.get(id_geo).is_ok() {
                //         // log::warn!("Geometry Ok");
                //         let vertex_desc = vec![trailbuffer.buffer_desc_billboard()];
                //         ActionGeometry::init(id_geo, &mut alter16, &mut alter17, &mut alter18, &mut alter19, &vertex_desc, None, id_mesh);

                //         // let mut verticescode = EVerticeExtendCodeComp::default();
                //         // verticescode.0.0 += EVerticeExtendCode::TRIAL_BILLBOARD;
                //         let slot = AssetDescVBSlot01::from(vertex_desc[0].clone());
                //         let geo_desc = GeometryDesc { list: vertex_desc };
                //         let buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                        
                //         let mut hasher = DefaultHasher::default();
                //         geo_desc.hash_resource(&mut hasher);
                //         // cmd.insert(GeometryResourceHash(hasher.finish()));

                //         // cmd
                //         //     .insert(geo_desc)
                //         //     .insert(slot)
                //         //     .insert(buffer)
                //         //     // .insert(verticescode)
                //         //     ;
                //             alter22.alter(id_geo, (GeometryResourceHash(hasher.finish()), geo_desc, slot, buffer ));
                //     }
                // // }
                
                // alter23.alter(entity, (ParticleTrail::new(maxcount), ));
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

// pub fn sys_act_particle_system_trail_material(
//     mut cmds: ResMut<ActionListCPUParticleSystemTrailMaterial>,
//     items: Query<&ParticleTrailMesh>,
//     mut actions: ResMut<ActionListMaterialUse>,
// ) {
//     cmds.drain().drain(..).for_each(|OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count)| {
//         if let Ok(trail) = items.get(entity) {
//             actions.push(OpsMaterialUse::Use(trail.mesh, idmat, pass));
//         } else if count < 8 {
//             cmds.push(OpsCPUParticleSystemTrailMaterial(entity, idmat, pass, count + 1))
//         }
//     });
// }
