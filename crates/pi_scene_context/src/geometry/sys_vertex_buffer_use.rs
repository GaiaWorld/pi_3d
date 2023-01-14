use std::marker::PhantomData;

use pi_ecs::{prelude::{Component, Query, Setup, ResMut, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::{TSystemStageInfo, ERunStageChap}};
use render_data_container::{RenderVertices, VertexBufferPool};
use render_geometry::vertex_data::TVertexBufferDesc;
use render_shader::instance_code::EInstanceCode;

use crate::{geometry::{geometry::RenderVerticesFrom, instance::{instanced_buffer::TInstancedBuffer, InstanceList}}};

use super::{vertex_buffer_useinfo::{TVertexBufferUseInfo, AssetDescVBSlot01, AssetDescVBSlot02, AssetDescVBSlot03, AssetDescVBSlot05, AssetDescVBSlot06, AssetDescVBSlot07, AssetDescVBSlot08, AssetDescVBSlot09, AssetDescVBSlot04, AssetResVBSlot01, AssetResVBSlot02, AssetResVBSlot03, AssetResVBSlot04, AssetResVBSlot05, AssetResVBSlot06, AssetResVBSlot07, AssetResVBSlot08, AssetResVBSlot09, AsKeyVertexBuffer, AssetKeyVBSlot01, AssetKeyVBSlot02, AssetKeyVBSlot03, AssetKeyVBSlot04, AssetKeyVBSlot05, AssetKeyVBSlot06, AssetKeyVBSlot07, AssetKeyVBSlot08, AssetKeyVBSlot09, AssetKeyVBSlot10, AssetKeyVBSlot11, AssetKeyVBSlot12, AssetKeyVBSlot13, AssetDescVBSlot10, AssetDescVBSlot11, AssetDescVBSlot12, AssetDescVBSlot13, TAssetResVertexBuffer, AssetResVBSlot10, AssetResVBSlot11, AssetResVBSlot12, AssetResVBSlot13}, GeometryDesc, geometry::{RenderGeometry, RenderGeometryEable}, instance::{instance_world_matrix::InstancedBufferWorldMatrix, instance_color::InstancedBufferColor, instance_tilloff::InstancedBufferTillOff}, SysGeometryVBCommand, SysVertexBufferLoad};

pub struct SysGeometryStatesInit;
impl TSystemStageInfo for SysGeometryStatesInit {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysGeometryVBCommand::key(),
        ]
    }
}

pub struct SysGeometryChangeInitSlot<D: TVertexBufferUseInfo + Component, D1: AsKeyVertexBuffer + Component>(PhantomData<(D, D1)>);
impl<D, D1> TSystemStageInfo for SysGeometryChangeInitSlot<D, D1>
where
    D: TVertexBufferUseInfo + Component,
    D1: AsKeyVertexBuffer + Component,
{
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysGeometryVBCommand::key(), 
        ]
    }
}
#[setup]
impl<D, D1> SysGeometryChangeInitSlot<D, D1>
where
    D: TVertexBufferUseInfo + Component,
    D1: AsKeyVertexBuffer + Component,
{
    #[system]
    pub fn material_change(
        mut items: Query<
            GameObject,
            (ObjectID, &GeometryDesc, &mut EInstanceCode, &InstanceList),
            Changed<GeometryDesc>,
        >,
        mut vbpool: ResMut<VertexBufferPool>,
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
                log::info!("SysGeometryChangeIntSlot: Slot {}", D::ASK_SLOT_COUNT);
                let slot_index = D::ASK_SLOT_COUNT as usize - 1;
                let desc = statistics.get_desc(slot_index);
                let instance_kind = desc.instance_kind();
                match instance_kind {
                    render_geometry::vertex_data::EInstanceKind::None => {
                        slotkey_cmd.insert(obj.clone(), D1::create(&desc));
                        slot_cmd.insert(obj.clone(), D::from(desc));
                    },
                    _ => {
                        let buff_id = ins_list.id();
                        slot_cmd.insert(obj.clone(), D::from(desc.clone()));
                        geo_enable_cmd.insert(obj.clone(), RenderGeometryEable(false));

                        match instance_kind {
                            render_geometry::vertex_data::EInstanceKind::WorldMatrix => {
                                let buff = InstancedBufferWorldMatrix::new(slot_index, buff_id, &mut vbpool);
                                ins_wm_cmd.insert(obj.clone(), buff);
                                instance_code.0 = instance_code.0 | EInstanceCode::BASE;
                            },
                            render_geometry::vertex_data::EInstanceKind::Color => {
                                let buff = InstancedBufferColor::new(slot_index, buff_id, &mut vbpool);
                                ins_color_cmd.insert(obj.clone(), buff);
                                log::info!("Instance Color");
                                instance_code.0 = instance_code.0 | EInstanceCode::COLOR;
                            },
                            render_geometry::vertex_data::EInstanceKind::TillOffset => {
                                let buff = InstancedBufferTillOff::new(slot_index, buff_id, &mut vbpool);
                                ins_tilloff_cmd.insert(obj.clone(), buff);
                                log::info!("Instance TillOffset");
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
                , &AssetDescVBSlot01, &AssetResVBSlot01,
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        log::debug!("SysGeometryVBUpdateSlot1: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
        )| {
            log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 1 {
                log::debug!("SysGeometryVBUpdateSlot1: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values));
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
            ),
            Or<(
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        log::debug!("SysGeometryVBUpdateSlot2: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , key2, res2
        )| {
            log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 2 {
                log::debug!("SysGeometryVBUpdateSlot2: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values));
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
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        log::debug!("SysGeometryVBUpdateSlot3: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
        )| {
            log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 3 {
                log::debug!("SysGeometryVBUpdateSlot3: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values));
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
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
                Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        log::debug!("SysGeometryVBUpdateSlot4: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
        )| {
            log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 4 {
                log::debug!("SysGeometryVBUpdateSlot4: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values));
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
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
                Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
                Changed<AssetDescVBSlot05>, Changed<AssetResVBSlot05>,
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        log::debug!("SysGeometryVBUpdateSlot5: ");
        items.iter_mut().for_each(|(
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , key5, res5
        )| {
            log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 5 {
                log::info!("SysGeometryVBUpdateSlot5: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                    (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values));
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
            ),
            Or<(
                
                Changed<AssetDescVBSlot01>, Changed<AssetResVBSlot01>,
                Changed<AssetDescVBSlot02>, Changed<AssetResVBSlot02>,
                Changed<AssetDescVBSlot03>, Changed<AssetResVBSlot03>,
                Changed<AssetDescVBSlot04>, Changed<AssetResVBSlot04>,
                Changed<AssetDescVBSlot05>, Changed<AssetResVBSlot05>,
                Changed<AssetDescVBSlot06>, Changed<AssetResVBSlot06>,
            )>
        >,
        mut geo_cmd: Commands<GameObject, RenderGeometry>,
    ) {
        log::debug!("SysGeometryVBUpdateSlot6: ");
        items.iter_mut().for_each(|(
            
            id_geo
            , desc
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , key5, res5
            , key6, res6
        )| {
            log::debug!(" > {}", desc.slot_count());
            if desc.slot_count() == 6 {
                log::info!("SysGeometryVBUpdateSlot6: 0");
                let values = vec![
                    (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                    (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
                    (key6.desc().step_mode(), RenderVertices::create(key6, res6)),
                ];
                geo_cmd.insert(id_geo.clone(), RenderGeometry::create(values));
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

        let stage_builder = stages.query_stage::<SysGeometryStatesInit>(ERunStageChap::Command);

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

        let stage_builder = stages.query_stage::<SysRenderGeometryInit>(ERunStageChap::Command);
        SysGeometryVBUpdateSlot01::setup(world, stage_builder);
        SysGeometryVBUpdateSlot02::setup(world, stage_builder);
        SysGeometryVBUpdateSlot03::setup(world, stage_builder);
        SysGeometryVBUpdateSlot04::setup(world, stage_builder);
        SysGeometryVBUpdateSlot05::setup(world, stage_builder);
        SysGeometryVBUpdateSlot06::setup(world, stage_builder);

        Ok(())
    }
}