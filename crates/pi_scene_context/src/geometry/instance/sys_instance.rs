
use pi_engine_shell::prelude::*;

use crate::{
    geometry::{
        vertex_buffer_useinfo::*, 
        base::*, 
    }, prelude::{AbstructMesh, GlobalEnable},
};

use super::*;

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
    pub fn sys_tick_instance_buffer_update<
        D: TInstanceData + Component,
        T: TInstanceBuffer + Component,
        F: TInstanceFlag + Component
    >(
        actives: Query<&GlobalEnable, With<AbstructMesh>>,
        instances: Query<&D>,
        mut sources: Query<
            (
                ObjectID,
                &InstanceSourceRefs, &GeometryID, &mut F,
            ),
            Changed<F>
        >,
        mut geometrys: Query<&mut T>,
        mut geoloader: ResMut<GeometryVBLoader>,
        mut vb_data_map: ResMut<VertexBufferDataMap3D>,
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
        mut allocator: ResMut<VertexBufferAllocator3D>,
        asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
    ) {
        let time = pi_time::Instant::now();
        sources.iter_mut().for_each(|(
            _,
            inslist, id_geo, mut flag,
        )| {
            // log::trace!("SysInstanceBufferUpdateFunc:");
            if flag.dirty() == false {
                return;
            }
            let id_geo = id_geo.0.clone();
            if let Ok(mut buffer) = geometrys.get_mut(id_geo.clone()) {
                log::debug!("SysInstanceBufferUpdateFunc: A, {:?}", inslist.len());
                let mut list = vec![];
                inslist.iter().for_each(|insid| {
                    if let (Ok(instance), Ok(abstructmesh)) = (instances.get(insid.clone()), actives.get(insid.clone())) {
                        if abstructmesh.0 {
                            list.push(instance);
                        }
                    }
                });
    
                if list.len() == 0 {
                    //
                } else {
                    flag.reset();
    
                    let data = D::collect(&list);
                    log::debug!("InstanceDataLen: {:?}", data.len());
                    let data = if data.len() > 0 {
                        data
                    } else {
                        return;
                    };
    
                    log::debug!("SysInstanceBufferUpdateFunc: B, {:?}", buffer.slot());
                    instance_buffer_update::<T>(
                        data,
                        id_geo,
                        &mut buffer,
                        &mut geoloader,
                        &mut vb_data_map,
                        &mut slots, &mut allocator, &asset_mgr,
                        &device, &queue
                    );
                }
            }
        });
        
        let time1 = pi_time::Instant::now();
        log::debug!("SysInstanceBufferUpdate<{}>: {:?}", T::display_name(), time1 - time);
    }
// }

pub fn geometry_update_instance_buffer<T: TInstanceBuffer>(
    data: Option<Vec<u8>>,
    id_geo: Entity,
    buffer: &mut T,
    geoloader: &mut GeometryVBLoader,
    vb_data_map: &mut VertexBufferDataMap3D,
) {
    let key = buffer.id();

    match buffer.slot() {
        EVertexBufferSlot::Slot01 => {
            geoloader.loader_01.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot02 => {
            geoloader.loader_02.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot03 => {
            geoloader.loader_03.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot04 => {
            geoloader.loader_04.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot05 => {
            geoloader.loader_05.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot06 => {
            geoloader.loader_06.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot07 => {
            geoloader.loader_07.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot08 => {
            geoloader.loader_08.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot09 => {
            geoloader.loader_09.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot10 => {
            geoloader.loader_10.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot11 => {
            geoloader.loader_11.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot12 => {
            geoloader.loader_12.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot13 => {
            geoloader.loader_13.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot14 => {
            geoloader.loader_14.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot15 => {
            geoloader.loader_15.request_instance(id_geo, &key, data, vb_data_map);
        },
        EVertexBufferSlot::Slot16 => {
            geoloader.loader_16.request_instance(id_geo, &key, data, vb_data_map);
        },
    }
}

pub fn instance_buffer_update<T: TInstanceBuffer>(
    data: Vec<u8>,
    id_geo: Entity,
    buffer: &mut T,
    geoloader: &mut GeometryVBLoader,
    vb_data_map: &mut VertexBufferDataMap3D,
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
    allocator: &mut VertexBufferAllocator,
    asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
    device: &RenderDevice,
    queue: &RenderQueue,
) {
    let key = buffer.id();

    match buffer.slot() {
        EVertexBufferSlot::Slot01 => {
            if let Ok(mut buffer) = slots.0.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_01.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot02 => {
            if let Ok(mut buffer) = slots.1.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_02.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot03 => {
            if let Ok(mut buffer) = slots.2.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_03.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot04 => {
            if let Ok(mut buffer) = slots.3.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_04.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot05 => {
            if let Ok(mut buffer) = slots.4.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_05.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot06 => {
            if let Ok(mut buffer) = slots.5.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_06.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot07 => {
            if let Ok(mut buffer) = slots.6.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_07.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot08 => {
            if let Ok(mut buffer) = slots.7.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_08.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot09 => {
            if let Ok(mut buffer) = slots.8.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_09.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot10 => {
            if let Ok(mut buffer) = slots.9.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_10.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot11 => {
            if let Ok(mut buffer) = slots.10.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_11.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot12 => {
            if let Ok(mut buffer) = slots.11.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_12.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot13 => {
            if let Ok(mut buffer) = slots.12.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_13.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot14 => {
            if let Ok(mut buffer) = slots.13.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_14.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot15 => {
            if let Ok(mut buffer) = slots.14.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_15.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
        EVertexBufferSlot::Slot16 => {
            if let Ok(mut buffer) = slots.15.get_mut(id_geo) {
                if let Some(newbuffer) = buffer.instance_update(device, queue, allocator, &data) {
                    *buffer = newbuffer;
                    return;
                }
            }
            geoloader.loader_16.request_instance(id_geo, &key, Some(data), vb_data_map);
        },
    }
}