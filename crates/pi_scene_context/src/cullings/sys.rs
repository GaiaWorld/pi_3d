
use std::ops::Range;

use pi_scene_shell::prelude::*;

use crate::{geometry::instance::instanced_buffer::{InstanceBufferAllocator, InstancedInfoComp}, prelude::*};

use super::base::{GeometryBounding, SceneBoundingPool, GeometryCullingMode, BoundingBoxDisplay};

pub fn sys_update_collider_by_matrix(
    addeds: ComponentAdded<GlobalMatrix>,
    changes2: ComponentChanged<GlobalMatrix>,
    mut items: Query<&mut Collider>,
) {
    addeds.iter().chain(changes2.iter()).for_each(|entity| {
        if let Ok(mut collider) = items.get_mut(*entity) {
            *collider = collider.clone();
        }
    });
}

pub fn sys_update_collider(
    mut scenes: Query<&mut SceneColliderPool>,
    addeds: ComponentAdded<Collider>,
    changes: ComponentChanged<Collider>,
    changes0: ComponentChanged<DisposeReady>,
    items: Query<(Entity, &Collider, &GlobalMatrix, &SceneID, &DisposeReady)>,
) {
    addeds.iter().chain(changes.iter()).for_each(|entity| {
        if let Ok((entity, collider, worldmatrix, idscene, dispose)) = items.get(*entity) {
            if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                if dispose.0 == true {
                    pool.remove(entity);
                } else {
                    pool.set(entity, collider, worldmatrix.matrix());
                }
            }
        }
    });
    changes0.iter().for_each(|entity| {
        if let Ok((entity, collider, worldmatrix, idscene, dispose)) = items.get(*entity) {
            if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                if dispose.0 == true {
                    pool.remove(entity);
                }
            }
        }
    });
}

pub fn sys_update_culling_by_worldmatrix(
    changes: ComponentChanged<RenderWorldMatrix>,
    changes0: ComponentChanged<DisposeReady>,
    mut items: Query<&mut ItemCullingDirty>,
) {
    changes.iter().for_each(|entity| {
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
    boundings: Query<(&SceneID, &GeometryBounding, &InstanceSourceRefs)>,
    modes: Query<&GeometryCullingMode>,
    instances: Query<&InstanceMesh>,
) {
    addeds.iter().chain(changes.iter()).for_each(|entity| {
        if let Ok(mesh) = instances.get(*entity) {
            if let Ok((idscene, info, instances)) = boundings.get(mesh.0) {
                if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                    if let Ok(mode) = modes.get(*entity) {
                        if let Ok((worldmatrix, disposed)) = items.get(*entity) {
                            if disposed.0 == true {
                                pool.remove(*entity);
                            } else {
                                pool.set(*entity, info, mode, &worldmatrix.0);
                            }
                        }
                    }
                }
            }
        } else if let Ok((idscene, info, instances)) = boundings.get(*entity) {
            if let Ok(mut pool) = scenes.get_mut(idscene.0) {
                if let Ok(mode) = modes.get(*entity) {
                    if let Ok((worldmatrix, disposed)) = items.get(*entity) {
                        if disposed.0 == true {
                            pool.remove(*entity);
                        } else {
                            pool.set(*entity, info, mode, &worldmatrix.0);

                            instances.iter().for_each(|entity| {
                                let entity = *entity;
                                if let Ok((worldmatrix, disposed)) = items.get(entity) {
                                    if disposed.0 == true {
                                        pool.remove(entity);
                                    } else {
                                        pool.set(entity, info, mode, &worldmatrix.0);
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }
    });
}

pub fn sys_tick_culling_box(
    scenes: Query<(&BoundingBoxDisplay, &SceneBoundingPool), Or<(Changed<SceneBoundingPool>, Changed<BoundingBoxDisplay>)>>,
    actives: Query<(&GlobalEnable, &GeometryBounding, &RenderWorldMatrix, &AbstructMeshCullingFlag)>,
    mut sources: Query<
        (
            Entity, &GeometryID, &MeshInstanceState, &mut RenderGeometryEable, &mut InstancedMeshTransparentSortCollection
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
        if let Ok((_idsource, idgeo, _meshinsstate, mut renderenable, mut instancessortinfos)) = sources.get_mut(boundingboxs.mesh) {
            let instances = pool.entities();
            if let Ok(InstancedInfoComp(Some(buffer))) = geometrys.get(idgeo.0) {
                if buffer.bytes_per_instance > 0 {
                    *renderenable = RenderGeometryEable(false);
                    instancessortinfos.0.clear();
                }
                // log::error!("Bounding A: {:?}", instances.len());

                if instances.len() > 0 {
                    let mut collected: Vec<u8> = vec![];
                    let tmp_alphaindex = 0;
                    let tmp_instance_start = 0;
                    let mut tmp_instance_end = 0;
                    instances.iter().for_each(|id| {
                        if let (Ok((enable, bounding, worldmatrix, culling)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
                            if enable.0 == true && disposed.0 == false && culling.0 == true {
                                bytemuck::cast_slice(worldmatrix.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                bytemuck::cast_slice(bounding.minimum.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                bytemuck::cast_slice(bounding.maximum.as_slice()).iter().for_each(|v| { collected.push(*v); });

                                tmp_instance_end += 1;
                            }
                        }
                    });
                    // log::error!("Bounding: {:?}", tmp_instance_end);
                    instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                    reset_instances_buffer_single(idgeo.0, buffer, &collected, &mut slots, &instancedcache, &mut allocator, &device, &queue);
                }
            }
        }
    });
}