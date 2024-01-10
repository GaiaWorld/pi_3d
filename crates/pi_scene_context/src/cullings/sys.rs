
use std::ops::Range;

use pi_scene_shell::prelude::*;

use crate::{prelude::*, geometry::instance::instanced_buffer::{InstancedInfo, InstanceBufferAllocator}};

use super::base::{GeometryBounding, SceneBoundingPool, GeometryCullingMode, BoundingBoxDisplay};

pub fn sys_update_culling_by_worldmatrix(
    mut scenes: Query<&mut SceneBoundingPool>,
    items: Query<(Entity, &RenderWorldMatrix, &SceneID, &DisposeReady), Or<(Changed<RenderWorldMatrix>, Changed<DisposeReady>)>>,
    boundings: Query<(&GeometryBounding, &GeometryCullingMode)>,
    instances: Query<&InstanceMesh>,
) {
    items.iter().for_each(|(entity, worldmatrix, idscene, dispose)| {
        if let Ok(mut pool) = scenes.get_mut(idscene.0) {
            if dispose.0 == true {
                pool.remove(entity);
            } else {
                let bounding = if let Ok(instance) = instances.get(entity) {
                    if let Ok(bounding) = boundings.get(instance.0) {
                        Some(bounding)
                    } else {
                        None
                    }
                } else {
                    if let Ok(bounding) = boundings.get(entity) {
                        Some(bounding)
                    } else {
                        None
                    }
                };

                if let Some((info, mode)) = bounding {
                    pool.set(entity, info, mode, &worldmatrix.0);
                }
            }
        }
    });
}

pub fn sys_update_culling_by_cullinginfo(
    mut scenes: Query<&mut SceneBoundingPool>,
    items: Query<(&RenderWorldMatrix, &DisposeReady), Changed<RenderWorldMatrix>>,
    boundings: Query<(Entity, &SceneID, &GeometryBounding, &GeometryCullingMode, &InstanceSourceRefs), Or<(Changed<GeometryBounding>, Changed<GeometryCullingMode>)>>,
) {
    boundings.iter().for_each(|(entity, idscene, info, mode, instances)| {
        // log::error!("AAAA");
        if let Ok(mut pool) = scenes.get_mut(idscene.0) {
            if let Ok((worldmatrix, disposed)) = items.get(entity) {
                if disposed.0 == true {
                    pool.remove(entity);
                } else {
                    pool.set(entity, info, mode, &worldmatrix.0);
                }
            }
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
    geometrys: Query<&InstancedInfo>,
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
    instancedcache: Res<InstanceBufferAllocator>,
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    scenes.iter().for_each(|(boundingboxs, pool)| {
        if boundingboxs.display == false { return; }
        if let Ok((_idsource, idgeo, _meshinsstate, mut renderenable, mut instancessortinfos)) = sources.get_mut(boundingboxs.mesh) {
            let instances = pool.entities();
            if let Ok(buffer) = geometrys.get(idgeo.0) {
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