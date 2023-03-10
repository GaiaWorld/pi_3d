use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::{ResMut, Commands, Res};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::{renderer::{vertex_buffer::{VertexBufferAllocator, EVertexBufferRange}, vertex_buffer_loader::{VertexBufferLoader, SingleVertexBufferDataMap}, indices::AssetResBufferIndices}, rhi::{device::RenderDevice, RenderQueue}};
use pi_share::Share;

use super::vertex_buffer_useinfo::*;


pub struct SysVertexBufferLoad;
impl TSystemStageInfo for SysVertexBufferLoad {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            
        ]
    }
}
#[setup]
impl SysVertexBufferLoad {
    #[system]
    fn sys(
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
        asset_mgr: Res<Share<AssetMgr<EVertexBufferRange>>>,
        mut allocator: ResMut<VertexBufferAllocator>,
        mut data_map: ResMut<SingleVertexBufferDataMap>,
        mut vb01_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot01>>,
        mut vb02_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot02>>,
        mut vb03_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot03>>,
        mut vb04_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot04>>,
        mut vb05_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot05>>,
        mut vb06_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot06>>,
        mut vb07_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot07>>,
        mut vb08_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot08>>,
        mut vb09_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot09>>,
        mut vb10_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot10>>,
        mut vb11_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot11>>,
        mut vb12_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot12>>,
        mut vb13_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot13>>,
        mut vb14_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot14>>,
        mut vb15_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot15>>,
        mut vb16_loader: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot16>>,
        mut vb01_cmd: Commands<GameObject, AssetResVBSlot01>,
        mut vb02_cmd: Commands<GameObject, AssetResVBSlot02>,
        mut vb03_cmd: Commands<GameObject, AssetResVBSlot03>,
        mut vb04_cmd: Commands<GameObject, AssetResVBSlot04>,
        mut vb05_cmd: Commands<GameObject, AssetResVBSlot05>,
        mut vb06_cmd: Commands<GameObject, AssetResVBSlot06>,
        mut vb07_cmd: Commands<GameObject, AssetResVBSlot07>,
        mut vb08_cmd: Commands<GameObject, AssetResVBSlot08>,
        mut vb09_cmd: Commands<GameObject, AssetResVBSlot09>,
        mut vb10_cmd: Commands<GameObject, AssetResVBSlot10>,
        mut vb11_cmd: Commands<GameObject, AssetResVBSlot11>,
        mut vb12_cmd: Commands<GameObject, AssetResVBSlot12>,
        mut vb13_cmd: Commands<GameObject, AssetResVBSlot13>,
        mut vb14_cmd: Commands<GameObject, AssetResVBSlot14>,
        mut vb15_cmd: Commands<GameObject, AssetResVBSlot15>,
        mut vb16_cmd: Commands<GameObject, AssetResVBSlot16>,
        mut indices_loader: ResMut<VertexBufferLoader<ObjectID, AssetResBufferIndices>>,
        mut indices_cmd: Commands<GameObject, AssetResBufferIndices>,
    ) {
        let mut data0 = data_map.single_create(&device, &queue, &mut allocator, &asset_mgr);
        let mut data2 = data_map.single_create_instance(&device, &queue, &mut allocator);
        data2.drain().for_each(|(k, v)| { data0.insert(k, v); });
        data0.drain().for_each(|(key, range)| {
            vb01_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad 01");
                vb01_cmd.insert(id, data);
            });
            vb02_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad 02");
                vb02_cmd.insert(id, data);
            });
            vb03_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad 03");
                vb03_cmd.insert(id, data);
            });
            vb04_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad 04");
                vb04_cmd.insert(id, data);
            });
            vb05_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad 05");
                vb05_cmd.insert(id, data);
            });
            vb06_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad 06");
                vb06_cmd.insert(id, data);
            });
            vb07_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad 07");
                vb07_cmd.insert(id, data);
            });
            vb08_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb08_cmd.insert(id, data);
            });
            vb09_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb09_cmd.insert(id, data);
            });
            vb10_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb10_cmd.insert(id, data);
            });
            vb11_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb11_cmd.insert(id, data);
            });
            vb12_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb12_cmd.insert(id, data);
            });
            vb13_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb13_cmd.insert(id, data);
            });
            vb14_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb14_cmd.insert(id, data);
            });
            vb15_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb15_cmd.insert(id, data);
            });
            vb16_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb16_cmd.insert(id, data);
            });
            vb16_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                vb16_cmd.insert(id, data);
            });
            indices_loader.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                log::info!("SysVertexBufferLoad Idx");
                indices_cmd.insert(id, data);
            });
        });
    }
}