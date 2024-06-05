
use pi_scene_shell::prelude::*;

use super::{base::*, AssetDescVBSlots, AssetResVBSlots, LoadedKeyVBSlots};

    pub fn sys_vertex_buffer_loaded(
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
        asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
        mut allocator: ResMut<VertexBufferAllocator3D>,
        mut data_map: ResMut<VertexBufferDataMap3D>,
        mut geoloader: ResMut<GeometryVBLoader>,
        mut geometries: Query<(&AssetDescVBSlots, &mut LoadedKeyVBSlots, &mut AssetResVBSlots, &mut AssetResBufferIndicesComp, &mut AssetKeyBufferIndices, &IndicesBufferDescComp)>,
    ) {
        let mut data0 = data_map.single_create(&device, &queue, &mut allocator, &asset_mgr);
        let mut data2 = data_map.single_create_instance(&device, &queue, &mut allocator);
        data2.drain().for_each(|(k, v)| { data0.insert(k, v); });
        data0.drain().for_each(|(key, range)| {
            geoloader.loader_01.loaded(&key, &range).drain(..).for_each(|((id, slot), data)| {
                if let Ok((keys, mut loadedkeys, mut reslist, _, _, _)) = geometries.get_mut(id) {
                    if let Some(req) = &keys[slot as usize] {
                        if &req.0.key == &key {
                            loadedkeys[slot as usize] = Some(key.clone());
                            reslist[slot as usize] = Some(data);
                        }
                    }
                    // log::warn!("SysVertexBufferLoad  Ok {:?}", key);
                } else {
                    // log::warn!("SysVertexBufferLoad  None {:?}", key);
                }
            });
        });
        
        let mut data1 = data_map.single_create_indices(&device, &queue, &mut allocator, &asset_mgr);
        data1.drain().for_each(|(key, range)| {
            // log::warn!("SysVertexBufferLoad {:?}", key);
            geoloader.loader_indices.loaded(&key, &range).drain(..).for_each(|(id, data)| {
                if let Ok((_, _, _, mut buffer, mut loadedkey, desc)) = geometries.get_mut(id) {
                    if let Some(desc) = &desc.0 {
                        if &desc.buffer == &key {
                            buffer.0 = Some(data);
                            loadedkey.0 = Some(key.clone());
                        }
                    }
                }
            });
        });
    }