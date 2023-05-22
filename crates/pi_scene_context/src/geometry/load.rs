
use pi_engine_shell::prelude::*;

use super::{vertex_buffer_useinfo::*, base::*};


// pub struct SysVertexBufferLoad;
// impl TSystemStageInfo for SysVertexBufferLoad {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
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
        mut commands: Commands,
    ) {
        let mut data0 = data_map.single_create(&device, &queue, &mut allocator, &asset_mgr);
        let mut data2 = data_map.single_create_instance(&device, &queue, &mut allocator);
        data2.drain().for_each(|(k, v)| { data0.insert(k, v); });
        data0.drain().for_each(|(key, range)| {
            geoloader.loader_01.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 01");
                commands.entity(id).insert(data);
            });
            geoloader.loader_02.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 02");
                commands.entity(id).insert(data);
            });
            geoloader.loader_03.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 03");
                commands.entity(id).insert(data);
            });
            geoloader.loader_04.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 04");
                commands.entity(id).insert(data);
            });
            geoloader.loader_05.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 05");
                commands.entity(id).insert(data);
            });
            geoloader.loader_06.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 06");
                commands.entity(id).insert(data);
            });
            geoloader.loader_07.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::debug!("SysVertexBufferLoad 07");
                commands.entity(id).insert(data);
            });
            geoloader.loader_08.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_09.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_10.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_11.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_12.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_13.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_14.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_15.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            geoloader.loader_16.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
        });
        
        let mut data1 = data_map.single_create_indices(&device, &queue, &mut allocator, &asset_mgr);
        data1.drain().for_each(|(key, range)| {
            
            // log::warn!("SysVertexBufferLoad {:?}", key);
            geoloader.loader_indices.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                // log::warn!("SysVertexBufferLoad Idx");
                commands.entity(id).insert(data);
            });
        });
    }
// }