
use pi_scene_shell::prelude::*;

use crate::{geometry::instance::{instanced_buffer::{InstanceBufferAllocator, InstancedInfoComp}, types::ModelInstanceAttributes}, prelude::*};

use super::base::{GeometryBounding, SceneBoundingPool, GeometryCullingMode, BoundingBoxDisplay};

pub fn sys_update_collider_by_matrix(
    changes2: ComponentChanged<GlobalMatrix>,
    changes1: ComponentChanged<RenderWorldMatrix>,
    mut items: Query<&mut Collider>,
) {
    changes1.iter().for_each(|entity| {
        if let Ok(mut collider) = items.get_mut(*entity) {
            collider.set_changed();
            // *collider = collider.clone();
        }
    });
    changes2.iter().for_each(|entity| {
        if let Ok(mut collider) = items.get_mut(*entity) {
            collider.set_changed();
            // *collider = collider.clone();
        }
    });
}

pub fn sys_update_collider(
    mut scenes: Query<&mut SceneColliderPool>,
    addeds: ComponentAdded<Collider>,
    changes: ComponentChanged<Collider>,
    changes0: ComponentChanged<DisposeReady>,
    gmatrix: Query<&GlobalMatrix>,
    rmatrix: Query<&RenderWorldMatrix>,
    items: Query<(&Collider, &SceneID, &DisposeReady)>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {
    let mut entities = entitysets.pop();
    // changes.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    // addeds.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    let mut temp = Vector3::zeros();
    // log::error!("sys_update_collider: {:?}", entities.len());
    changes.iter().chain(addeds.iter()).for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok((collider, idscene, dispose)) = items.get(*entity) {
            if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                if dispose.0 == true {
                    pool.remove(*entity);
                } else {
                    if let Ok(worldmatrix) = rmatrix.get(*entity) {
                        pool.set(*entity, collider, &worldmatrix.0, &mut temp);
                    } else if let Ok(worldmatrix) = gmatrix.get(*entity) {
                        pool.set(*entity, collider, worldmatrix.matrix(), &mut temp);
                    }
                }
            } else {
                // log::error!("sys_update_collider Scene Not Found SceneColliderPool : ");
            }
        } else {
            // log::error!("sys_update_collider Item Component Not Found : ");
        }
    });
    changes0.iter().for_each(|entity| {
        if let Ok((_collider, idscene, dispose)) = items.get(*entity) {
            if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                if dispose.0 == true {
                    pool.remove(*entity);
                }
            }
        }
    });
    entitysets.push(entities);
}

pub fn sys_update_culling_by_worldmatrix(
    rmchanges: ComponentChanged<RenderWorldMatrix>,
    changes0: ComponentChanged<DisposeReady>,
    mut items: Query<&mut ItemCullingDirty>,
) {
    rmchanges.iter().for_each(|entity| {
        if let Ok(mut flag) = items.get_mut(*entity) {
            *flag = ItemCullingDirty;
        }
    });

    changes0.iter().for_each(|entity| {
        if let Ok(mut flag) = items.get_mut(*entity) {
            *flag = ItemCullingDirty;
        }
    });
}

pub fn sys_update_culling_by_cullinginfo(
    mut scenes: Query<&mut SceneBoundingPool>,
    addeds: ComponentAdded<ItemCullingDirty>,
    changes: ComponentChanged<ItemCullingDirty>,
    items: Query<(&RenderWorldMatrix, &DisposeReady)>,
    boundings: Query<(&SceneID, &GeometryBounding, &GeometryCullingMode, &InstanceSourceRefs, &ModelInstanceAttributes)>,
    instances: Query<&InstanceMesh>,
    entitysets: Res<EntityFilterForComponentChanged>,
    mut viewers: Query<(&SceneID, &mut ViewerCullingDirty)>,
) {
    // log::error!("sys_update_culling_by_cullinginfo");
    let mut entities = entitysets.pop();
    let mut dirtyscenes = entitysets.pop();
    // addeds.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    // changes.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    changes.iter().chain(addeds.iter()).for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok(instance) = instances.get(*entity) {
            if let Ok((idscene, info, mode, _instances, _)) = boundings.get(instance.0) {
                if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                    if let Ok((worldmatrix, disposed)) = items.get(*entity) {
                        dirtyscenes.insert(&idscene.0);
                        if disposed.0 == true {
                            pool.remove(*entity);
                        } else {
                            pool.set(*entity, info, mode, &worldmatrix.0);
                        }
                    }
                }
            }
        } else if let Ok((idscene, info, mode, instancerefs, insattr)) = boundings.get(*entity) {
            if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                if let Ok((meshworldmatrix, disposed)) = items.get(*entity) {
                    dirtyscenes.insert(&idscene.0);
                    if disposed.0 == true {
                        pool.remove(*entity);
                    } else {
                        if insattr.bytes().len() > 0 {
                            pool.set(*entity, info, &GeometryCullingMode(ECullingStrategy::None), &meshworldmatrix.0);
                        } else {
                            pool.set(*entity, info, mode, &meshworldmatrix.0);
                        }

                        instancerefs.iter().for_each(|(_k, instance)| {
                            if !entities.insert(instance) { return; }
                            let instance = *instance;
                            // if let Some(instance) = &instance { *instance } else { return; }
                            if let Ok((worldmatrix, disposed)) = items.get(instance) {
                                if disposed.0 == true {
                                    pool.remove(instance);
                                } else {
                                    pool.set(instance, info, mode, &worldmatrix.0);
                                }
                            }
                        });
                    }
                }
            }
        }
    });
    viewers.iter_mut().for_each(|(idscene, mut flag)| {
        if dirtyscenes.contains(&idscene.0) {
            flag.set_changed();
        }
    });
    entitysets.push(entities);
}

pub fn sys_tick_culling_box(
    scenes: Query<(&BoundingBoxDisplay, &SceneBoundingPool), Or<(Changed<SceneBoundingPool>, Changed<BoundingBoxDisplay>)>>,
    actives: Query<(&GlobalEnable, &GeometryBounding, &RenderWorldMatrix, &AbstructMeshCullingFlag)>,
    mut sources: Query<
        (
            Entity, &GeometryID, &MeshInstanceState, &mut InstancedMeshTransparentSortCollection
        )
    >,
    dispoeds: Query<&DisposeReady>,
    geometrys: Query<&InstancedInfoComp>,
    mut slots: Query<(&AssetDescVBSlots, &mut AssetResVBSlots, &mut LoadedKeyVBSlots, &mut FlagGeometryDirty)>,
    instancedcache: Res<InstanceBufferAllocator>,
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    scenes.iter().for_each(|(boundingboxs, pool)| {
        if boundingboxs.display == false { return; }
        if let Ok((_idsource, idgeo, _meshinsstate, mut instancessortinfos)) = sources.get_mut(boundingboxs.mesh) {
            // let instances = pool.entities();
            // if let Ok(InstancedInfoComp(Some(buffer))) = geometrys.get(idgeo.0) {
            //     // if buffer.bytes_per_instance > 0 {
            //     //     *renderenable = RenderGeometryEable(false);
            //     //     instancessortinfos.reset();
            //     // }
            //     instancessortinfos.reset();
            //     // log::error!("Bounding A: {:?}", instances.len());

            //     if instances.len() > 0 {
            //         let mut collected: Vec<u8> = vec![];
            //         let tmp_alphaindex = 0;
            //         let tmp_instance_start = 0;
            //         let mut tmp_instance_end = 0;
            //         instances.iter().for_each(|id| {
            //             if let (Ok((enable, bounding, worldmatrix, culling)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
            //                 if enable.0 == true && disposed.0 == false && culling.0 == true {
            //                     bytemuck::cast_slice(worldmatrix.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
            //                     bytemuck::cast_slice(bounding.minimum.as_slice()).iter().for_each(|v| { collected.push(*v); });
            //                     bytemuck::cast_slice(bounding.maximum.as_slice()).iter().for_each(|v| { collected.push(*v); });

            //                     tmp_instance_end += 1;
            //                 }
            //             }
            //         });
            //         // log::error!("Bounding: {:?}", tmp_instance_end);
            //         instancessortinfos.ranges.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
            //         // reset_instances_buffer_single(idgeo.0, buffer, &collected, &mut slots, &instancedcache, &mut allocator, &device, &queue);
            //         {
            //             let instancedinfo = buffer;
            //             if let Ok((desclist, mut buffer, mut keys, mut flag)) = slots.get_mut(idgeo.0) {
            //                 match instancedinfo.slot() {
            //                     EVertexBufferSlot::Slot01 => { if let Some(buffer) = &mut buffer[0] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[0] = desclist.key(0); *flag = FlagGeometryDirty; } },
            //                     EVertexBufferSlot::Slot02 => { if let Some(buffer) = &mut buffer[1] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[1] = desclist.key(1); *flag = FlagGeometryDirty; } },
            //                     EVertexBufferSlot::Slot03 => { if let Some(buffer) = &mut buffer[2] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[2] = desclist.key(2); *flag = FlagGeometryDirty; } },
            //                     EVertexBufferSlot::Slot04 => { if let Some(buffer) = &mut buffer[3] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[3] = desclist.key(3); *flag = FlagGeometryDirty; } },
            //                     EVertexBufferSlot::Slot05 => { if let Some(buffer) = &mut buffer[4] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[4] = desclist.key(4); *flag = FlagGeometryDirty; } },
            //                     EVertexBufferSlot::Slot06 => { if let Some(buffer) = &mut buffer[5] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[5] = desclist.key(5); *flag = FlagGeometryDirty; } },
            //                     EVertexBufferSlot::Slot07 => { if let Some(buffer) = &mut buffer[6] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[6] = desclist.key(6); *flag = FlagGeometryDirty; } },
            //                     EVertexBufferSlot::Slot08 => { if let Some(buffer) = &mut buffer[7] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[7] = desclist.key(7); *flag = FlagGeometryDirty; } },
            //                     _ => {}
            //                 }
            //             }
            //         }
            //     }
            // }
        }
    });
}