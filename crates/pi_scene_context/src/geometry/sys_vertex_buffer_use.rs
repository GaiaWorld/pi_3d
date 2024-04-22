
use std::ops::Range;

use pi_scene_shell::prelude::*;

use crate::geometry::geometry::RenderVerticesFrom;

use super::{
    vertex_buffer_useinfo::*,
    base::GeometryDesc,
    geometry::*
};

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
            (&MeshID, &GeometryDesc, Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>)
            , (Option<&AssetDescVBSlot01>, Option<&AssetResVBSlot01>)
            , (Option<&AssetDescVBSlot02>, Option<&AssetResVBSlot02>)
            , (Option<&AssetDescVBSlot03>, Option<&AssetResVBSlot03>)
            , (Option<&AssetDescVBSlot04>, Option<&AssetResVBSlot04>)
            , (Option<&AssetDescVBSlot05>, Option<&AssetResVBSlot05>)
            , (Option<&AssetDescVBSlot06>, Option<&AssetResVBSlot06>)
            , (Option<&AssetDescVBSlot07>, Option<&AssetResVBSlot07>)
            , (Option<&AssetDescVBSlot08>, Option<&AssetResVBSlot08>)
            // , (Option<&AssetDescVBSlot09>, Option<&AssetResVBSlot09>)
            // , (Option<&AssetDescVBSlot10>, Option<&AssetResVBSlot10>)
            // , (Option<&AssetDescVBSlot11>, Option<&AssetResVBSlot11>)
            // , (Option<&AssetDescVBSlot12>, Option<&AssetResVBSlot12>)
        ),
        Or<(
            Changed<AssetResVBSlot01>, Changed<AssetResVBSlot02>, Changed<AssetResVBSlot03>, Changed<AssetResVBSlot04>,
            Changed<AssetResVBSlot05>, Changed<AssetResVBSlot06>, Changed<AssetResVBSlot07>, Changed<AssetResVBSlot08>,
            // Changed<AssetResVBSlot09>, Changed<AssetResVBSlot10>, Changed<AssetResVBSlot11>, Changed<AssetResVBSlot12>,
            Changed<AssetResBufferIndices>, Changed<IndicesBufferDesc>
        )>
    >,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut meshes: Query<&mut RenderGeometryEable>,
) {
    items.iter().for_each(|(
        idgeo, 
        (idmesh, desc, indicesdesc, indices)
        , (key01, res01)
        , (key02, res02)
        , (key03, res03)
        , (key04, res04)
        , (key05, res05)
        , (key06, res06)
        , (key07, res07)
        , (key08, res08)
        // , (key09, res09)
        // , (key10, res10)
        // , (key11, res11)
        // , (key12, res12)
    )| {
        if let (Ok(mut geometry), Ok(mut rendergeo)) = (geometries.get_mut(idgeo), meshes.get_mut(idmesh.0)) {

            let mut values = vec![];
            let mut instance_memory = None;

            if let (Some(key), Some(res)) = (key01, res01) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 0, key.range(), key.desc(), indicesdesc, indices) {
                    values = a; instance_memory = b;
                } else {
                    return;
                }
                // match &res.0 {
                //     EVerticesBufferTmp::Memory(mem) => { instance_memory = Some(mem.clone()); },
                //     EVerticesBufferTmp::Buffer(buf) => {
                //         values.push((key.desc().step_mode(), RenderVertices::create(key, buf.clone())));
                //     },
                // }
                // if desc.slot_count() == 1 {
                //     geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices), instance_memory));
                //     *rendergeo = RenderGeometryEable(true);
                //     return;
                // }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key02, res02) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 1, key.range(), key.desc(), indicesdesc, indices) {
                    values = a; instance_memory = b;
                } else { return; }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key03, res03) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 2, key.range(), key.desc(), indicesdesc, indices) {
                    values = a; instance_memory = b;
                } else { return; }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key04, res04) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 3, key.range(), key.desc(), indicesdesc, indices) {
                    values = a; instance_memory = b;
                } else { return; }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key05, res05) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 4, key.range(), key.desc(), indicesdesc, indices) {
                    values = a; instance_memory = b;
                } else { return; }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key06, res06) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 5, key.range(), key.desc(), indicesdesc, indices) {
                    values = a; instance_memory = b;
                } else { return; }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key07, res07) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 6, key.range(), key.desc(), indicesdesc, indices) {
                    values = a; instance_memory = b;
                } else { return; }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key08, res08) {
                if let Some((a, b)) = _sys_vertex_buffer_slots_loaded(values, instance_memory, &res.0, desc, &mut geometry, &mut rendergeo, 7, key.range(), key.desc(), indicesdesc, indices) {
                    // values = a; instance_memory = b;
                } else { return; }
            } else { return; }
            
            // if let (Some(key), Some(res)) = (key09, res09) {
            //     match &res.0 {
            //         EVerticesBufferTmp::Memory(mem) => { instance_memory = Some(mem.clone()); },
            //         EVerticesBufferTmp::Buffer(buf) => {
            //             values.push((key.desc().step_mode(), RenderVertices::create(key, buf.clone())));
            //         },
            //     }
            //     if desc.slot_count() == 9 {
            //         geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices), instance_memory));
            //         *rendergeo = RenderGeometryEable(true);
            //         return;
            //     }
            // } else { return; }

            // if let (Some(key), Some(res)) = (key10, res10) {
            //     match &res.0 {
            //         EVerticesBufferTmp::Memory(mem) => { instance_memory = Some(mem.clone()); },
            //         EVerticesBufferTmp::Buffer(buf) => {
            //             values.push((key.desc().step_mode(), RenderVertices::create(key, buf.clone())));
            //         },
            //     }
            //     if desc.slot_count() == 10 {
            //         geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices), instance_memory));
            //         *rendergeo = RenderGeometryEable(true);
            //         return;
            //     }
            // } else { return; }

            // if let (Some(key), Some(res)) = (key11, res11) {
            //     match &res.0 {
            //         EVerticesBufferTmp::Memory(mem) => { instance_memory = Some(mem.clone()); },
            //         EVerticesBufferTmp::Buffer(buf) => {
            //             values.push((key.desc().step_mode(), RenderVertices::create(key, buf.clone())));
            //         },
            //     }
            //     if desc.slot_count() == 11 {
            //         geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices), instance_memory));
            //         *rendergeo = RenderGeometryEable(true);
            //         return;
            //     }
            // } else { return; }

            // if let (Some(key), Some(res)) = (key12, res12) {
            //     match &res.0 {
            //         EVerticesBufferTmp::Memory(mem) => { instance_memory = Some(mem.clone()); },
            //         EVerticesBufferTmp::Buffer(buf) => {
            //             values.push((key.desc().step_mode(), RenderVertices::create(key, buf.clone())));
            //         },
            //     }
            //     if desc.slot_count() == 12 {
            //         geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices), instance_memory));
            //         *rendergeo = RenderGeometryEable(true);
            //         return;
            //     }
            // } else { return; }
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