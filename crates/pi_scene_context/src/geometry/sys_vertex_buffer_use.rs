
use std::{ops::Range, sync::Arc};

use pi_scene_shell::prelude::*;

use super::{
    base::GeometryDesc, geometry::*, vertex_buffer_useinfo::*, FlagGeometryDirty
};

#[inline(never)]
fn _sys_vertex_buffer_slots_loaded(
    mut values: Vec<(wgpu::VertexStepMode, RenderVertices)>,
    mut instance_memory: Option<Arc<EVerteicesInstance>>,
    res: &EVerticesBufferTmp,
    desc: &GeometryDesc,
    geometry: &mut RenderGeometryComp,
    rendergeo:&mut RenderGeometryEable,
    slot: u32,
    buffer_range: Option<Range<u64>>,
    buffdesc: &VertexBufferDesc,
    indicesdesc: Option<&IndicesBufferDesc>,
    indices: Option<&AssetResBufferIndices>
) -> Option<(Vec<(wgpu::VertexStepMode, RenderVertices)>, Option<Arc<EVerteicesInstance>>)> {
    match res {
        EVerticesBufferTmp::Instance(mem) => { instance_memory = Some(mem.clone()); },
        EVerticesBufferTmp::Buffer(buf) => {
            let buff = RenderVertices {
                slot,
                buffer: buf.clone(),
                buffer_range,
                size_per_value: buffdesc.stride()
            };
            values.push((buffdesc.step_mode(), buff));
        },
    }
    if desc.slot_count() == (slot + 1) as usize {
        geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices), instance_memory));
        *rendergeo = RenderGeometryEable(true);
        return None;
    } else {
        return Some((values, instance_memory));
    }
}

pub fn sys_vertex_buffer_slots_loaded(
    addeds: ComponentAdded<FlagGeometryDirty>,
    changes: ComponentChanged<FlagGeometryDirty>,
    items: Query<
        (
            Entity, 
            (&MeshID, &GeometryDesc, &IndicesBufferDescComp, &AssetResBufferIndicesComp, &AssetKeyBufferIndices)
            , &AssetDescVBSlots, &AssetResVBSlots, &LoadedKeyVBSlots
        )
    >,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut meshes: Query<&mut RenderGeometryEable>,
    // devicelimits: Res<DeviceLimits3D>,
) {
    let mut counter = 0;
    let changes = changes.iter().chain(addeds.iter());
    changes.for_each(|entity| {
        if let Ok((
            idgeo, 
            (idmesh, desc, indicesdesc, indices, indiceskey)
            , desclist, datalist, keyslist
        )) = items.get(*entity) {
            if let (Ok(mut geometry), Ok(mut rendergeo)) = (geometries.get_mut(idgeo), meshes.get_mut(idmesh.0)) {
                counter += 1;

                let mut values = vec![];
                let mut instance_memory = None;
                let mut isready = true;

                let max: usize = desc.slot_count();
                for slot in 0..max {
                    match (desclist.get(slot), datalist.get(slot)) {
                        (Some(Some(desc)), Some(Some(data))) => {
                            // log::warn!("Ready Slot {:?}", slot);
                            match &data.0 {
                                EVerticesBufferTmp::Instance(mem) => { instance_memory = Some(mem.clone()); },
                                EVerticesBufferTmp::Buffer(buf) => {
                                    let buff = RenderVertices {
                                        slot: slot as u32,
                                        buffer: buf.clone(),
                                        buffer_range: desc.range(),
                                        size_per_value: desc.0.stride()
                                    };
                                    values.push((desc.0.step_mode(), buff));
                                },
                            }
                        },
                        (None, None) => {
                            break;
                        },
                        _ => {
                            // log::error!("Not Ready Slot {:?}", (slot, max, idgeo, idmesh.0, desclist.get(slot)));
                            isready = false;
                            break;
                        }
                    }
                }
    
                if isready {
                    // if instance_memory.is_none() {
                    //     log::error!("Geo Ready {:?}", (idgeo, idmesh.0, desclist.get(0)));
                    // }
                    match (&indicesdesc.0, &indiceskey.0, &indices.0) {
                        (Some(desc), Some(key), Some(data)) => {
                            if &desc.buffer == key {
                                geometry.0 = Some(RenderGeometry::create(values, (indicesdesc.0.as_ref() , indices.0.as_ref()), instance_memory));
                                
                                *rendergeo = RenderGeometryEable(true);
                            } else {
                                if rendergeo.0 == true {
                                    *rendergeo = RenderGeometryEable(false);
                                }
                            }
                        },
                        (None, None, None) => {
                            geometry.0 = Some(RenderGeometry::create(values, (indicesdesc.0.as_ref() , indices.0.as_ref()), instance_memory));
                            *rendergeo = RenderGeometryEable(true);
                        },
                        _ => {
                            isready = false;
                            if rendergeo.0 == true {
                                *rendergeo = RenderGeometryEable(false);
                            }
                        }
                    }
                }
            }
        }
    });
    // log::error!("sys_vertex_buffer_slots_loaded {:?}", counter);
}

pub fn sys_geometry_enable(
    addeds: ComponentAdded<RenderGeometryComp>,
    changes: ComponentChanged<RenderGeometryComp>,
    geometries: Query<(&RenderGeometryComp, &MeshID)>,
    mut meshes: Query<&mut RenderGeometryEable>,
) {
    addeds.iter().chain(changes.iter()).for_each(|entity| {
        if let Ok((geometrycomp, idmesh)) = geometries.get(*entity) {
            if let Ok(mut state) = meshes.get_mut(idmesh.0) {
                *state = RenderGeometryEable(geometrycomp.is_some());
            }
        }
    });
    changes.iter().for_each(|entity| {
        if let Ok((geometrycomp, idmesh)) = geometries.get(*entity) {
            if let Ok(mut state) = meshes.get_mut(idmesh.0) {
                *state = RenderGeometryEable(geometrycomp.is_some());
            }
        }
    });
}