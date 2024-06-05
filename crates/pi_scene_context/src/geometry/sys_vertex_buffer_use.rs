
use std::ops::Range;

use pi_scene_shell::prelude::*;

use crate::geometry::geometry::RenderVerticesFrom;

use super::{
    vertex_buffer_useinfo::*,
    base::GeometryDesc,
    geometry::*
};

#[inline(never)]
fn _sys_vertex_buffer_slots_loaded(
    mut values: Vec<(wgpu::VertexStepMode, RenderVertices)>,
    mut instance_memory: Option<EVerteicesMemory>,
    res: &EVerticesBufferTmp,
    desc: &GeometryDesc,
    geometry: &mut RenderGeometryComp,
    rendergeo:&mut RenderGeometryEable,
    slot: u32,
    buffer_range: Option<Range<u64>>,
    buffdesc: &VertexBufferDesc,
    indicesdesc: Option<&IndicesBufferDesc>,
    indices: Option<&AssetResBufferIndices>
) -> Option<(Vec<(wgpu::VertexStepMode, RenderVertices)>, Option<EVerteicesMemory>)> {
    match res {
        EVerticesBufferTmp::Memory(mem) => { instance_memory = Some(mem.clone()); },
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
    items: Query<
        (
            Entity, 
            (&MeshID, &GeometryDesc, &IndicesBufferDescComp, &AssetResBufferIndicesComp, &AssetKeyBufferIndices)
            , &AssetDescVBSlots, &AssetResVBSlots, &LoadedKeyVBSlots
        ),
        Or<(
            Changed<AssetDescVBSlots>, Changed<AssetResVBSlots>,
            Changed<AssetResBufferIndicesComp>, Changed<IndicesBufferDescComp>
        )>
    >,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut meshes: Query<&mut RenderGeometryEable>,
) {
    items.iter().for_each(|(
        idgeo, 
        (idmesh, desc, indicesdesc, indices, indiceskey)
        , desclist, datalist, keyslist
    )| {
        if let (Ok(mut geometry), Ok(mut rendergeo)) = (geometries.get_mut(idgeo), meshes.get_mut(idmesh.0)) {

            let mut values = vec![];
            let mut instance_memory = None;
            let mut isready = true;

            for slot in 0..VB_SLOTS_COUNT {
                match (&desclist[slot], &keyslist[slot], &datalist[slot]) {
                    (Some(desc), Some(key), Some(data)) => {
                        match &data.0 {
                            EVerticesBufferTmp::Memory(mem) => { instance_memory = Some(mem.clone()); },
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
                    (None, None, None) => {
                        break;
                    },
                    _ => {
                        isready = false;
                        break;
                    }
                }
            }

            if isready {
                match (&indicesdesc.0, &indiceskey.0, &indices.0) {
                    (Some(desc), Some(key), Some(data)) => {
                        if &desc.buffer == key {
                            geometry.0 = Some(RenderGeometry::create(values, (indicesdesc.0.as_ref() , indices.0.as_ref()), instance_memory));
                            *rendergeo = RenderGeometryEable(true);
                        } else {
                            *rendergeo = RenderGeometryEable(false);
                        }
                    },
                    (None, None, None) => {
                        geometry.0 = Some(RenderGeometry::create(values, (indicesdesc.0.as_ref() , indices.0.as_ref()), instance_memory));
                        *rendergeo = RenderGeometryEable(true);
                    },
                    _ => {
                        isready = false;
                    }
                }
            }

        }

    });
}

pub fn sys_geometry_enable(
    geometries: Query<(&RenderGeometryComp, &MeshID), Changed<RenderGeometryComp>>,
    mut meshes: Query<&mut RenderGeometryEable>,
) {
    geometries.iter().for_each(|(geometrycomp, idmesh)| {
        if let Ok(mut state) = meshes.get_mut(idmesh.0) {
            *state = RenderGeometryEable(geometrycomp.is_some());
        }
    });
}