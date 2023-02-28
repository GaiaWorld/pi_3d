use std::marker::PhantomData;

use pi_ecs::{prelude::{Component, Query, Setup, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::{render_3d::shader::instance_code::EInstanceCode, renderer::{instance::EInstanceKind, indices::{IndicesBufferDesc, AssetResBufferIndices}, vertices::RenderVertices}};

use crate::{
    geometry::{
        geometry::RenderVerticesFrom,
        instance::{InstanceList}
    },
    meshes::command::SysMeshModifyCommand
};

use super::{
    vertex_buffer_useinfo::*,
    GeometryDesc,
    geometry::{RenderGeometry, RenderGeometryEable},
    instance::{instance_world_matrix::InstancedBufferWorldMatrix, instance_color::InstancedBufferColor, instance_tilloff::InstancedBufferTillOff},
    SysGeometryVBCommand,
    SysVertexBufferLoad
};

pub struct SysGeometryStatesInit;
impl TSystemStageInfo for SysGeometryStatesInit {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysGeometryVBCommand::key(), SysMeshModifyCommand::key(),
        ]
    }
}

pub struct SysGeometryChangeInitSlot<D: TVertexBufferUseInfo + Component, D1: AsKeyVertexBuffer + Component>(PhantomData<(D, D1)>);
#[setup]
impl<D, D1> SysGeometryChangeInitSlot<D, D1>
where
    D: TVertexBufferUseInfo + Component,
    D1: AsKeyVertexBuffer + Component,
{
    #[system]
    fn sys(
        mut items: Query<
            GameObject,
            (ObjectID, &GeometryDesc, &mut EInstanceCode, &InstanceList),
            Changed<GeometryDesc>,
        >,
        mut slot_cmd: Commands<GameObject, D>,
        mut slotkey_cmd: Commands<GameObject, D1>,
        mut geo_enable_cmd: Commands<GameObject, RenderGeometryEable>,
        mut ins_wm_cmd: Commands<GameObject, InstancedBufferWorldMatrix>,
        mut ins_color_cmd: Commands<GameObject, InstancedBufferColor>,
        mut ins_tilloff_cmd: Commands<GameObject, InstancedBufferTillOff>,
    ) {
        items.iter_mut().for_each(|(
            obj, 
            statistics, mut instance_code, ins_list
        )| {
            if statistics.slot_count() >= D::ASK_SLOT_COUNT as usize {
                // log::info!(">>>>>>>>>>>>>>>>>>>>>> SysGeometryChangeIntSlot: Slot {}", D::ASK_SLOT_COUNT);
                let slot_index = D::ASK_SLOT_COUNT as usize - 1;
                let desc = statistics.get_desc(slot_index);
                let instance_kind = desc.instance_kind();
                match instance_kind {
                    EInstanceKind::None => {
                        slotkey_cmd.insert(obj.clone(), D1::create(&desc));
                        slot_cmd.insert(obj.clone(), D::from(desc));
                    },
                    _ => {
                        let buff_id = ins_list.id();
                        slot_cmd.insert(obj.clone(), D::from(desc.clone()));
                        geo_enable_cmd.insert(obj.clone(), RenderGeometryEable(false));

                        match instance_kind {
                            EInstanceKind::WorldMatrix => {
                                let buff = InstancedBufferWorldMatrix { slot: slot_index, id: String::from(buff_id + "WorldMatrix"), index: 0 };
                                ins_wm_cmd.insert(obj.clone(), buff);
                                instance_code.0 = instance_code.0 | EInstanceCode::BASE;
                            },
                            EInstanceKind::Color => {
                                let buff = InstancedBufferColor { slot: slot_index, id: String::from(buff_id + "Color"), index: 0 };
                                ins_color_cmd.insert(obj.clone(), buff);
                                // log::debug!("Instance Color");
                                instance_code.0 = instance_code.0 | EInstanceCode::COLOR;
                            },
                            EInstanceKind::TillOffset => {
                                let buff = InstancedBufferTillOff { slot: slot_index, id: String::from(buff_id + "TillOff"), index: 0 };
                                ins_tilloff_cmd.insert(obj.clone(), buff);
                                // log::debug!("Instance TillOffset");
                                instance_code.0 = instance_code.0 | EInstanceCode::TILL_OFF_1;
                            },
                            _ => { },
                        }
                    },
                };
            }
        });
    }
}


pub type SysGeometryChangeSlot01 = SysGeometryChangeInitSlot<AssetDescVBSlot01, AssetKeyVBSlot01>;
pub type SysGeometryChangeSlot02 = SysGeometryChangeInitSlot<AssetDescVBSlot02, AssetKeyVBSlot02>;
pub type SysGeometryChangeSlot03 = SysGeometryChangeInitSlot<AssetDescVBSlot03, AssetKeyVBSlot03>;
pub type SysGeometryChangeSlot04 = SysGeometryChangeInitSlot<AssetDescVBSlot04, AssetKeyVBSlot04>;
pub type SysGeometryChangeSlot05 = SysGeometryChangeInitSlot<AssetDescVBSlot05, AssetKeyVBSlot05>;
pub type SysGeometryChangeSlot06 = SysGeometryChangeInitSlot<AssetDescVBSlot06, AssetKeyVBSlot06>;
pub type SysGeometryChangeSlot07 = SysGeometryChangeInitSlot<AssetDescVBSlot07, AssetKeyVBSlot07>;
pub type SysGeometryChangeSlot08 = SysGeometryChangeInitSlot<AssetDescVBSlot08, AssetKeyVBSlot08>;
pub type SysGeometryChangeSlot09 = SysGeometryChangeInitSlot<AssetDescVBSlot09, AssetKeyVBSlot09>;
pub type SysGeometryChangeSlot10 = SysGeometryChangeInitSlot<AssetDescVBSlot10, AssetKeyVBSlot10>;
pub type SysGeometryChangeSlot11 = SysGeometryChangeInitSlot<AssetDescVBSlot11, AssetKeyVBSlot11>;
pub type SysGeometryChangeSlot12 = SysGeometryChangeInitSlot<AssetDescVBSlot12, AssetKeyVBSlot12>;
pub type SysGeometryChangeSlot13 = SysGeometryChangeInitSlot<AssetDescVBSlot13, AssetKeyVBSlot13>;
pub type SysGeometryChangeSlot14 = SysGeometryChangeInitSlot<AssetDescVBSlot14, AssetKeyVBSlot14>;
pub type SysGeometryChangeSlot15 = SysGeometryChangeInitSlot<AssetDescVBSlot15, AssetKeyVBSlot15>;
pub type SysGeometryChangeSlot16 = SysGeometryChangeInitSlot<AssetDescVBSlot16, AssetKeyVBSlot16>;

pub struct SysRenderGeometryInit;
impl TSystemStageInfo for SysRenderGeometryInit {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysGeometryStatesInit::key(), SysVertexBufferLoad::key()
        ]
    }
}

pub struct SysGeometryVBUpdateSlot01;
#[setup]
impl SysGeometryVBUpdateSlot01
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   
                ObjectID,
                &GeometryDesc
                , &AssetDescVBSlot01, &AssetResVBSlot01
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot1: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , indicesdesc , indices
        )| {
            log::info!(" vvvvvvvv {}", desc.slot_count());
            if desc.slot_count() == 1 {
                // log::debug!("SysGeometryVBUpdateSlot1: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values, (indicesdesc , indices)));
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot02;
#[setup]
impl SysGeometryVBUpdateSlot02
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   
                ObjectID,
                &GeometryDesc
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
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot2: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , indicesdesc , indices
        )| {
            log::info!(" vvvvvvvv {}", desc.slot_count());
            if desc.slot_count() == 2 {
                // log::debug!("SysGeometryVBUpdateSlot2: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values, (indicesdesc , indices)));
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot03;
#[setup]
impl SysGeometryVBUpdateSlot03
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   
                ObjectID
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
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot3: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , indicesdesc , indices
        )| {
            log::info!(" vvvvvvvv {}", desc.slot_count());
            if desc.slot_count() == 3 {
                log::info!(">>>>>> SysGeometryVBUpdateSlot3: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values, (indicesdesc , indices)));
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot04;
#[setup]
impl SysGeometryVBUpdateSlot04
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   
                ObjectID
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
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot4: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , indicesdesc , indices
        )| {
            // log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 4 {
                log::info!(">>>>>>>>>> SysGeometryVBUpdateSlot4: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values, (indicesdesc , indices)));
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot05;
#[setup]
impl SysGeometryVBUpdateSlot05
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   
                ObjectID
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
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot5: ");
        items.iter_mut().for_each(|(
            id_geo
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
                // log::info!("SysGeometryVBUpdateSlot5: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                    (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values, (indicesdesc , indices)));
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot06;
#[setup]
impl SysGeometryVBUpdateSlot06
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   
                ObjectID
                , &GeometryDesc
                , &AssetDescVBSlot01, &AssetResVBSlot01
                , &AssetDescVBSlot02, &AssetResVBSlot02
                , &AssetDescVBSlot03, &AssetResVBSlot03
                , &AssetDescVBSlot04, &AssetResVBSlot04
                , &AssetDescVBSlot05, &AssetResVBSlot05
                , &AssetDescVBSlot06, &AssetResVBSlot06
                , Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
                Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
                Changed<AssetDescVBSlot05>, Changed<AssetResVBSlot05>,
                Changed<AssetDescVBSlot06>, Changed<AssetResVBSlot06>,
                Changed<AssetResBufferIndices>
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        // log::debug!("SysGeometryVBUpdateSlot6: ");
        items.iter_mut().for_each(|(
            
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , key5, res5
            , key6, res6
            , indicesdesc , indices
        )| {
            // log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 6 {
                // log::info!("SysGeometryVBUpdateSlot6: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                    (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
                    (key6.desc().step_mode(), RenderVertices::create(key6, res6)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values, (indicesdesc , indices)));
            }
        });
    }
}

pub struct  PluginVertexBuffers;
impl pi_engine_shell::plugin::Plugin for PluginVertexBuffers {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {

        let world = engine.world_mut();

        let stage_builder = stages.query_stage::<SysGeometryStatesInit>(ERunStageChap::Initial);

        SysGeometryChangeSlot01::setup(world, stage_builder);
        SysGeometryChangeSlot02::setup(world, stage_builder);
        SysGeometryChangeSlot03::setup(world, stage_builder);
        SysGeometryChangeSlot04::setup(world, stage_builder);
        SysGeometryChangeSlot05::setup(world, stage_builder);
        SysGeometryChangeSlot06::setup(world, stage_builder);
        SysGeometryChangeSlot07::setup(world, stage_builder);
        SysGeometryChangeSlot08::setup(world, stage_builder);
        SysGeometryChangeSlot09::setup(world, stage_builder);
        SysGeometryChangeSlot10::setup(world, stage_builder);
        SysGeometryChangeSlot11::setup(world, stage_builder);
        SysGeometryChangeSlot12::setup(world, stage_builder);
        SysGeometryChangeSlot13::setup(world, stage_builder);
        SysGeometryChangeSlot14::setup(world, stage_builder);
        SysGeometryChangeSlot15::setup(world, stage_builder);
        SysGeometryChangeSlot16::setup(world, stage_builder);

        let stage_builder = stages.query_stage::<SysRenderGeometryInit>(ERunStageChap::Initial);
        SysGeometryVBUpdateSlot01::setup(world, stage_builder);
        SysGeometryVBUpdateSlot02::setup(world, stage_builder);
        SysGeometryVBUpdateSlot03::setup(world, stage_builder);
        SysGeometryVBUpdateSlot04::setup(world, stage_builder);
        SysGeometryVBUpdateSlot05::setup(world, stage_builder);
        SysGeometryVBUpdateSlot06::setup(world, stage_builder);

        Ok(())
    }
}