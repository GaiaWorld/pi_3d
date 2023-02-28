use std::{time::Instant, marker::PhantomData};

use pi_assets::mgr::AssetMgr;
use pi_ecs::{prelude::{Query, ResMut, Res, Component, Commands}, query::{Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::{rhi::{device::RenderDevice, RenderQueue}, renderer::{vertex_buffer::{VertexBufferAllocator, EVertexBufferRange}, instance::{types::{TInstancedData, TInstanceFlag}, instanced_buffer::TInstancedBuffer}, vertex_buffer_desc::EVertexBufferSlot}};
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
    // fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
    //     vec![
    //         S::key(), SysInstancedBufferInitFunc::<T>::key()
    //     ]
    // }
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
                (
                    Option<&mut AssetDescVBSlot01>,
                    Option<&mut AssetDescVBSlot02>,
                    Option<&mut AssetDescVBSlot03>,
                    Option<&mut AssetDescVBSlot04>,
                    Option<&mut AssetDescVBSlot05>,
                    Option<&mut AssetDescVBSlot06>,
                    Option<&mut AssetDescVBSlot07>,
                    Option<&mut AssetDescVBSlot08>,
                    Option<&mut AssetDescVBSlot09>,
                    Option<&mut AssetDescVBSlot10>,
                    Option<&mut AssetDescVBSlot11>,
                    Option<&mut AssetDescVBSlot12>,
                    Option<&mut AssetDescVBSlot13>,
                    Option<&mut AssetDescVBSlot14>,
                    Option<&mut AssetDescVBSlot15>,
                    Option<&mut AssetDescVBSlot16>,
                )
            ),
            Changed<F>
        >,
        mut allocator: ResMut<VertexBufferAllocator>,
        asset_mgr: Res<Share<AssetMgr<EVertexBufferRange>>>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
        mut geo_flag_cmd: Commands<GameObject, RenderGeometryEable>,
        mut cmd_01: Commands<GameObject, AssetResVBSlot01>,
        mut cmd_02: Commands<GameObject, AssetResVBSlot02>,
        mut cmd_03: Commands<GameObject, AssetResVBSlot03>,
        mut cmd_04: Commands<GameObject, AssetResVBSlot04>,
        mut cmd_05: Commands<GameObject, AssetResVBSlot05>,
        mut cmd_06: Commands<GameObject, AssetResVBSlot06>,
        mut cmd_07: Commands<GameObject, AssetResVBSlot07>,
        mut cmd_08: Commands<GameObject, AssetResVBSlot08>,
        mut cmd_09: Commands<GameObject, AssetResVBSlot09>,
        mut cmd_10: Commands<GameObject, AssetResVBSlot10>,
        mut cmd_11: Commands<GameObject, AssetResVBSlot11>,
        mut cmd_12: Commands<GameObject, AssetResVBSlot12>,
        mut cmd_13: Commands<GameObject, AssetResVBSlot13>,
        mut cmd_14: Commands<GameObject, AssetResVBSlot14>,
        mut cmd_15: Commands<GameObject, AssetResVBSlot15>,
        mut cmd_16: Commands<GameObject, AssetResVBSlot16>,
    ) {
        let time = Instant::now();
        sources.iter_mut().for_each(|(
            id_obj,
            inslist, mut buffer, mut flag, mut geodisable,
            (
                desc01, desc02, desc03, desc04, desc05, desc06, 
                desc07, desc08, desc09, desc10, desc11, desc12, 
                desc13, desc14, desc15, desc16, 
            )
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

                if let Some(bufferrange) = D::collect(&list, key.clone(), &device, &queue, &mut allocator, &asset_mgr) {
                    
                    // log::debug!("SysInstanceBufferUpdateFunc: B, {:?}", buffer.slot());
                    match buffer.slot() {
                        EVertexBufferSlot::Slot01 => {
                            cmd_01.insert(id_obj, AssetResVBSlot01(bufferrange));
                            if let Some(mut desc) = desc01 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot02 => {
                            cmd_02.insert(id_obj, AssetResVBSlot02(bufferrange));
                            if let Some(mut desc) = desc02 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot03 => {
                            cmd_03.insert(id_obj, AssetResVBSlot03(bufferrange));
                            if let Some(mut desc) = desc03 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot04 => {
                            cmd_04.insert(id_obj, AssetResVBSlot04(bufferrange));
                            if let Some(mut desc) = desc04 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot05 => {
                            cmd_05.insert(id_obj, AssetResVBSlot05(bufferrange));
                            if let Some(mut desc) = desc05 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot06 => {
                            cmd_06.insert(id_obj, AssetResVBSlot06(bufferrange));
                            if let Some(mut desc) = desc06 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot07 => {
                            cmd_07.insert(id_obj, AssetResVBSlot07(bufferrange));
                            if let Some(mut desc) = desc07 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot08 => {
                            cmd_08.insert(id_obj, AssetResVBSlot08(bufferrange));
                            if let Some(mut desc) = desc08 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot09 => {
                            cmd_09.insert(id_obj, AssetResVBSlot09(bufferrange));
                            if let Some(mut desc) = desc09 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot10 => {
                            cmd_10.insert(id_obj, AssetResVBSlot10(bufferrange));
                            if let Some(mut desc) = desc10 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot11 => {
                            cmd_11.insert(id_obj, AssetResVBSlot11(bufferrange));
                            if let Some(mut desc) = desc11 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot12 => {
                            cmd_12.insert(id_obj, AssetResVBSlot12(bufferrange));
                            if let Some(mut desc) = desc12 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot13 => {
                            cmd_13.insert(id_obj, AssetResVBSlot13(bufferrange));
                            if let Some(mut desc) = desc12 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot14 => {
                            cmd_14.insert(id_obj, AssetResVBSlot14(bufferrange));
                            if let Some(mut desc) = desc12 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot15 => {
                            cmd_15.insert(id_obj, AssetResVBSlot15(bufferrange));
                            if let Some(mut desc) = desc12 { desc.0.key = key.clone() }
                        },
                        EVertexBufferSlot::Slot16 => {
                            cmd_16.insert(id_obj, AssetResVBSlot16(bufferrange));
                            if let Some(mut desc) = desc12 { desc.0.key = key.clone() }
                        },
                    }
                }
            }
        });
        
        let time1 = Instant::now();
        log::info!("SysInstancedBufferUpdate<{}>: {:?}", T::display_name(), time1 - time);
    }
}
