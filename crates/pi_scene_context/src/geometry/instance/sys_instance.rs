
use std::{sync::Arc, ops::Range};

use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix};

use crate::{
    geometry::vertex_buffer_useinfo::*,
    prelude::*,
};

use super::{*, instanced_buffer::*, instance_vec4::*};

///
/// T: Mesh 中 保存实例数据的buffer
/// D: 实例数据
/// F: 实例数据在Mesh上的脏标识
/// S: 脏标识更新的System
// pub struct SysInstanceBufferUpdateFunc<D: TInstanceData + Component, T: TInstanceBuffer + Component, F: TInstanceFlag + Component, S: TSystemStageInfo>(PhantomData<(D, T, F, S)>);
// impl<D: TInstanceData + Component, T: TInstanceBuffer + Component, F: TInstanceFlag + Component, S: TSystemStageInfo> TSystemStageInfo for SysInstanceBufferUpdateFunc<D, T, F, S> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             S::key(),
//         ]
//     }
// }
// #[setup]
// impl<D: TInstanceData + Component, T: TInstanceBuffer + Component, F: TInstanceFlag + Component, S: TSystemStageInfo + 'static> SysInstanceBufferUpdateFunc<D, T, F, S> {
//     #[system]

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

    pub fn sys_instance_color(
        mut items: Query<(&InstanceMesh, &InstanceRGB, &InstanceAlpha, &mut InstanceColor), Or<(Changed<InstanceRGB>, Changed<InstanceAlpha>)>>,
        mut sources: Query<&mut InstanceColorDirty>
    ) {
        items.iter_mut().for_each(|(source, rgb, alpha, mut color)| {
            *color = InstanceColor(Vector4::new(rgb.0, rgb.1, rgb.2, alpha.0));
            if let Ok(mut flag) = sources.get_mut(source.0) {
                *flag = InstanceColorDirty(true);
            }
        });
    }

    pub fn sys_tick_instanced_buffer_update_single(
        actives: Query<(&GlobalEnable, &InstanceMesh, &InstanceTransparentIndex, &AbstructMeshCullingFlag), With<AbstructMesh>>,
        instancematrixs: Query<&RenderWorldMatrix>,
        instancecolors: Query<&InstanceColor>,
        instancetilloffs: Query<&InstanceTillOff>,
        instancevec4_a: Query<&InstanceCustomVec4A>,
        instancevec4_b: Query<&InstanceCustomVec4B>,
        instancevec4_c: Query<&InstanceCustomVec4C>,
        instancevec4_d: Query<&InstanceCustomVec4D>,
        mut sources: Query<
            (
                Entity, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut RenderGeometryEable, &mut InstancedMeshTransparentSortCollection
            ),
            Or<(
                Changed<InstanceColorDirty>, Changed<InstanceTillOffDirty>, Changed<InstanceWorldMatrixDirty>, Changed<InstanceSourceRefs>, Changed<MeshInstanceState>
                , Changed<InstanceCustomVec4ADirty>, Changed<InstanceCustomVec4BDirty>, Changed<InstanceCustomVec4CDirty>, Changed<InstanceCustomVec4DDirty>
            )>
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
        // log::warn!("Instance Update");
        let defaultcolor = InstanceColor(Vector4::new(1., 1., 1., 1.));
        let defaulttilloff = InstanceTillOff(Vector4::new(1., 1., 0., 0.));
        let defaultmatrix = RenderWorldMatrix(Matrix::identity());
        sources.iter_mut().for_each(|(idsource, instances, idgeo, meshinsstate, mut renderenable, mut instancessortinfos)| {
            if let Ok(disposed) = dispoeds.get(idsource) {
                if disposed.0 == true { return; }
                if meshinsstate.use_single_instancebuffer == false { return; }
                // *renderenable = RenderGeometryEable(false);

                if let Ok(buffer) = geometrys.get(idgeo.0) {
                    if buffer.state > 0 {
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
    
                            if (buffer.state & InstanceState::INSTANCE_INDEX) == InstanceState::INSTANCE_INDEX {
                                bytemuck::cast_slice(&[idx]).iter().for_each(|v| { collected.push(*v); });
                            }
                            if (buffer.state & InstanceState::INSTANCE_BASE) == InstanceState::INSTANCE_BASE {
                                if let Ok(item) = instancematrixs.get(instance) {
                                    bytemuck::cast_slice(item.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(defaultmatrix.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_COLOR) == InstanceState::INSTANCE_COLOR {
                                if let Ok(item) = instancecolors.get(instance) {
                                    bytemuck::cast_slice(item.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(defaultcolor.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_TILL_OFF_1) == InstanceState::INSTANCE_TILL_OFF_1 {
                                if let Ok(item) = instancetilloffs.get(instance) {
                                    bytemuck::cast_slice(item.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(defaulttilloff.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_A) == InstanceState::INSTANCE_CUSTOM_VEC4_A {
                                if let Ok(v) = instancevec4_a.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_B) == InstanceState::INSTANCE_CUSTOM_VEC4_B {
                                if let Ok(v) = instancevec4_b.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_C) == InstanceState::INSTANCE_CUSTOM_VEC4_C {
                                if let Ok(v) = instancevec4_c.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_D) == InstanceState::INSTANCE_CUSTOM_VEC4_D {
                                if let Ok(v) = instancevec4_d.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
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
        });
    }


    pub fn sys_tick_instanced_buffer_update(
        actives: Query<(&GlobalEnable, &InstanceMesh, &InstanceTransparentIndex, &AbstructMeshCullingFlag), With<AbstructMesh>>,
        instancematrixs: Query<&RenderWorldMatrix>,
        instancecolors: Query<&InstanceColor>,
        instancetilloffs: Query<&InstanceTillOff>,
        instancevec4_a: Query<&InstanceCustomVec4A>,
        instancevec4_b: Query<&InstanceCustomVec4B>,
        instancevec4_c: Query<&InstanceCustomVec4C>,
        instancevec4_d: Query<&InstanceCustomVec4D>,
        mut sources: Query<
            (
                Entity, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut RenderGeometryEable, &mut InstancedMeshTransparentSortCollection
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
        mut instancedcache: ResMut<InstanceBufferAllocator>,
        mut allocator: ResMut<VertexBufferAllocator3D>,
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
    ) {
        // log::warn!("Instance Update");
        let defaultcolor = InstanceColor(Vector4::new(1., 1., 1., 1.));
        let defaulttilloff = InstanceTillOff(Vector4::new(1., 1., 0., 0.));
        let defaultmatrix = RenderWorldMatrix(Matrix::identity());
        sources.iter_mut().for_each(|(idsource, instances, idgeo, meshinsstate, mut renderenable, mut instancessortinfos)| {
            if let Ok(disposed) = dispoeds.get(idsource) {
                if disposed.0 == true { return; }
                if meshinsstate.use_single_instancebuffer == true { return; }
                // *renderenable = RenderGeometryEable(false);

                if let Ok(buffer) = geometrys.get(idgeo.0) {
                    if buffer.state > 0 {
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
    
                            if (buffer.state & InstanceState::INSTANCE_INDEX) == InstanceState::INSTANCE_INDEX {
                                bytemuck::cast_slice(&[idx]).iter().for_each(|v| { collected.push(*v); });
                            }
                            if (buffer.state & InstanceState::INSTANCE_BASE) == InstanceState::INSTANCE_BASE {
                                if let Ok(item) = instancematrixs.get(instance) {
                                    bytemuck::cast_slice(item.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(defaultmatrix.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_COLOR) == InstanceState::INSTANCE_COLOR {
                                if let Ok(item) = instancecolors.get(instance) {
                                    bytemuck::cast_slice(item.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(defaultcolor.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_TILL_OFF_1) == InstanceState::INSTANCE_TILL_OFF_1 {
                                if let Ok(item) = instancetilloffs.get(instance) {
                                    bytemuck::cast_slice(item.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(defaulttilloff.0.as_slice()).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_A) == InstanceState::INSTANCE_CUSTOM_VEC4_A {
                                if let Ok(v) = instancevec4_a.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_B) == InstanceState::INSTANCE_CUSTOM_VEC4_B {
                                if let Ok(v) = instancevec4_b.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_C) == InstanceState::INSTANCE_CUSTOM_VEC4_C {
                                if let Ok(v) = instancevec4_c.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
                            if (buffer.state & InstanceState::INSTANCE_CUSTOM_VEC4_D) == InstanceState::INSTANCE_CUSTOM_VEC4_D {
                                if let Ok(v) = instancevec4_d.get(instance) {
                                    bytemuck::cast_slice(&[v.0, v.1, v.2, v.3]).iter().for_each(|v| { collected.push(*v); });
                                } else {
                                    bytemuck::cast_slice(&[0., 0., 0., 0.]).iter().for_each(|v| { collected.push(*v); });
                                }
                            }
    
                            idx += 0;
                        });
                        if tmp_instance_start != tmp_instance_end {
                            instancessortinfos.0.push((tmp_alphaindex, Range { start: tmp_instance_start, end: tmp_instance_end }));
                        }
                        reset_instances_buffer(idgeo.0, buffer, &collected, &mut slots, &mut instancedcache, &mut allocator, &device, &queue);
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

pub fn reset_instances_buffer(
    idgeo: Entity,
    instancedinfo: &InstancedInfo,
    collected: &[u8],
    slots: &mut (
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
    instancedcache: &mut InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device:&RenderDevice,
    queue: &PiRenderQueue,
) {
    let data = instancedcache.collect(&collected, instancedinfo.bytes_per_instance(), allocator, &device, &queue);
    let data = if let Some(data) = data {
        EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))
    } else {
        let data = instancedcache.instance_initial_buffer();
        EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))
    };

    reset_instances_buffer_range(idgeo, instancedinfo, slots, data);
}

pub fn reset_instances_buffer_range(
    idgeo: Entity,
    instancedinfo: &InstancedInfo,
    slots: &mut (
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
    data: EVerticesBufferUsage
) {
    match instancedinfo.slot() {
        EVertexBufferSlot::Slot01 => if let Ok(mut buffer) = slots.0.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot02 => if let Ok(mut buffer) = slots.1.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot03 => if let Ok(mut buffer) = slots.2.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot04 => if let Ok(mut buffer) = slots.3.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot05 => if let Ok(mut buffer) = slots.4.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot06 => if let Ok(mut buffer) = slots.5.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot07 => if let Ok(mut buffer) = slots.6.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot08 => if let Ok(mut buffer) = slots.7.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot09 => if let Ok(mut buffer) = slots.8.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot10 => if let Ok(mut buffer) = slots.9.get_mut(idgeo)  { buffer.0 = data; },
        EVertexBufferSlot::Slot11 => if let Ok(mut buffer) = slots.10.get_mut(idgeo) { buffer.0 = data; },
        EVertexBufferSlot::Slot12 => if let Ok(mut buffer) = slots.11.get_mut(idgeo) { buffer.0 = data; },
        EVertexBufferSlot::Slot13 => if let Ok(mut buffer) = slots.12.get_mut(idgeo) { buffer.0 = data; },
        EVertexBufferSlot::Slot14 => if let Ok(mut buffer) = slots.13.get_mut(idgeo) { buffer.0 = data; },
        EVertexBufferSlot::Slot15 => if let Ok(mut buffer) = slots.14.get_mut(idgeo) { buffer.0 = data; },
        EVertexBufferSlot::Slot16 => if let Ok(mut buffer) = slots.15.get_mut(idgeo) { buffer.0 = data; },
    };
}

pub fn reset_instances_buffer_single(
    idgeo: Entity,
    instancedinfo: &InstancedInfo,
    collected: &[u8],
    slots: &mut (
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
    instancedcache: &InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device:&RenderDevice,
    queue: &PiRenderQueue,
) {
    match instancedinfo.slot() {
        EVertexBufferSlot::Slot01 => if let Ok(mut buffer) = slots.0.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot02 => if let Ok(mut buffer) = slots.1.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot03 => if let Ok(mut buffer) = slots.2.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot04 => if let Ok(mut buffer) = slots.3.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot05 => if let Ok(mut buffer) = slots.4.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot06 => if let Ok(mut buffer) = slots.5.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot07 => if let Ok(mut buffer) = slots.6.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot08 => if let Ok(mut buffer) = slots.7.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot09 => if let Ok(mut buffer) = slots.8.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot10 => if let Ok(mut buffer) = slots.9.get_mut(idgeo)  { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot11 => if let Ok(mut buffer) = slots.10.get_mut(idgeo) { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot12 => if let Ok(mut buffer) = slots.11.get_mut(idgeo) { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot13 => if let Ok(mut buffer) = slots.12.get_mut(idgeo) { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot14 => if let Ok(mut buffer) = slots.13.get_mut(idgeo) { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot15 => if let Ok(mut buffer) = slots.14.get_mut(idgeo) { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
        EVertexBufferSlot::Slot16 => if let Ok(mut buffer) = slots.15.get_mut(idgeo) { update_instanced_buffer_for_single(&mut buffer.0, collected, instancedcache, allocator, device, queue) },
    };
}

fn update_instanced_buffer_for_single(
    oldbuffer: &mut EVerticesBufferUsage,
    collected: &[u8],
    instancedcache: &InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device:&RenderDevice,
    queue: &PiRenderQueue,
) {
    match oldbuffer {
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
    }
}