
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
        actives: Query<(&GlobalEnable, &InstanceMesh, &InstanceTransparentIndex, &AbstructMeshCullingFlag), With<AbstructMesh>>,
        instanceattributes: Query<&ModelInstanceAttributes>,
        added: ComponentAdded<DirtyInstanceSourceRefs>,
        changes: ComponentChanged<DirtyInstanceSourceRefs>,
        mut sources: Query<
            (
                Entity, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut RenderGeometryEable, &mut InstancedMeshTransparentSortCollection
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
        changes.for_each(|entity| {
            if let Ok((idsource, instances, idgeo, meshinsstate, mut renderenable, mut instancessortinfos)) = sources.get_mut(*entity) {
                if let Ok(disposed) = dispoeds.get(idsource) {
                    if disposed.0 == true { return; }
                    if meshinsstate.use_single_instancebuffer == false { return; }
                    // *renderenable = RenderGeometryEable(false);

                    if let Ok(InstancedInfoComp(Some(buffer))) = geometrys.get(idgeo.0) {
                        
                        if buffer.bytes_per_instance > 0 {
                            *renderenable = RenderGeometryEable(false);
                            instancessortinfos.0.clear();
                        }
    
                        // 实例按渲染队列排序
                        let sorted_instances = &mut temp.instancesort;
                        sorted_instances.clear();
                        instances.iter().for_each(|id| {
                            if let (Ok((enable, _, instancelayer, culling)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
                                if enable.0 == true && disposed.0 == false && culling.0 == true {
                                    sorted_instances.push(TmpInstanceSort { entity: *id, index: instancelayer.0 });
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
                            sorted_instances.iter().for_each(|instance| {
                                if tmp_alphaindex != instance.index {
                                    instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                                    tmp_alphaindex = instance.index;
                                    tmp_instance_start = tmp_instance_end;
                                }
                                tmp_instance_end += 1;
    
                                let instance = instance.entity;
    
                                if let Ok(instancedata) = instanceattributes.get(instance) {
                                    // instancedata.bytes().iter().for_each(|v| { collected.push(*v); });
                                    combinedata.record(instancedata.bytes());
                                }
    
                                idx += 0;
                            });
                            if tmp_instance_start != tmp_instance_end {
                                instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                            }
                            // reset_instances_buffer_single(idgeo.0, buffer, &collected, &mut slots, &instancedcache, &mut allocator, &device, &queue);
                            {
                                let collected = combinedata.data(&Range { start: 0, end: combinedata.usedsize() });
                                let instancedinfo = buffer;
                                if let Ok((desclist, mut buffer, mut keys, mut flag)) = slots.get_mut(idgeo.0) {
                                    match instancedinfo.slot() {
                                        EVertexBufferSlot::Slot01 => { if let Some(buffer) = &mut buffer[0] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[0] = desclist.key(0); *flag = FlagGeometryDirty; } },
                                        EVertexBufferSlot::Slot02 => { if let Some(buffer) = &mut buffer[1] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[1] = desclist.key(1); *flag = FlagGeometryDirty; } },
                                        EVertexBufferSlot::Slot03 => { if let Some(buffer) = &mut buffer[2] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[2] = desclist.key(2); *flag = FlagGeometryDirty; } },
                                        EVertexBufferSlot::Slot04 => { if let Some(buffer) = &mut buffer[3] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[3] = desclist.key(3); *flag = FlagGeometryDirty; } },
                                        EVertexBufferSlot::Slot05 => { if let Some(buffer) = &mut buffer[4] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[4] = desclist.key(4); *flag = FlagGeometryDirty; } },
                                        EVertexBufferSlot::Slot06 => { if let Some(buffer) = &mut buffer[5] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[5] = desclist.key(5); *flag = FlagGeometryDirty; } },
                                        EVertexBufferSlot::Slot07 => { if let Some(buffer) = &mut buffer[6] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[6] = desclist.key(6); *flag = FlagGeometryDirty; } },
                                        EVertexBufferSlot::Slot08 => { if let Some(buffer) = &mut buffer[7] { update_instanced_buffer_for_single(&mut buffer.0, &collected, &instancedcache, &mut allocator, &device, &queue); keys.0[7] = desclist.key(7); *flag = FlagGeometryDirty; } },
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }


    pub fn sys_tick_instanced_buffer_update(
        // added: ComponentAdded<DirtyInstanceSourceRefs>,
        // changes: ComponentChanged<DirtyInstanceSourceRefs>,
        actives: Query<(&GlobalEnable, &InstanceMesh, &InstanceTransparentIndex, &AbstructMeshCullingFlag), With<AbstructMesh>>,
        instanceattributes: Query<&ModelInstanceAttributes>,
        mut sources: Query<
            (
                Entity, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut RenderGeometryEable, &mut InstancedMeshTransparentSortCollection
            )
        >,
        dispoeds: Query<&DisposeReady>,
        geometrys: Query<&InstancedInfoComp>,
        mut slots: Query<(&AssetDescVBSlots, &mut AssetResVBSlots, &mut LoadedKeyVBSlots, &mut FlagGeometryDirty)>,
        mut instancedatacommon: ResMut<InstanceDataCommon>,
        mut temp: ResMut<TmpCommonVec>,
    ) {
        // log::error!("Instance Update");
        let mut counter = 0;
        // let mut size = 0;
        // let changes = changes.iter().chain(added.iter());
        // changes.for_each(|entity| {
        //     if let Ok((idsource, instances, idgeo, meshinsstate, mut renderenable, mut instancessortinfos)) = sources.get_mut(*entity) {
        instancedatacommon.reset();
        temp.instancesort.clear();

            sources.iter_mut().for_each(|(idsource, instances, idgeo, meshinsstate, mut renderenable, mut instancessortinfos)| {
                if let Ok(disposed) = dispoeds.get(idsource) {
                    if disposed.0 == true { return; }
                    if meshinsstate.use_single_instancebuffer == true { return; }
                    // *renderenable = RenderGeometryEable(false);
    
                    // log::error!("sys_tick_instanced_buffer_update: ");
                    // return;
    
                    if let Ok(InstancedInfoComp(Some(instancedinfo))) = geometrys.get(idgeo.0) {
                        // if instancedinfo.bytes_per_instance > 0 {
                        // }
                        *renderenable = RenderGeometryEable(false);
                        instancessortinfos.0.clear();
                        // return;
                        if let Ok((desclist, mut buffer, mut keys, mut flag)) = slots.get_mut(idgeo.0)  { 
                            // log::warn!("Instance Buffer {:?}", (instancedinfo.slot(), count));
    
                            // 实例按渲染队列排序
                            temp.instancesort.clear();
                            let sorted_instances = &mut temp.instancesort;
                            instances.iter().for_each(|id| {
                                if let (Ok((enable, _, instancelayer, culling)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
                                    if enable.0 == true && disposed.0 == false && culling.0 {
                                        sorted_instances.push(TmpInstanceSort { entity: *id, index: instancelayer.0 });
                                    }
                                }
                            });
                            sorted_instances.sort();
    
                            // return;
                            // log::error!("InstanceCount: {}", sorted_instances.len());
                            if sorted_instances.len() > 0 {
                                let vbslotdata = match instancedinfo.slot() {
                                    EVertexBufferSlot::Slot01 => { let item = buffer.get_mut(0).unwrap(); keys.0[0] = desclist.key(0); *flag = FlagGeometryDirty; item },
                                    EVertexBufferSlot::Slot02 => { let item = buffer.get_mut(1).unwrap(); keys.0[1] = desclist.key(1); *flag = FlagGeometryDirty; item },
                                    EVertexBufferSlot::Slot03 => { let item = buffer.get_mut(2).unwrap(); keys.0[2] = desclist.key(2); *flag = FlagGeometryDirty; item },
                                    EVertexBufferSlot::Slot04 => { let item = buffer.get_mut(3).unwrap(); keys.0[3] = desclist.key(3); *flag = FlagGeometryDirty; item },
                                    EVertexBufferSlot::Slot05 => { let item = buffer.get_mut(4).unwrap(); keys.0[4] = desclist.key(4); *flag = FlagGeometryDirty; item },
                                    EVertexBufferSlot::Slot06 => { let item = buffer.get_mut(5).unwrap(); keys.0[5] = desclist.key(5); *flag = FlagGeometryDirty; item },
                                    EVertexBufferSlot::Slot07 => { let item = buffer.get_mut(6).unwrap(); keys.0[6] = desclist.key(6); *flag = FlagGeometryDirty; item },
                                    EVertexBufferSlot::Slot08 => { let item = buffer.get_mut(7).unwrap(); keys.0[7] = desclist.key(7); *flag = FlagGeometryDirty; item },
                                    _ => {
                                        return;
                                    }
                                };
    
                                let start = instancedatacommon.usedsize();
                                let mut idx: u32 = 0;
                                // let mut collected: Vec<u8> = Vec::with_capacity(sorted_instances.len() * instancedinfo.bytes_per_instance as usize);
                                let mut tmp_alphaindex = sorted_instances[0].index;
                                let mut tmp_instance_start = 0;
                                let mut tmp_instance_end = 0;
                                sorted_instances.iter().for_each(|instance| {
                                    if tmp_alphaindex != instance.index {
                                        instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                                        tmp_alphaindex = instance.index;
                                        tmp_instance_start = tmp_instance_end;
                                    }
                                    tmp_instance_end += 1;
    
                                    let instance = instance.entity;
            
                                    if let Ok(instancedata) = instanceattributes.get(instance) {
                                        instancedatacommon.record(instancedata.bytes());
                                        // instancedata.bytes().iter().for_each(|v| { collected.push(*v); });
                                    }
            
                                    idx += 0;
                                });
                                if tmp_instance_start != tmp_instance_end {
                                    instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                                }
                                // size += collected.len();
                                counter += 1;
    
    
                                let end = instancedatacommon.usedsize();

                                let data = EVerteicesInstance { data: Range { start, end }, itemcount: tmp_instance_end as u32, slot: instancedinfo.slot() as u8 };
                                let data = EVerticesBufferTmp::Instance(Arc::new(data));
                                *vbslotdata = Some(AssetResVBSlot(data));
                            }
                        }
                    }
                }
            // }
            });
        // });

        // log::error!("sys_tick_instanced_buffer_update {:?}", (counter, size));
    }


    pub fn sys_instanced_buffer_upload(
        mut instancedcache: ResMut<InstanceBufferAllocator>,
        queue: Res<PiRenderQueue>,
    ) {
        instancedcache.upload(&queue);
    }

#[inline(never)]
pub fn reset_instances_buffer_single(
    idgeo: Entity,
    instancedinfo: &InstancedInfo,
    collected: &[u8],
    slots: &mut Query<(&AssetDescVBSlots, &mut AssetResVBSlots, &mut LoadedKeyVBSlots, &mut FlagGeometryDirty)>,
    instancedcache: &InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device:&RenderDevice,
    queue: &PiRenderQueue,
) {
    if let Ok((desclist, mut buffer, mut keys, mut flag)) = slots.get_mut(idgeo) {
        match instancedinfo.slot() {
            EVertexBufferSlot::Slot01 => { if let Some(buffer) = &mut buffer[0] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[0] = desclist.key(0); *flag = FlagGeometryDirty; } },
            EVertexBufferSlot::Slot02 => { if let Some(buffer) = &mut buffer[1] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[1] = desclist.key(1); *flag = FlagGeometryDirty; } },
            EVertexBufferSlot::Slot03 => { if let Some(buffer) = &mut buffer[2] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[2] = desclist.key(2); *flag = FlagGeometryDirty; } },
            EVertexBufferSlot::Slot04 => { if let Some(buffer) = &mut buffer[3] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[3] = desclist.key(3); *flag = FlagGeometryDirty; } },
            EVertexBufferSlot::Slot05 => { if let Some(buffer) = &mut buffer[4] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[4] = desclist.key(4); *flag = FlagGeometryDirty; } },
            EVertexBufferSlot::Slot06 => { if let Some(buffer) = &mut buffer[5] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[5] = desclist.key(5); *flag = FlagGeometryDirty; } },
            EVertexBufferSlot::Slot07 => { if let Some(buffer) = &mut buffer[6] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[6] = desclist.key(6); *flag = FlagGeometryDirty; } },
            EVertexBufferSlot::Slot08 => { if let Some(buffer) = &mut buffer[7] { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue); keys.0[7] = desclist.key(7); *flag = FlagGeometryDirty; } },
            _ => {}
        }
    }
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