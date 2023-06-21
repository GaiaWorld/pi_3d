
use pi_engine_shell::prelude::*;

use crate::{
    geometry::{
        vertex_buffer_useinfo::*, 
        base::*, 
    }, prelude::AbstructMesh,
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
        actives: Query<&AbstructMesh>,
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
                        Some(data)
                    } else {
                        return;
                    };
    
                    log::debug!("SysInstanceBufferUpdateFunc: B, {:?}", buffer.slot());
                    geometry_update_instance_buffer::<T>(
                        data,
                        id_geo,
                        &mut buffer,
                        &mut geoloader,
                        &mut vb_data_map
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