
use pi_scene_shell::prelude::*;

use crate::geometry::geometry::RenderVerticesFrom;

use super::{
    vertex_buffer_useinfo::*,
    base::GeometryDesc,
    geometry::*
};

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
            , (Option<&AssetDescVBSlot09>, Option<&AssetResVBSlot09>)
            , (Option<&AssetDescVBSlot10>, Option<&AssetResVBSlot10>)
            , (Option<&AssetDescVBSlot11>, Option<&AssetResVBSlot11>)
            , (Option<&AssetDescVBSlot12>, Option<&AssetResVBSlot12>)
        ),
        Or<(
            Changed<AssetResVBSlot01>, Changed<AssetResVBSlot02>, Changed<AssetResVBSlot03>, Changed<AssetResVBSlot04>,
            Changed<AssetResVBSlot05>, Changed<AssetResVBSlot06>, Changed<AssetResVBSlot07>, Changed<AssetResVBSlot08>,
            Changed<AssetResVBSlot09>, Changed<AssetResVBSlot10>, Changed<AssetResVBSlot11>, Changed<AssetResVBSlot12>,
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
        , (key09, res09)
        , (key10, res10)
        , (key11, res11)
        , (key12, res12)
    )| {
        if let (Ok(mut geometry), Ok(mut rendergeo)) = (geometries.get_mut(idgeo), meshes.get_mut(idmesh.0)) {

            let mut values = vec![];

            if let (Some(key), Some(res)) = (key01, res01) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 1 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key02, res02) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 2 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key03, res03) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 3 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key04, res04) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 4 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key05, res05) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 5 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key06, res06) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 6 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key07, res07) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 7 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key08, res08) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 8 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
            
            if let (Some(key), Some(res)) = (key09, res09) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 9 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }

            if let (Some(key), Some(res)) = (key10, res10) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 10 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }

            if let (Some(key), Some(res)) = (key11, res11) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 11 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }

            if let (Some(key), Some(res)) = (key12, res12) {
                values.push((key.desc().step_mode(), RenderVertices::create(key, res)));
                if desc.slot_count() == 12 {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                    *rendergeo = RenderGeometryEable(true);
                    return;
                }
            } else { return; }
        }

    });
}

    // pub fn sys_vertex_buffer_loaded_01(
    //     items: Query<
    //         (   
    //             ObjectID
    //             , &MeshID
    //             , &GeometryDesc
    //             , &AssetDescVBSlot01, &AssetResVBSlot01
    //             , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    //         ),
    //         Or<(
                
    //             Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
    //             Changed<AssetResBufferIndices>
    //         )>
    //     >,
    //     mut geometries: Query<&mut RenderGeometryComp>,
    //     mut meshes: Query<&mut RenderGeometryEable>,
    // ) {
    //     // log::debug!("SysGeometryVBLoaded 1: ");
    //     items.iter().for_each(|(
    //         id_geo
    //         , id_mesh
    //         , desc
    //         , key1, res1
    //         , indicesdesc , indices
    //     )| {
    //         // log::warn!(" > {}", desc.slot_count());
    //         if desc.slot_count() == 1 {
    //             let id_mesh = id_mesh.0.clone();
    //             // log::debug!("SysGeometryVBLoaded 1: 0");
    //             let values = vec![
    //                 (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
    //             ];
    //             if let Ok(mut geometry) = geometries.get_mut(id_geo) {
    //                 geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
    //             }
    //             if let Ok(mut item) = meshes.get_mut(id_mesh) {
    //                 *item = RenderGeometryEable(true);
    //             }
    //         }
    //     });
    // }

    // pub fn sys_vertex_buffer_loaded_02(
    //     items: Query<
    //         (   
    //             ObjectID
    //             , &MeshID
    //             , &GeometryDesc
    //             , &AssetDescVBSlot01, &AssetResVBSlot01
    //             , &AssetDescVBSlot02, &AssetResVBSlot02
    //             , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    //         ),
    //         Or<(
    //             Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
    //             Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
    //             Changed<AssetResBufferIndices>
    //         )>
    //     >,
    //     mut geometries: Query<&mut RenderGeometryComp>,
    //     mut meshes: Query<&mut RenderGeometryEable>,
    // ) {
    //     // log::debug!("SysGeometryVBUpdateSlot2: ");
    //     items.iter().for_each(|(
    //         id_geo
    //         , id_mesh
    //         , desc
    //         , key1, res1
    //         , key2, res2
    //         , indicesdesc , indices
    //     )| {
    //         // log::warn!(" > {}", desc.slot_count());
    //         if desc.slot_count() == 2 {
    //             let id_mesh = id_mesh.0.clone();
    //             // log::debug!("SysGeometryVBUpdateSlot2: 0");
    //             let values = vec![
    //                 (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
    //                 (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
    //             ];
    //             if let Ok(mut geometry) = geometries.get_mut(id_geo) {
    //                 geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
    //             }
    //             if let Ok(mut item) = meshes.get_mut(id_mesh) {
    //                 *item = RenderGeometryEable(true);
    //             }
    //         }
    //     });
    // }

    // pub fn sys_vertex_buffer_loaded_03(
    //     items: Query<
    //         (   
    //             ObjectID
    //             , &MeshID
    //             , &GeometryDesc
    //             , &AssetDescVBSlot01, &AssetResVBSlot01
    //             , &AssetDescVBSlot02, &AssetResVBSlot02
    //             , &AssetDescVBSlot03, &AssetResVBSlot03
    //             , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    //         ),
    //         Or<(
                
    //             Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
    //             Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
    //             Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
    //             Changed<AssetResBufferIndices>
    //         )>
    //     >,
    //     mut geometries: Query<&mut RenderGeometryComp>,
    //     mut meshes: Query<&mut RenderGeometryEable>,
    // ) {
    //     // log::debug!("SysGeometryVBUpdateSlot3: ");
    //     items.iter().for_each(|(
    //         id_geo
    //         , id_mesh
    //         , desc
    //         , key1, res1
    //         , key2, res2
    //         , key3, res3
    //         , indicesdesc , indices
    //     )| {
    //         // log::warn!(" > {}", desc.slot_count());
    //         if desc.slot_count() == 3 {
    //             let id_mesh = id_mesh.0.clone();
    //             let values = vec![
    //                 (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
    //                 (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
    //                 (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
    //             ];
    //             if let Ok(mut geometry) = geometries.get_mut(id_geo) {
    //                 geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
    //             }
    //             if let Ok(mut item) = meshes.get_mut(id_mesh) {
    //                 *item = RenderGeometryEable(true);
    //             }
    //         }
    //     });
    // }

    // pub fn sys_vertex_buffer_loaded_04(
    //     items: Query<
    //         (   
    //             ObjectID
    //             , &MeshID
    //             , &GeometryDesc
    //             , &AssetDescVBSlot01, &AssetResVBSlot01
    //             , &AssetDescVBSlot02, &AssetResVBSlot02
    //             , &AssetDescVBSlot03, &AssetResVBSlot03
    //             , &AssetDescVBSlot04, &AssetResVBSlot04
    //             , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    //         ),
    //         Or<(
                
    //             Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
    //             Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
    //             Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
    //             Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
    //             Changed<AssetResBufferIndices>
    //         )>
    //     >,
    //     mut geometries: Query<&mut RenderGeometryComp>,
    //     mut meshes: Query<&mut RenderGeometryEable>,
    // ) {
    //     // log::debug!("SysGeometryVBUpdateSlot4: ");
    //     items.iter().for_each(|(
    //         id_geo
    //         , id_mesh
    //         , desc
    //         , key1, res1
    //         , key2, res2
    //         , key3, res3
    //         , key4, res4
    //         , indicesdesc , indices
    //     )| {
    //         // log::warn!(" > {}", desc.slot_count());
    //         if desc.slot_count() == 4 {
    //             let id_mesh = id_mesh.0.clone();
    //             let values = vec![
    //                 (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
    //                 (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
    //                 (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
    //                 (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
    //             ];
    //             // log::warn!("RenderGeometry: 0");
    //             if let Ok(mut geometry) = geometries.get_mut(id_geo) {
    //                 // log::warn!("RenderGeometry: 1");
    //                 geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
    //             }
    //             if let Ok(mut item) = meshes.get_mut(id_mesh) {
    //                 *item = RenderGeometryEable(true);
    //             }
    //         }
    //     });
    // }

    // pub fn sys_vertex_buffer_loaded_05(
    //     items: Query<
    //         (   
    //             ObjectID
    //             , &MeshID
    //             , &GeometryDesc
    //             , &AssetDescVBSlot01, &AssetResVBSlot01
    //             , &AssetDescVBSlot02, &AssetResVBSlot02
    //             , &AssetDescVBSlot03, &AssetResVBSlot03
    //             , &AssetDescVBSlot04, &AssetResVBSlot04
    //             , &AssetDescVBSlot05, &AssetResVBSlot05
    //             , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    //         ),
    //         Or<(
                
    //             Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
    //             Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
    //             Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
    //             Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
    //             Changed<AssetDescVBSlot05>, Changed<AssetResVBSlot05>,
    //             Changed<AssetResBufferIndices>
    //         )>
    //     >,
    //     mut geometries: Query<&mut RenderGeometryComp>,
    //     mut meshes: Query<&mut RenderGeometryEable>,
    // ) {
    //     // log::debug!("SysGeometryVBUpdateSlot5: ");
    //     items.iter().for_each(|(
    //         id_geo
    //         , id_mesh
    //         , desc
    //         , key1, res1
    //         , key2, res2
    //         , key3, res3
    //         , key4, res4
    //         , key5, res5
    //         , indicesdesc , indices
    //     )| {
    //         // log::warn!(" > {}", desc.slot_count());
    //         if desc.slot_count() == 5 {
    //             let id_mesh = id_mesh.0.clone();
    //             // log::debug!("SysGeometryVBUpdateSlot5: 0");
    //             let values = vec![
    //                 (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
    //                 (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
    //                 (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
    //                 (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
    //                 (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
    //             ];
    //             if let Ok(mut geometry) = geometries.get_mut(id_geo) {
    //                 geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
    //             }
    //             if let Ok(mut item) = meshes.get_mut(id_mesh) {
    //                 *item = RenderGeometryEable(true);
    //             }
    //         }
    //     });
    // }

    // pub fn sys_vertex_buffer_loaded_06(
    //     items: Query<
    //         (   
    //             ObjectID
    //             , &MeshID
    //             , &GeometryDesc
    //             , (&AssetDescVBSlot01, &AssetResVBSlot01
    //             , &AssetDescVBSlot02, &AssetResVBSlot02
    //             , &AssetDescVBSlot03, &AssetResVBSlot03
    //             , &AssetDescVBSlot04, &AssetResVBSlot04
    //             , &AssetDescVBSlot05, &AssetResVBSlot05
    //             , &AssetDescVBSlot06, &AssetResVBSlot06)
    //             , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
    //         ),
    //         Or<(
                
    //             Changed<AssetResVBSlot01>,
    //             Changed<AssetResVBSlot02>,
    //             Changed<AssetResVBSlot03>,
    //             Changed<AssetResVBSlot04>,
    //             Changed<AssetResVBSlot05>,
    //             Changed<AssetResVBSlot06>,
    //             Changed<AssetResBufferIndices>
    //         )>
    //     >,
    //     mut geometries: Query<&mut RenderGeometryComp>,
    //     mut meshes: Query<&mut RenderGeometryEable>,
    // ) {
    //     // log::debug!("SysGeometryVBUpdateSlot6: ");
    //     items.iter().for_each(|(
            
    //         id_geo
    //         , id_mesh
    //         , desc
    //         , (key1, res1
    //         , key2, res2
    //         , key3, res3
    //         , key4, res4
    //         , key5, res5
    //         , key6, res6)
    //         , indicesdesc , indices
    //     )| {
    //         // log::warn!(" > {}", desc.slot_count());
    //         if desc.slot_count() == 6 {
    //             let id_mesh = id_mesh.0.clone();
    //             // log::debug!("SysGeometryVBUpdateSlot6: 0");
    //             let values = vec![
    //                 (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
    //                 (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
    //                 (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
    //                 (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
    //                 (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
    //                 (key6.desc().step_mode(), RenderVertices::create(key6, res6)),
    //             ];
    //             if let Ok(mut geometry) = geometries.get_mut(id_geo) {
    //                 geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
    //             }
    //             if let Ok(mut item) = meshes.get_mut(id_mesh) {
    //                 *item = RenderGeometryEable(true);
    //             }
    //         }
    //     });
    // }

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