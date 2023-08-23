
use pi_engine_shell::prelude::*;
use pi_render::renderer::vertices::RenderVertices;


use crate::geometry::geometry::RenderVerticesFrom;

use super::{
    vertex_buffer_useinfo::*,
    base::GeometryDesc,
    geometry::*
};

    pub fn sys_vertex_buffer_loaded_01(
        items: Query<
            (   
                ObjectID
                , &MeshID
                , &GeometryDesc
                , &AssetDescVBSlot01, &AssetResVBSlot01
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geometries: Query<&mut RenderGeometryComp>,
        mut meshes: Query<&mut RenderGeometryEable>,
    ) {
        // log::debug!("SysGeometryVBLoaded 1: ");
        items.iter().for_each(|(
            id_geo
            , id_mesh
            , desc
            , key1, res1
            , indicesdesc , indices
        )| {
            // log::warn!(" > {}", desc.slot_count());
            if desc.slot_count() == 1 {
                let id_mesh = id_mesh.0.clone();
                // log::debug!("SysGeometryVBLoaded 1: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                ];
                if let Ok(mut geometry) = geometries.get_mut(id_geo) {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Ok(mut item) = meshes.get_mut(id_mesh) {
                    *item = RenderGeometryEable(true);
                }
            }
        });
    }

    pub fn sys_vertex_buffer_loaded_02(
        items: Query<
            (   
                ObjectID
                , &MeshID
                , &GeometryDesc
                , &AssetDescVBSlot01, &AssetResVBSlot01
                , &AssetDescVBSlot02, &AssetResVBSlot02
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geometries: Query<&mut RenderGeometryComp>,
        mut meshes: Query<&mut RenderGeometryEable>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot2: ");
        items.iter().for_each(|(
            id_geo
            , id_mesh
            , desc
            , key1, res1
            , key2, res2
            , indicesdesc , indices
        )| {
            // log::warn!(" > {}", desc.slot_count());
            if desc.slot_count() == 2 {
                let id_mesh = id_mesh.0.clone();
                // log::debug!("SysGeometryVBUpdateSlot2: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                ];
                if let Ok(mut geometry) = geometries.get_mut(id_geo) {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Ok(mut item) = meshes.get_mut(id_mesh) {
                    *item = RenderGeometryEable(true);
                }
            }
        });
    }

    pub fn sys_vertex_buffer_loaded_03(
        items: Query<
            (   
                ObjectID
                , &MeshID
                , &GeometryDesc
                , &AssetDescVBSlot01, &AssetResVBSlot01
                , &AssetDescVBSlot02, &AssetResVBSlot02
                , &AssetDescVBSlot03, &AssetResVBSlot03
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geometries: Query<&mut RenderGeometryComp>,
        mut meshes: Query<&mut RenderGeometryEable>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot3: ");
        items.iter().for_each(|(
            id_geo
            , id_mesh
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , indicesdesc , indices
        )| {
            // log::warn!(" > {}", desc.slot_count());
            if desc.slot_count() == 3 {
                let id_mesh = id_mesh.0.clone();
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                ];
                if let Ok(mut geometry) = geometries.get_mut(id_geo) {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Ok(mut item) = meshes.get_mut(id_mesh) {
                    *item = RenderGeometryEable(true);
                }
            }
        });
    }

    pub fn sys_vertex_buffer_loaded_04(
        items: Query<
            (   
                ObjectID
                , &MeshID
                , &GeometryDesc
                , &AssetDescVBSlot01, &AssetResVBSlot01
                , &AssetDescVBSlot02, &AssetResVBSlot02
                , &AssetDescVBSlot03, &AssetResVBSlot03
                , &AssetDescVBSlot04, &AssetResVBSlot04
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
                Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geometries: Query<&mut RenderGeometryComp>,
        mut meshes: Query<&mut RenderGeometryEable>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot4: ");
        items.iter().for_each(|(
            id_geo
            , id_mesh
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , indicesdesc , indices
        )| {
            // log::warn!(" > {}", desc.slot_count());
            if desc.slot_count() == 4 {
                let id_mesh = id_mesh.0.clone();
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                ];
                // log::warn!("RenderGeometry: 0");
                if let Ok(mut geometry) = geometries.get_mut(id_geo) {
                    // log::warn!("RenderGeometry: 1");
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Ok(mut item) = meshes.get_mut(id_mesh) {
                    *item = RenderGeometryEable(true);
                }
            }
        });
    }

    pub fn sys_vertex_buffer_loaded_05(
        items: Query<
            (   
                ObjectID
                , &MeshID
                , &GeometryDesc
                , &AssetDescVBSlot01, &AssetResVBSlot01
                , &AssetDescVBSlot02, &AssetResVBSlot02
                , &AssetDescVBSlot03, &AssetResVBSlot03
                , &AssetDescVBSlot04, &AssetResVBSlot04
                , &AssetDescVBSlot05, &AssetResVBSlot05
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
                Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
                Changed<AssetDescVBSlot05>, Changed<AssetResVBSlot05>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geometries: Query<&mut RenderGeometryComp>,
        mut meshes: Query<&mut RenderGeometryEable>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot5: ");
        items.iter().for_each(|(
            id_geo
            , id_mesh
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , key5, res5
            , indicesdesc , indices
        )| {
            // log::warn!(" > {}", desc.slot_count());
            if desc.slot_count() == 5 {
                let id_mesh = id_mesh.0.clone();
                // log::debug!("SysGeometryVBUpdateSlot5: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                    (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
                ];
                if let Ok(mut geometry) = geometries.get_mut(id_geo) {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Ok(mut item) = meshes.get_mut(id_mesh) {
                    *item = RenderGeometryEable(true);
                }
            }
        });
    }

    pub fn sys_vertex_buffer_loaded_06(
        items: Query<
            (   
                ObjectID
                , &MeshID
                , &GeometryDesc
                , (&AssetDescVBSlot01, &AssetResVBSlot01
                , &AssetDescVBSlot02, &AssetResVBSlot02
                , &AssetDescVBSlot03, &AssetResVBSlot03
                , &AssetDescVBSlot04, &AssetResVBSlot04
                , &AssetDescVBSlot05, &AssetResVBSlot05
                , &AssetDescVBSlot06, &AssetResVBSlot06)
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                
                Changed<AssetResVBSlot01>,
                Changed<AssetResVBSlot02>,
                Changed<AssetResVBSlot03>,
                Changed<AssetResVBSlot04>,
                Changed<AssetResVBSlot05>,
                Changed<AssetResVBSlot06>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geometries: Query<&mut RenderGeometryComp>,
        mut meshes: Query<&mut RenderGeometryEable>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot6: ");
        items.iter().for_each(|(
            
            id_geo
            , id_mesh
            , desc
            , (key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , key5, res5
            , key6, res6)
            , indicesdesc , indices
        )| {
            // log::warn!(" > {}", desc.slot_count());
            if desc.slot_count() == 6 {
                let id_mesh = id_mesh.0.clone();
                // log::debug!("SysGeometryVBUpdateSlot6: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                    (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
                    (key6.desc().step_mode(), RenderVertices::create(key6, res6)),
                ];
                if let Ok(mut geometry) = geometries.get_mut(id_geo) {
                    geometry.0 = Some(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Ok(mut item) = meshes.get_mut(id_mesh) {
                    *item = RenderGeometryEable(true);
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