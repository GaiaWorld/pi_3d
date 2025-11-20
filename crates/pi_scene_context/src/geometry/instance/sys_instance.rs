
use core::f32;
use std::ops::Range;

use pi_scene_shell::prelude::*;

use crate::{
    geometry::vertex_buffer_useinfo::*,
    prelude::*,
};

use super::{*, instanced_buffer::*, types::ModelInstanceAttributes, };

pub fn sys_tick_instanced_buffer_update_single(
    actives: Query<(&GlobalEnable, &RenderQueueSortParam, &AbstructMeshCullingFlag, &GlobalMatrix, &LocalPosition), With<InstanceMesh>>,
    instanceattributes: Query<&ModelInstanceAttributes>,
    changeds: ComponentChanged<InstanceSourceRefs>,
    mut sources: Query<
        (
            Entity, &EInstanceSortMode, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut InstancedMeshTransparentSortCollection
        )
    >,
    dispoeds: Query<&DisposeReady>,
    geometrys: Query<&InstancedInfoComp>,
    mut slots: Query<(&AssetDescVBSlots, &mut AssetResVBSlots, &mut LoadedKeyVBSlots, &mut FlagGeometryDirty)>,
    instancedcache: Res<InstanceBufferAllocator>,
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
    mut temp: ResMut<TmpCommonVec>,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_tick_instanced_buffer_update_single"));
    let mut entities = entitysets.pop();
    changeds.iter().for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok((idsource, sortmode, instances, idgeo, meshinsstate, mut instancessortinfos)) = sources.get_mut(*entity) {
            if let Ok(disposed) = dispoeds.get(idsource) {
                if disposed.0 == true { return; }
                if meshinsstate.use_single_instancebuffer == false { return; }
                if let Ok(InstancedInfoComp(Some(buffer))) = geometrys.get(idgeo.0) {
                    instancessortinfos.reset();
                    instancessortinfos.sizeperinstance = buffer.bytes_per_instance as u16;
                    instancessortinfos.use_single_instancebuffer = meshinsstate.use_single_instancebuffer;

                    // 实例按渲染队列排序
                    temp.clear();
                    
                    if collect_instance_info(sortmode, instances, &mut instancessortinfos, &actives, &dispoeds, &instanceattributes, &mut temp, u32::MAX) {
                        let collected = instancessortinfos.data.as_slice();
                        let instancedinfo = buffer;
                        if let Ok((desclist, mut buffer, mut keys, mut flag)) = slots.get_mut(idgeo.0) {
                            let buffer = match instancedinfo.slot() {
                                EVertexBufferSlot::Slot01 => { if let Some(buffer) = &mut buffer[0] { keys.0[0] = desclist.key(0); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                EVertexBufferSlot::Slot02 => { if let Some(buffer) = &mut buffer[1] { keys.0[1] = desclist.key(1); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                EVertexBufferSlot::Slot03 => { if let Some(buffer) = &mut buffer[2] { keys.0[2] = desclist.key(2); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                EVertexBufferSlot::Slot04 => { if let Some(buffer) = &mut buffer[3] { keys.0[3] = desclist.key(3); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                EVertexBufferSlot::Slot05 => { if let Some(buffer) = &mut buffer[4] { keys.0[4] = desclist.key(4); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                EVertexBufferSlot::Slot06 => { if let Some(buffer) = &mut buffer[5] { keys.0[5] = desclist.key(5); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                EVertexBufferSlot::Slot07 => { if let Some(buffer) = &mut buffer[6] { keys.0[6] = desclist.key(6); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                EVertexBufferSlot::Slot08 => { if let Some(buffer) = &mut buffer[7] { keys.0[7] = desclist.key(7); *flag = FlagGeometryDirty;  &mut buffer.0 } else { return; } },
                                _ => { return; }
                            };
                            update_instanced_buffer_for_single(buffer, &collected, &instancedcache, &mut allocator, &device, &queue);
                        }
                    }
                }
            }
        }
    });
    entitysets.push(entities);
}


pub fn sys_tick_instanced_buffer_update(
    changeds: ComponentChanged<InstanceSourceRefs>,
    actives: Query<(&GlobalEnable, &RenderQueueSortParam, &AbstructMeshCullingFlag, &GlobalMatrix, &LocalPosition), With<InstanceMesh>>,
    instanceattributes: Query<&ModelInstanceAttributes>,
    mut sources: Query<
        (
            Entity, &EInstanceSortMode, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut InstancedMeshTransparentSortCollection
        ),
    >,
    dispoeds: Query<&DisposeReady>,
    geometrys: Query<&InstancedInfoComp>,
    mut temp: ResMut<TmpCommonVec>,
    engineopt: Res<EngineCustomPlugins>,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut performance: ResMut<Performance>,
) {
    let mut entities = entitysets.pop();
    // performance.systems.push(String::from("sys_tick_instanced_buffer_update"));
    // log::error!("Instance Update");
    let mut counter = 0;
    changeds.iter().for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok((idsource, sortmode, instances, idgeo, meshinsstate, mut instancessortinfos)) = sources.get_mut(*entity) {
            if let Ok(disposed) = dispoeds.get(idsource) {
                if disposed.0 == true { return; }
                if meshinsstate.use_single_instancebuffer == true { return; }

                if let Ok(InstancedInfoComp(Some(instancedinfo))) = geometrys.get(idgeo.0) {
                    // *renderenable = RenderGeometryEable(false);

                    // 实例按渲染队列排序
                    temp.clear();
                    instancessortinfos.reset();
                    instancessortinfos.use_single_instancebuffer = meshinsstate.use_single_instancebuffer;
                    instancessortinfos.sizeperinstance = instancedinfo.bytes_per_instance as u16;

                    if collect_instance_info(sortmode, instances, &mut instancessortinfos, &actives, &dispoeds, &instanceattributes, &mut temp, engineopt.max_instance_batch_count) {
                        counter += 1;
                    }
                    // log::error!("{:?}", (instancessortinfos.count, instancessortinfos.data.len()));
                }
            }
        }
    });
    entitysets.push(entities);

    // log::error!("sys_tick_instanced_buffer_update {:?}", (counter));
}


pub fn sys_instanced_buffer_upload(
    mut instancedcache: ResMut<InstanceBufferAllocator>,
    queue: Res<PiRenderQueue>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_instanced_buffer_upload"));
    instancedcache.upload(&queue);
}

pub fn update_instanced_buffer_for_single(
    oldbuffer: &mut EVerticesBufferTmp,
    collected: &[u8],
    instancedcache: &InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device:&RenderDevice,
    queue: &PiRenderQueue,
) {
    match oldbuffer {
        EVerticesBufferTmp::Instance(_) => {
            
        },
        EVerticesBufferTmp::Buffer(oldbuffer) => match oldbuffer {
            EVerticesBufferUsage::EVBRange(buffer) => {
                let newbuffer = if instancedcache.check(buffer.buffer()) == false {
                    buffer.buffer().size() < collected.len() as u64
                } else {
                    true
                };
                if newbuffer {
                    if let Some(newbuffer) = allocator.create_not_updatable_buffer(device, queue, collected, None) {
                        *buffer = Share::new(newbuffer);
                    }
                } else {
                    queue.write_buffer(buffer.buffer(), 0, collected);
                    *buffer = buffer.clone();
                }
            },
            _ => { },
        },
    }
}

fn collect_instance_info(
    sortmode: &EInstanceSortMode,
    instances: &InstanceSourceRefs,
    instancessortinfos: &mut InstancedMeshTransparentSortCollection,
    actives: &Query<(&GlobalEnable, &RenderQueueSortParam, &AbstructMeshCullingFlag, &GlobalMatrix, &LocalPosition), With<InstanceMesh>>,
    dispoeds: &Query<&DisposeReady>,
    instanceattributes: &Query<&ModelInstanceAttributes>,
    temp: &mut TmpCommonVec,
    max_instance_batch_count: u32,
) -> bool {
    
    let (isglobal, vidx, scl) = sortmode.arg_for_sortparam();

    instances.iter().for_each(|(_k, id)| {
        if let (Ok((enable, instancelayer, culling, gtransform, localpos)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
            if enable.0 == true && disposed.0 == false && culling.0 {
                let sortparam = if isglobal  {
                    gtransform.position().as_slice()[vidx] * scl
                } else {
                    localpos.0.as_slice()[vidx] * scl
                };
                temp.push(*id, instancelayer.index, sortparam, gtransform.xyz());
            }
        }
    });

    temp.sort();
    if  temp.is_empty() == false {
        let mut minx = f32::MAX;
        let mut miny = f32::MAX;
        let mut minz = f32::MAX;
        let mut maxx = f32::MIN;
        let mut maxy = f32::MIN;
        let mut maxz = f32::MIN;

        let mut tmp_alphaindex = i32::MIN;
        let mut tmp_instance_start = 0;
        let mut tmp_instance_end = 0;

        temp.iter(|(idinstance, index, xyz)| {
            if let Ok(instancedata) = instanceattributes.get(*idinstance) {
                if tmp_alphaindex != *index || tmp_instance_end - tmp_instance_start > max_instance_batch_count {
                    instancessortinfos.ranges.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }, ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)));
                    tmp_alphaindex = *index;
                    tmp_instance_start = tmp_instance_end;
                    minx = f32::MAX;
                    miny = f32::MAX;
                    minz = f32::MAX;
                    maxx = f32::MIN;
                    maxy = f32::MIN;
                    maxz = f32::MIN;
                }
                minx = minx.min(xyz.0);
                miny = miny.min(xyz.1);
                minz = minz.min(xyz.2);
                maxx = maxx.max(xyz.0);
                maxy = maxy.max(xyz.1);
                maxz = maxz.max(xyz.2);
                tmp_instance_end += 1;

                unsafe_vec_append_slice(&mut instancessortinfos.data, instancedata.bytes());
            }
        });
        if tmp_instance_start != tmp_instance_end {
            instancessortinfos.ranges.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }, ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)));
        }
        
        instancessortinfos.count = tmp_instance_end as u32;
        return true;
    } else {
        return false;
    }
}