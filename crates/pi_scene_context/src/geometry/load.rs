
use pi_scene_shell::prelude::*;

use super::{base::*, AssetResVBSlot01, AssetResVBSlot02, AssetResVBSlot03, AssetResVBSlot04, AssetResVBSlot05, AssetResVBSlot06, AssetResVBSlot07, AssetResVBSlot08};


// pub struct SysVertexBufferLoad;
// impl TSystemStageInfo for SysVertexBufferLoad {
//     fn depends() -> Vec<pi_scene_shell::run_stage::KeySystem> {
//         vec![
            
//         ]
//     }
// }
// #[setup]
// impl SysVertexBufferLoad {
//     #[system]
    pub fn sys_vertex_buffer_loaded(
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
        asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
        mut allocator: ResMut<VertexBufferAllocator3D>,
        mut data_map: ResMut<VertexBufferDataMap3D>,
        mut geoloader: ResMut<GeometryVBLoader>,
        // mut commands: Commands,
        mut alter1: Alter<(), (), (AssetResVBSlot01,), ()>,
        mut alter2: Alter<(), (), (AssetResVBSlot02,), ()>,
        mut alter3: Alter<(), (), (AssetResVBSlot03,), ()>,
        mut alter4: Alter<(), (), (AssetResVBSlot04,), ()>,
        mut alter5: Alter<(), (), (AssetResVBSlot05,), ()>,
        mut alter6: Alter<(), (), (AssetResVBSlot06,), ()>,
        mut alter7: Alter<(), (), (AssetResVBSlot07,), ()>,
        mut alter8: Alter<(), (), (AssetResVBSlot08,), ()>,
        mut alter9: Alter<(), (), (AssetResBufferIndices,), ()>,
    ) {
        let mut data0 = data_map.single_create(&device, &queue, &mut allocator, &asset_mgr);
        let mut data2 = data_map.single_create_instance(&device, &queue, &mut allocator);
        data2.drain().for_each(|(k, v)| { data0.insert(k, v); });
        data0.drain().for_each(|(key, range)| {
            geoloader.loader_01.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 01");
                if alter1.get(id).is_ok() {
                    alter1.alter(id, (data,));
                }
            });
            geoloader.loader_02.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 02");
                if alter2.get(id).is_ok() {
                    alter2.alter(id, (data,));
                }
            });
            geoloader.loader_03.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 03");
                if alter3.get(id).is_ok() {
                    alter3.alter(id, (data,));
                }
            });
            geoloader.loader_04.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 04");
                if alter4.get(id).is_ok() {
                    alter4.alter(id, (data,));
                }
            });
            geoloader.loader_05.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 05");
                if alter5.get(id).is_ok() {
                    alter5.alter(id, (data,));
                }
            });
            geoloader.loader_06.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 06");
                if alter6.get(id).is_ok() {
                    alter6.alter(id, (data,));
                }
            });
            geoloader.loader_07.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 07");
                if alter7.get(id).is_ok() {
                    alter7.alter(id, (data,));
                }
            });
            geoloader.loader_08.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                
                if alter8.get(id).is_ok() {
                    alter8.alter(id, (data,));
                }
            });
            // geoloader.loader_09.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
            // geoloader.loader_10.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
            // geoloader.loader_11.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
            // geoloader.loader_12.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
            // geoloader.loader_13.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
            // geoloader.loader_14.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
            // geoloader.loader_15.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
            // geoloader.loader_16.loaded(&key, &range).drain(..).for_each(|(id, data)| {
            //     if let Some(mut cmd) = commands.get_entity(id) {
            //         cmd.insert(data);
            //     }
            // });
        });
        
        let mut data1 = data_map.single_create_indices(&device, &queue, &mut allocator, &asset_mgr);
        data1.drain().for_each(|(key, range)| {
            
            // log::warn!("SysVertexBufferLoad {:?}", key);
            geoloader.loader_indices.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::warn!("SysVertexBufferLoad Idx");
                if alter9.get(id).is_ok() {
                    alter9.alter(id, (data,));
                }
            });
        });
    }
// }