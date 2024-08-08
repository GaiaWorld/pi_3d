
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
                        let mut sorted_instances = vec![];
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
                            let mut collected: Vec<u8> = vec![];
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
                                    instancedata.bytes().iter().for_each(|v| { collected.push(*v); });
                                }
    
                                idx += 0;
                            });
                            if tmp_instance_start != tmp_instance_end {
                                instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                            }
                            reset_instances_buffer_single(idgeo.0, buffer, &collected, &mut slots, &instancedcache, &mut allocator, &device, &queue);
                        }
                    }
                }
            }
        });
    }


    pub fn sys_tick_instanced_buffer_update(
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
        mut instancedcache: ResMut<InstanceBufferAllocator>,
        mut allocator: ResMut<VertexBufferAllocator3D>,
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
    ) {
        // log::error!("Instance Update");
        sources.iter_mut().for_each(|(idsource, instances, idgeo, meshinsstate, mut renderenable, mut instancessortinfos)| {
            if let Ok(disposed) = dispoeds.get(idsource) {
                if disposed.0 == true { return; }
                if meshinsstate.use_single_instancebuffer == true { return; }
                // *renderenable = RenderGeometryEable(false);
                
                // log::error!("sys_tick_instanced_buffer_update: ");

                if let Ok(InstancedInfoComp(Some(buffer))) = geometrys.get(idgeo.0) {
                    if buffer.bytes_per_instance > 0 {
                        *renderenable = RenderGeometryEable(false);
                        instancessortinfos.0.clear();
                    }

                    // 实例按渲染队列排序
                    let mut sorted_instances = vec![];
                    instances.iter().for_each(|id| {
                        if let (Ok((enable, _, instancelayer, culling)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
                            if enable.0 == true && disposed.0 == false && culling.0 == true {
                                sorted_instances.push(TmpInstanceSort { entity: *id, index: instancelayer.0 });
                            }
                        }
                    });
                    sorted_instances.sort();

                    // log::error!("InstanceCount: {}", sorted_instances.len());
                    if sorted_instances.len() > 0 {
                        let mut idx: u32 = 0;
                        let mut collected: Vec<u8> = vec![];
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
                                instancedata.bytes().iter().for_each(|v| { collected.push(*v); });
                            }
    
                            idx += 0;
                        });
                        if tmp_instance_start != tmp_instance_end {
                            instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                        }
                        reset_instances_buffer(idgeo.0, buffer, collected, tmp_instance_end, &mut slots, &mut instancedcache, &mut allocator, &device, &queue);
                    }
                }
            }
        });
    }


    pub fn sys_instanced_buffer_upload(
        mut instancedcache: ResMut<InstanceBufferAllocator>,
        queue: Res<PiRenderQueue>,
    ) {
        instancedcache.upload(&queue);
    }

#[inline(never)]
pub fn reset_instances_buffer(
    idgeo: Entity,
    instancedinfo: &InstancedInfo,
    collected: Vec<u8>,
    count: u32,
    slots: &mut Query<(&AssetDescVBSlots, &mut AssetResVBSlots, &mut LoadedKeyVBSlots, &mut FlagGeometryDirty)>,
    _instancedcache: &mut InstanceBufferAllocator,
    _allocator: &mut VertexBufferAllocator3D,
    _device:&RenderDevice,
    _queue: &PiRenderQueue,
) {
    // let data = instancedcache.collect(&collected, instancedinfo.bytes_per_instance(), allocator, &device, &queue);
    // let data = if let Some(data) = data {
    //     EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))
    // } else {
    //     let data = instancedcache.instance_initial_buffer();
    //     EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))
    // };

    let data = collected;
    reset_instances_buffer_range(idgeo, instancedinfo, slots, data, count);
}

#[inline(never)]
pub fn reset_instances_buffer_range(
    idgeo: Entity,
    instancedinfo: &InstancedInfo,
    slots: &mut Query<(&AssetDescVBSlots, &mut AssetResVBSlots, &mut LoadedKeyVBSlots, &mut FlagGeometryDirty)>,
    data: Vec<u8>,
    count: u32,
) {
    let data = EVerticesBufferTmp::Memory(EVerteicesMemory { data: data, itemcount: count as u32, slot: instancedinfo.slot() as u8 });
    if let Ok((desclist, mut buffer, mut keys, mut flag)) = slots.get_mut(idgeo)  { 
        match instancedinfo.slot() {
            EVertexBufferSlot::Slot01 => { buffer[0] = Some(AssetResVBSlot(data)); keys.0[0] = desclist.key(0); *flag = FlagGeometryDirty; },
            EVertexBufferSlot::Slot02 => { buffer[1] = Some(AssetResVBSlot(data)); keys.0[1] = desclist.key(1); *flag = FlagGeometryDirty; },
            EVertexBufferSlot::Slot03 => { buffer[2] = Some(AssetResVBSlot(data)); keys.0[2] = desclist.key(2); *flag = FlagGeometryDirty; },
            EVertexBufferSlot::Slot04 => { buffer[3] = Some(AssetResVBSlot(data)); keys.0[3] = desclist.key(3); *flag = FlagGeometryDirty; },
            EVertexBufferSlot::Slot05 => { buffer[4] = Some(AssetResVBSlot(data)); keys.0[4] = desclist.key(4); *flag = FlagGeometryDirty; },
            EVertexBufferSlot::Slot06 => { buffer[5] = Some(AssetResVBSlot(data)); keys.0[5] = desclist.key(5); *flag = FlagGeometryDirty; },
            EVertexBufferSlot::Slot07 => { buffer[6] = Some(AssetResVBSlot(data)); keys.0[6] = desclist.key(6); *flag = FlagGeometryDirty; },
            EVertexBufferSlot::Slot08 => { buffer[7] = Some(AssetResVBSlot(data)); keys.0[7] = desclist.key(7); *flag = FlagGeometryDirty; },
            _ => {}
        }
    }
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

fn update_instanced_buffer_for_single(
    oldbuffer: &mut EVerticesBufferTmp,
    collected: &[u8],
    instancedcache: &InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device:&RenderDevice,
    queue: &PiRenderQueue,
) {
    match oldbuffer {
        EVerticesBufferTmp::Memory(_) => {
            
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