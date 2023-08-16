
use pi_engine_shell::prelude::*;
use pi_render::{renderer::{vertices::{RenderVertices}}};


use crate::{
    geometry::{
        geometry::RenderVerticesFrom,
    },
};

use super::{
    vertex_buffer_useinfo::*,
    base::GeometryDesc,
    geometry::{RenderGeometry, RenderGeometryEable},
    sys_vertex_buffer_loaded
};

// pub struct SysGeometryStatesInit;
// impl TSystemStageInfo for SysGeometryStatesInit {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysGeometryVBCommand::key(), SysMeshModifyCommand::key(),
//         ]
//     }
// }

// pub struct SysGeometryChangeInitSlot<D: TVertexBufferUseInfo + Component, D1: From<EVerticesBufferUsage> + Component>(PhantomData<(D, D1)>);
// #[setup]
// impl<D, D1> SysGeometryChangeInitSlot<D, D1>
// where
//     D: TVertexBufferUseInfo + Component,
//     D1: From<EVerticesBufferUsage> + Component,
// {
//     #[system]
//     fn sys(
//         mut items: Query<
//             GameObject,
//             (ObjectID, &MeshID, &GeometryDesc, &EInstanceCode),
//             Changed<GeometryDesc>,
//         >,
//         mut slot_cmd: Commands<D>,
//         mut res_cmd: Commands<D1>,
//         mut vb_data_map: ResMut<SingleVertexBufferDataMap>,
//         mut loader_01: ResMut<VertexBufferLoader<ObjectID, D1>>,
//         asset_mgr: Res<Share<AssetMgr<EVertexBufferRange>>>,
//         mut instance_source_record: ResMut<InstanceSourceRecord>,
//         mut geo_enable_cmd: Commands<RenderGeometryEable>,
//         mut ins_wm_cmd: Commands<InstanceBufferWorldMatrix>,
//         mut ins_color_cmd: Commands<InstanceBufferColor>,
//         mut ins_tilloff_cmd: Commands<InstanceBufferTillOff>,
//         mut instance_cmd: Commands<EInstanceCode>,
//     ) {
//         items.iter_mut().for_each(|(
//             id_geo, id_mesh,
//             statistics, mut instance_code
//         )| {
//             if statistics.slot_count() >= D::ASK_SLOT_COUNT as usize {
//                 // log::info!(">>>>>>>>>>>>>>>>>>>>>> SysGeometryChangeIntSlot: Slot {}", D::ASK_SLOT_COUNT);
//                 let slot_index = D::ASK_SLOT_COUNT as usize - 1;
//                 let desc = statistics.get_desc(slot_index);
//                 let instance_kind = desc.instance_kind();
//                 match instance_kind {
//                     EInstanceKind::None => {
//                         if let Some(data) = asset_mgr.get(&desc.key) {
//                             res_cmd.insert(id_geo, D1::from(EVerticesBufferUsage::Other(data)));
//                         } else {
//                             loader_01.request(id_geo, &desc.key, None, &mut vb_data_map);
//                         }
//                         slot_cmd.insert(id_geo, D::from(desc));
//                     },
//                     _ => {
//                         geo_enable_cmd.insert(id_mesh.0.clone(), RenderGeometryEable(false));

//                         let buff_id = instance_source_record.id().to_string();
//                         slot_cmd.insert(id_geo.clone(), D::from(desc.clone()));

//                         match instance_kind {
//                             EInstanceKind::WorldMatrix => {
//                                 let buff = InstanceBufferWorldMatrix { slot: slot_index, id: String::from(buff_id + "WorldMatrix"), index: 0 };
//                                 ins_wm_cmd.insert(id_geo.clone(), buff);
//                                 instance_code.0 = instance_code.0 | EInstanceCode::BASE;
//                             },
//                             EInstanceKind::Color => {
//                                 let buff = InstanceBufferColor { slot: slot_index, id: String::from(buff_id + "Color"), index: 0 };
//                                 ins_color_cmd.insert(id_geo.clone(), buff);
//                                 // log::debug!("Instance Color");
//                                 instance_code.0 = instance_code.0 | EInstanceCode::COLOR;
//                             },
//                             EInstanceKind::TillOffset => {
//                                 let buff = InstanceBufferTillOff { slot: slot_index, id: String::from(buff_id + "TillOff"), index: 0 };
//                                 ins_tilloff_cmd.insert(id_geo.clone(), buff);
//                                 // log::debug!("Instance TillOffset");
//                                 instance_code.0 = instance_code.0 | EInstanceCode::TILL_OFF_1;
//                             },
//                             _ => { },
//                         }
//                     },
//                 };
//             }
//         });
//     }
// }


// pub type SysGeometryChangeSlot01 = SysGeometryChangeInitSlot<AssetDescVBSlot01, AssetResVBSlot01>;
// pub type SysGeometryChangeSlot02 = SysGeometryChangeInitSlot<AssetDescVBSlot02, AssetResVBSlot02>;
// pub type SysGeometryChangeSlot03 = SysGeometryChangeInitSlot<AssetDescVBSlot03, AssetResVBSlot03>;
// pub type SysGeometryChangeSlot04 = SysGeometryChangeInitSlot<AssetDescVBSlot04, AssetResVBSlot04>;
// pub type SysGeometryChangeSlot05 = SysGeometryChangeInitSlot<AssetDescVBSlot05, AssetResVBSlot05>;
// pub type SysGeometryChangeSlot06 = SysGeometryChangeInitSlot<AssetDescVBSlot06, AssetResVBSlot06>;
// pub type SysGeometryChangeSlot07 = SysGeometryChangeInitSlot<AssetDescVBSlot07, AssetResVBSlot07>;
// pub type SysGeometryChangeSlot08 = SysGeometryChangeInitSlot<AssetDescVBSlot08, AssetResVBSlot08>;
// pub type SysGeometryChangeSlot09 = SysGeometryChangeInitSlot<AssetDescVBSlot09, AssetResVBSlot09>;
// pub type SysGeometryChangeSlot10 = SysGeometryChangeInitSlot<AssetDescVBSlot10, AssetResVBSlot10>;
// pub type SysGeometryChangeSlot11 = SysGeometryChangeInitSlot<AssetDescVBSlot11, AssetResVBSlot11>;
// pub type SysGeometryChangeSlot12 = SysGeometryChangeInitSlot<AssetDescVBSlot12, AssetResVBSlot12>;
// pub type SysGeometryChangeSlot13 = SysGeometryChangeInitSlot<AssetDescVBSlot13, AssetResVBSlot13>;
// pub type SysGeometryChangeSlot14 = SysGeometryChangeInitSlot<AssetDescVBSlot14, AssetResVBSlot14>;
// pub type SysGeometryChangeSlot15 = SysGeometryChangeInitSlot<AssetDescVBSlot15, AssetResVBSlot15>;
// pub type SysGeometryChangeSlot16 = SysGeometryChangeInitSlot<AssetDescVBSlot16, AssetResVBSlot16>;

// pub struct SysRenderGeometryInit;
// impl TSystemStageInfo for SysRenderGeometryInit {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysVertexBufferLoad::key()
//         ]
//     }
// }

// pub struct SysGeometryVBUpdateSlot01;
// #[setup]
// impl SysGeometryVBUpdateSlot01
// {
//     #[system]
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
        mut commands: Commands,
    ) {
        // log::debug!("SysGeometryVBLoaded 1: ");
        items.iter().for_each(|(
            id_geo
            , id_mesh
            , desc
            , key1, res1
            , indicesdesc , indices
        )| {
            if desc.slot_count() == 1 {
                let id_mesh = id_mesh.0.clone();
                // log::debug!("SysGeometryVBLoaded 1: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                ];
                if let Some(mut cmd) = commands.get_entity(id_geo) {
                    cmd.insert(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Some(mut cmd) = commands.get_entity(id_mesh) {
                    cmd.insert(RenderGeometryEable(true));
                }
            }
        });
    }
// }

// pub struct SysGeometryVBUpdateSlot02;
// #[setup]
// impl SysGeometryVBUpdateSlot02
// {
//     #[system]
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
        mut commands: Commands,
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
            if desc.slot_count() == 2 {
                let id_mesh = id_mesh.0.clone();
                // log::debug!("SysGeometryVBUpdateSlot2: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                ];
                if let Some(mut cmd) = commands.get_entity(id_geo) {
                    cmd.insert(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Some(mut cmd) = commands.get_entity(id_mesh) {
                    cmd.insert(RenderGeometryEable(true));
                }
            }
        });
    }
// }

// pub struct SysGeometryVBUpdateSlot03;
// #[setup]
// impl SysGeometryVBUpdateSlot03
// {
//     #[system]
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
        mut commands: Commands,
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
            if desc.slot_count() == 3 {
                let id_mesh = id_mesh.0.clone();
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                ];
                if let Some(mut cmd) = commands.get_entity(id_geo) {
                    cmd.insert(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Some(mut cmd) = commands.get_entity(id_mesh) {
                    cmd.insert(RenderGeometryEable(true));
                }
            }
        });
    }
// }

// pub struct SysGeometryVBUpdateSlot04;
// #[setup]
// impl SysGeometryVBUpdateSlot04
// {
//     #[system]
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
        mut commands: Commands,
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
            // log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 4 {
                let id_mesh = id_mesh.0.clone();
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                ];
                if let Some(mut cmd) = commands.get_entity(id_geo) {
                    cmd.insert(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Some(mut cmd) = commands.get_entity(id_mesh) {
                    cmd.insert(RenderGeometryEable(true));
                }
            }
        });
    }
// }

// pub struct SysGeometryVBUpdateSlot05;
// #[setup]
// impl SysGeometryVBUpdateSlot05
// {
//     #[system]
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
        mut commands: Commands,
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
            // log::debug!(" > {}", desc.slot_count());
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
                if let Some(mut cmd) = commands.get_entity(id_geo) {
                    cmd.insert(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Some(mut cmd) = commands.get_entity(id_mesh) {
                    cmd.insert(RenderGeometryEable(true));
                }
            }
        });
    }
// }

// pub struct SysGeometryVBUpdateSlot06;
// #[setup]
// impl SysGeometryVBUpdateSlot06
// {
//     #[system]
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
        mut commands: Commands,
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
            // log::debug!(" > {}", desc.slot_count());
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
                if let Some(mut cmd) = commands.get_entity(id_geo) {
                    cmd.insert(RenderGeometry::create(values, (indicesdesc , indices)));
                }
                if let Some(mut cmd) = commands.get_entity(id_mesh) {
                    cmd.insert(RenderGeometryEable(true));
                }
            }
        });
    }
// }

pub fn sys_geometry_enable(
    geometries: Query<(&RenderGeometry, &MeshID), Changed<RenderGeometry>>,
    mut meshes: Query<&mut RenderGeometryEable>,
) {
    geometries.iter().for_each(|(_, idmesh)| {
        if let Ok(mut state) = meshes.get_mut(idmesh.0) {
            *state = RenderGeometryEable(true);
        }
    });
}