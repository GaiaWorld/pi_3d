use pi_assets::mgr::AssetMgr;
use pi_engine_shell::prelude::*;
use pi_render::{renderer::{vertex_buffer::{VertexBufferAllocator, EVertexBufferRange}, vertex_buffer_loader::{SingleVertexBufferDataMap}, indices::AssetResBufferIndices}, rhi::{device::RenderDevice, RenderQueue}};
use pi_share::Share;

use super::{vertex_buffer_useinfo::*, base::VBLoaderSlot};


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
        mut allocator: ResMut<VertexBufferAllocator>,
        mut data_map: ResMut<SingleVertexBufferDataMap>,
        mut vb01_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot01>>,
        mut vb02_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot02>>,
        mut vb03_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot03>>,
        mut vb04_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot04>>,
        mut vb05_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot05>>,
        mut vb06_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot06>>,
        mut vb07_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot07>>,
        mut vb08_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot08>>,
        mut vb09_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot09>>,
        mut vb10_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot10>>,
        mut vb11_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot11>>,
        mut vb12_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot12>>,
        mut vb13_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot13>>,
        mut vb14_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot14>>,
        mut vb15_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot15>>,
        mut vb16_loader: ResMut<VBLoaderSlot<ObjectID, AssetResVBSlot16>>,
        mut commands: Commands,
        mut indices_loader: ResMut<VBLoaderSlot<ObjectID, AssetResBufferIndices>>,
    ) {
        let mut data0 = data_map.single_create(&device, &queue, &mut allocator, &asset_mgr);
        let mut data2 = data_map.single_create_instance(&device, &queue, &mut allocator);
        data2.drain().for_each(|(k, v)| { data0.insert(k, v); });
        data0.drain().for_each(|(key, range)| {
            vb01_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad 01");
                commands.entity(id).insert(data);
            });
            vb02_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad 02");
                commands.entity(id).insert(data);
            });
            vb03_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad 03");
                commands.entity(id).insert(data);
            });
            vb04_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad 04");
                commands.entity(id).insert(data);
            });
            vb05_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad 05");
                commands.entity(id).insert(data);
            });
            vb06_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad 06");
                commands.entity(id).insert(data);
            });
            vb07_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad 07");
                commands.entity(id).insert(data);
            });
            vb08_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb09_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb10_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb11_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb12_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb13_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb14_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb15_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb16_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            vb16_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                commands.entity(id).insert(data);
            });
            indices_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::debug!("SysVertexBufferLoad Idx");
                commands.entity(id).insert(data);
            });
        });
    }
// }