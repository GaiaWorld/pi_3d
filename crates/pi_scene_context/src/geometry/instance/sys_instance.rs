
use core::f32;
use std::{sync::Arc, ops::Range};

use pi_scene_shell::prelude::*;

use crate::{
    geometry::vertex_buffer_useinfo::*,
    prelude::*,
};

use super::{*, instanced_buffer::*, types::ModelInstanceAttributes, };

#[derive(Clone, Copy)]
pub struct TmpInstanceSort {
    pub entity: Entity,
    pub index: i32,
    pub xyz: (f32, f32, f32),
}
impl PartialEq for TmpInstanceSort {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for TmpInstanceSort {
    fn assert_receiver_is_total_eq(&self) {

    }
}
impl PartialOrd for TmpInstanceSort {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}
impl Ord for TmpInstanceSort {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

    pub fn sys_tick_instanced_buffer_update_single(
        actives: Query<(&GlobalEnable, &InstanceMesh, &InstanceTransparentIndex, &AbstructMeshCullingFlag, &GlobalMatrix), With<AbstructMesh>>,
        instanceattributes: Query<&ModelInstanceAttributes>,
        added: ComponentAdded<InstanceSourceRefs>,
        changes: ComponentChanged<InstanceSourceRefs>,
        mut sources: Query<
            (
                Entity, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut InstancedMeshTransparentSortCollection
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
        mut combinedata: ResMut<CombineDataCommon>,
    ) {
        let changes = changes.iter().chain(added.iter());
        let mut minx = f32::MAX;
        let mut miny = f32::MAX;
        let mut minz = f32::MAX;
        let mut maxx = f32::MIN;
        let mut maxy = f32::MIN;
        let mut maxz = f32::MIN;
        changes.for_each(|entity| {
            if let Ok((idsource, instances, idgeo, meshinsstate, mut instancessortinfos)) = sources.get_mut(*entity) {
                if let Ok(disposed) = dispoeds.get(idsource) {
                    if disposed.0 == true { return; }
                    if meshinsstate.use_single_instancebuffer == false { return; }
                    if let Ok(InstancedInfoComp(Some(buffer))) = geometrys.get(idgeo.0) {
                        instancessortinfos.reset();
                        instancessortinfos.sizeperinstance = buffer.bytes_per_instance as u16;
                        instancessortinfos.use_single_instancebuffer = meshinsstate.use_single_instancebuffer;
    
                        // 实例按渲染队列排序
                        let sorted_instances = &mut temp.instancesort;
                        sorted_instances.clear();
                        instances.iter().for_each(|id| {
                            if let (Ok((enable, _, instancelayer, culling, gtransform)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
                                if enable.0 == true && disposed.0 == false && culling.0 == true {
                                    sorted_instances.push(TmpInstanceSort { entity: *id, index: instancelayer.0, xyz: gtransform.xyz() });
                                }
                            }
                        });
                        sorted_instances.sort();
                        // log::warn!("InstanceCount: {}", sorted_instances.len());
    
                        if sorted_instances.len() > 0 {
                            let mut idx: u32 = 0;
                            combinedata.reset();

                            let mut tmp_alphaindex = sorted_instances[0].index;
                            let mut tmp_instance_start = 0;
                            let mut tmp_instance_end = 0;
                            minx = minx.min(sorted_instances[0].xyz.0);
                            miny = miny.min(sorted_instances[0].xyz.1);
                            minz = minz.min(sorted_instances[0].xyz.2);
                            maxx = maxx.max(sorted_instances[0].xyz.0);
                            maxy = maxy.max(sorted_instances[0].xyz.1);
                            maxz = maxz.max(sorted_instances[0].xyz.2);

                            sorted_instances.iter().for_each(|instance| {
                                let idinstance = instance.entity;
                                if let Ok(instancedata) = instanceattributes.get(idinstance) {
                                    if tmp_alphaindex != instance.index {
                                        instancessortinfos.ranges.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }, ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)));
                                        tmp_alphaindex = instance.index;
                                        tmp_instance_start = tmp_instance_end;
                                        minx = f32::MAX;
                                        miny = f32::MAX;
                                        minz = f32::MAX;
                                        maxx = f32::MIN;
                                        maxy = f32::MIN;
                                        maxz = f32::MIN;
                                    }
                                    minx = minx.min(instance.xyz.0);
                                    miny = miny.min(instance.xyz.1);
                                    minz = minz.min(instance.xyz.2);
                                    maxx = maxx.max(instance.xyz.0);
                                    maxy = maxy.max(instance.xyz.1);
                                    maxz = maxz.max(instance.xyz.2);
                                    tmp_instance_end += 1;
    
                                    instancedata.bytes().iter().for_each(|v| { instancessortinfos.data.push(*v); });
                                }
    
                                idx += 0;
                            });
                            if tmp_instance_start != tmp_instance_end {
                                instancessortinfos.ranges.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }, ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)));
                            }
                            {
                                let collected = combinedata.data(&Range { start: 0, end: combinedata.usedsize() });
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
            }
        });
    }


    pub fn sys_tick_instanced_buffer_update(
        added: ComponentAdded<InstanceSourceRefs>,
        changes: ComponentChanged<InstanceSourceRefs>,
        actives: Query<(&GlobalEnable, &InstanceMesh, &InstanceTransparentIndex, &AbstructMeshCullingFlag, &GlobalMatrix), With<AbstructMesh>>,
        instanceattributes: Query<&ModelInstanceAttributes>,
        mut sources: Query<
            (
                Entity, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut InstancedMeshTransparentSortCollection
            )
        >,
        dispoeds: Query<&DisposeReady>,
        geometrys: Query<&InstancedInfoComp>,
        mut temp: ResMut<TmpCommonVec>,
    ) {
        // log::error!("Instance Update");
        let mut counter = 0;
        // let mut size = 0;
        let mut minx = f32::MAX;
        let mut miny = f32::MAX;
        let mut minz = f32::MAX;
        let mut maxx = f32::MIN;
        let mut maxy = f32::MIN;
        let mut maxz = f32::MIN;
        let changes = changes.iter().chain(added.iter());
        changes.for_each(|entity| {
            if let Ok((idsource, instances, idgeo, meshinsstate, mut instancessortinfos)) = sources.get_mut(*entity) {
                if let Ok(disposed) = dispoeds.get(idsource) {
                    if disposed.0 == true { return; }
                    if meshinsstate.use_single_instancebuffer == true { return; }
    
                    if let Ok(InstancedInfoComp(Some(instancedinfo))) = geometrys.get(idgeo.0) {
                        // *renderenable = RenderGeometryEable(false);
    
                        // 实例按渲染队列排序
                        temp.instancesort.clear();
                        instancessortinfos.reset();
                        instancessortinfos.use_single_instancebuffer = meshinsstate.use_single_instancebuffer;
                        instancessortinfos.sizeperinstance = instancedinfo.bytes_per_instance as u16;
                        let sorted_instances = &mut temp.instancesort;
                        instances.iter().for_each(|id| {
                            if let (Ok((enable, _, instancelayer, culling, gtransform)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
                                if enable.0 == true && disposed.0 == false && culling.0 {
                                    sorted_instances.push(TmpInstanceSort { entity: *id, index: instancelayer.0, xyz: gtransform.xyz() });
                                }
                            }
                        });
                        sorted_instances.sort();

                        // log::error!("InstanceCount: {:?}", (instances.len(), sorted_instances.len()));
                        if sorted_instances.len() > 0 {
                            let mut idx: u32 = 0;
                            // let mut collected: Vec<u8> = Vec::with_capacity(sorted_instances.len() * instancedinfo.bytes_per_instance as usize);
                            let mut tmp_alphaindex = sorted_instances[0].index;
                            let mut tmp_instance_start = 0;
                            let mut tmp_instance_end = 0;
                            minx = minx.min(sorted_instances[0].xyz.0);
                            miny = miny.min(sorted_instances[0].xyz.1);
                            minz = minz.min(sorted_instances[0].xyz.2);
                            maxx = maxx.max(sorted_instances[0].xyz.0);
                            maxy = maxy.max(sorted_instances[0].xyz.1);
                            maxz = maxz.max(sorted_instances[0].xyz.2);
                            sorted_instances.iter().for_each(|instance| {
                                let idinstance = instance.entity;
                                if let Ok(instancedata) = instanceattributes.get(idinstance) {
                                    if tmp_alphaindex != instance.index {
                                        instancessortinfos.ranges.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }, ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)));
                                        tmp_alphaindex = instance.index;
                                        tmp_instance_start = tmp_instance_end;
                                        minx = f32::MAX;
                                        miny = f32::MAX;
                                        minz = f32::MAX;
                                        maxx = f32::MIN;
                                        maxy = f32::MIN;
                                        maxz = f32::MIN;
                                    }
                                    minx = minx.min(instance.xyz.0);
                                    miny = miny.min(instance.xyz.1);
                                    minz = minz.min(instance.xyz.2);
                                    maxx = maxx.max(instance.xyz.0);
                                    maxy = maxy.max(instance.xyz.1);
                                    maxz = maxz.max(instance.xyz.2);
                                    tmp_instance_end += 1;

                                    unsafe_vec_append_slice(&mut instancessortinfos.data, instancedata.bytes());
                                    // instancedata.bytes().iter().for_each(|v| { instancessortinfos.data.push(*v); });
                                }
                                idx += 0;
                            });
                            if tmp_instance_start != tmp_instance_end {
                                instancessortinfos.ranges.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }, ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)));
                            }
                            counter += 1;

                            instancessortinfos.count = tmp_instance_end as u32;
                        }
                    }
                }
            }
        });

        // log::error!("sys_tick_instanced_buffer_update {:?}", (counter, size));
    }


    pub fn sys_instanced_buffer_upload(
        mut instancedcache: ResMut<InstanceBufferAllocator>,
        queue: Res<PiRenderQueue>,
    ) {
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
                        *buffer = Arc::new(newbuffer);
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