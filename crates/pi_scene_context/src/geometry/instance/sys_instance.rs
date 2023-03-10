use std::{time::Instant, marker::PhantomData};

use pi_assets::mgr::AssetMgr;
use pi_ecs::{prelude::{Query, ResMut, Res, Component, Commands}, query::{Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::{rhi::{device::RenderDevice, RenderQueue}, renderer::{vertex_buffer::{VertexBufferAllocator, EVertexBufferRange}, instance::{types::{TInstancedData, TInstanceFlag}, instanced_buffer::TInstancedBuffer}, vertex_buffer_desc::EVertexBufferSlot, vertex_buffer_loader::{SingleVertexBufferDataMap, VertexBufferLoader}}};
use pi_share::Share;

use crate::{
    geometry::{
        vertex_buffer_useinfo::*, 
        geometry::RenderGeometryEable, 
    },
};

use super::{InstanceList};

///
/// T: Mesh 中 保存实例数据的buffer
/// D: 实例数据
/// F: 实例数据在Mesh上的脏标识
/// S: 脏标识更新的System
pub struct SysInstanceBufferUpdateFunc<D: TInstancedData + Component, T: TInstancedBuffer + Component, F: TInstanceFlag + Component, S: TSystemStageInfo>(PhantomData<(D, T, F, S)>);
impl<D: TInstancedData + Component, T: TInstancedBuffer + Component, F: TInstanceFlag + Component, S: TSystemStageInfo> TSystemStageInfo for SysInstanceBufferUpdateFunc<D, T, F, S> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            S::key(),
        ]
    }
}
#[setup]
impl<D: TInstancedData + Component, T: TInstancedBuffer + Component, F: TInstanceFlag + Component, S: TSystemStageInfo + 'static> SysInstanceBufferUpdateFunc<D, T, F, S> {
    #[system]
    pub fn tick(
        instances: Query<GameObject, &D>,
        mut sources: Query<
            GameObject,
            (
                ObjectID,
                &InstanceList, &mut T, &mut F, &mut RenderGeometryEable,
            ),
            Changed<F>
        >,
        mut geo_flag_cmd: Commands<GameObject, RenderGeometryEable>,
        mut loader_01: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot01>>,
        mut loader_02: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot02>>,
        mut loader_03: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot03>>,
        mut loader_04: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot04>>,
        mut loader_05: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot05>>,
        mut loader_06: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot06>>,
        mut loader_07: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot07>>,
        mut loader_08: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot08>>,
        mut loader_09: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot09>>,
        mut loader_10: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot10>>,
        mut loader_11: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot11>>,
        mut loader_12: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot12>>,
        mut loader_13: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot13>>,
        mut loader_14: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot14>>,
        mut loader_15: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot15>>,
        mut loader_16: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot16>>,
        mut vb_data_map: ResMut<SingleVertexBufferDataMap>,
    ) {
        let time = Instant::now();
        sources.iter_mut().for_each(|(
            id_obj,
            inslist, mut buffer, mut flag, mut geodisable,
        )| {
            // log::trace!("SysInstanceBufferUpdateFunc:");
            if flag.dirty() == false {
                return;
            }
            log::info!("SysInstanceBufferUpdateFunc: A, {:?}", inslist.list.len());
            let mut list = vec![];
            inslist.list.iter().for_each(|insid| {
                if let Some(instance) = instances.get(insid.clone()) {
                    list.push(instance);
                }
            });

            if list.len() == 0 {
                geodisable.0 = false;
                geo_flag_cmd.insert(id_obj, RenderGeometryEable(false));
            } else {
                geodisable.0 = true;
                geo_flag_cmd.insert(id_obj, RenderGeometryEable(true));
                flag.reset();

                let key = buffer.id();

                let data = D::collect(&list);
                log::info!("InstanceDataLen: {:?}", data.len());
                let data = if data.len() > 0 {
                    Some(data)
                } else {
                    return;
                };
                    

                log::info!("SysInstanceBufferUpdateFunc: B, {:?}", buffer.slot());
                match buffer.slot() {
                    EVertexBufferSlot::Slot01 => {
                        loader_01.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot02 => {
                        loader_02.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot03 => {
                        loader_03.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot04 => {
                        loader_04.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot05 => {
                        loader_05.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot06 => {
                        loader_06.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot07 => {
                        loader_07.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot08 => {
                        loader_08.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot09 => {
                        loader_09.request(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot10 => {
                        loader_10.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot11 => {
                        loader_11.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot12 => {
                        loader_12.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot13 => {
                        loader_13.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot14 => {
                        loader_14.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot15 => {
                        loader_15.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                    EVertexBufferSlot::Slot16 => {
                        loader_16.request_instance(id_obj, &key, data, &mut vb_data_map);
                    },
                }
            }
        });
        
        let time1 = Instant::now();
        log::info!("SysInstancedBufferUpdate<{}>: {:?}", T::display_name(), time1 - time);
    }
}
