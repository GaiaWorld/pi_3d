use std::marker::PhantomData;

use pi_ecs::{prelude::{Component, Query, Setup, ResMut}, query::{Changed, Write, WithOut, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::object::{GameObject, ObjectID};
use render_data_container::{RenderVertices, VertexBufferPool, KeyVertexBuffer};
use render_geometry::vertex_data::TVertexBufferDesc;
use render_shader::instance_code::EInstanceCode;

use crate::{geometry::{geometry::RenderVerticesFrom, instance::instanced_buffer::TInstancedBuffer}, meshes::instance::{instanced_mesh::{InstanceSourceRecord, InstanceList, InstanceSource}}};

use super::{vertex_buffer_useinfo::{TVertexBufferUseInfo, AssetDescVBSlot1, AssetDescVBSlot2, AssetDescVBSlot3, AssetDescVBSlot5, AssetDescVBSlot6, AssetDescVBSlot7, AssetDescVBSlot8, AssetDescVBSlot9, AssetDescVBSlot4, AssetResVBSlot1, AssetResVBSlot2, AssetResVBSlot3, AssetResVBSlot4, AssetResVBSlot5, AssetResVBSlot6, AssetResVBSlot7, AssetResVBSlot8, AssetResVBSlot9, AsKeyVertexBuffer, AssetKeyVBSlot1, AssetKeyVBSlot2, AssetKeyVBSlot3, AssetKeyVBSlot4, AssetKeyVBSlot5, AssetKeyVBSlot6, AssetKeyVBSlot7, AssetKeyVBSlot8, AssetKeyVBSlot9, AssetKeyVBSlot10, AssetKeyVBSlot11, AssetKeyVBSlot12, AssetKeyVBSlot13, AssetDescVBSlot10, AssetDescVBSlot11, AssetDescVBSlot12, AssetDescVBSlot13, TAssetResVertexBuffer, AssetResVBSlot10, AssetResVBSlot11, AssetResVBSlot12, AssetResVBSlot13}, GeometryDesc, geometry::{RenderGeometry, RenderGeometryEable}, instance::{instance_world_matrix::InstancedBufferWorldMatrix, instance_color::InstancedBufferColor}};


pub struct SysGeometryChangeIntSlot<D: TVertexBufferUseInfo + Component, D1: AsKeyVertexBuffer + Component>(PhantomData<(D, D1)>);
#[setup]
impl<D, D1> SysGeometryChangeIntSlot<D, D1>
where
    D: TVertexBufferUseInfo + Component,
    D1: AsKeyVertexBuffer + Component,
{
    #[system]
    pub fn material_change(
        mut items: Query<
            GameObject,
            (ObjectID, &GeometryDesc, Write<D>, Write<D1>, Write<EInstanceCode>, Write<InstanceList>, Write<RenderGeometryEable>, Write<InstancedBufferWorldMatrix>, Write<InstancedBufferColor>),
            (Changed<GeometryDesc>),
        >,
        mut ins_record: ResMut<InstanceSourceRecord>,
        mut vbpool: ResMut<VertexBufferPool>,
    ) {
        items.iter_mut().for_each(|(
            id, statistics, mut slot, mut slotkey, mut instance_code, 
            mut ins_list, mut render_geoenable,
            mut buf_wm, mut buf_color
        )| {
            
            if statistics.slot_count() >= D::ASK_SLOT_COUNT as usize {
                // println!(">>>>> Geo Slot {}", D::ASK_SLOT_COUNT); 
                let slot_index = D::ASK_SLOT_COUNT as usize - 1;
                let desc = statistics.get_desc(slot_index);
                let instance_kind = desc.instance_kind();
                match instance_kind {
                    render_geometry::vertex_data::EInstanceKind::None => {
                        slotkey.write(D1::create(&desc));
                        slot.write(D::from(desc));
                    },
                    _ => {
                        let buff_id = if let Some(temp) = ins_list.get_mut() {
                            temp.id()
                        } else {
                            let instance_list = InstanceList::new(&mut ins_record);
                            let id = instance_list.id();
                            ins_list.write(instance_list);
                            id
                        };
                        slot.write(D::from(desc));
                        render_geoenable.write(RenderGeometryEable(false));

                        match instance_kind {
                            render_geometry::vertex_data::EInstanceKind::WorldMatrix => {
                                let buff = InstancedBufferWorldMatrix::new(slot_index, buff_id, &mut vbpool);
                                buf_wm.write(buff);
            
                                if let Some(ins_code) = instance_code.get_mut() {
                                    ins_code.0 = ins_code.0 | EInstanceCode::BASE;
                                    instance_code.notify_modify();
                                } else {
                                    instance_code.write(EInstanceCode(EInstanceCode::BASE));
                                }
                            },
                            render_geometry::vertex_data::EInstanceKind::Color => {
                                let buff = InstancedBufferColor::new(slot_index, buff_id, &mut vbpool);
                                buf_color.write(buff);
            
                                if let Some(ins_code) = instance_code.get_mut() {
                                    ins_code.0 = ins_code.0 | EInstanceCode::COLOR;
                                    instance_code.notify_modify();
                                } else {
                                    instance_code.write(EInstanceCode(EInstanceCode::COLOR));
                                }
                            },
                            render_geometry::vertex_data::EInstanceKind::TillOffset => todo!(),
                            _ => { },
                        }
                    },
                };
            }
        });
    }
}


pub type SysGeometryChangeSlot1 = SysGeometryChangeIntSlot<AssetDescVBSlot1, AssetKeyVBSlot1>;
pub type SysGeometryChangeSlot2 = SysGeometryChangeIntSlot<AssetDescVBSlot2, AssetKeyVBSlot2>;
pub type SysGeometryChangeSlot3 = SysGeometryChangeIntSlot<AssetDescVBSlot3, AssetKeyVBSlot3>;
pub type SysGeometryChangeSlot4 = SysGeometryChangeIntSlot<AssetDescVBSlot4, AssetKeyVBSlot4>;
pub type SysGeometryChangeSlot5 = SysGeometryChangeIntSlot<AssetDescVBSlot5, AssetKeyVBSlot5>;
pub type SysGeometryChangeSlot6 = SysGeometryChangeIntSlot<AssetDescVBSlot6, AssetKeyVBSlot6>;
pub type SysGeometryChangeSlot7 = SysGeometryChangeIntSlot<AssetDescVBSlot7, AssetKeyVBSlot7>;
pub type SysGeometryChangeSlot8 = SysGeometryChangeIntSlot<AssetDescVBSlot8, AssetKeyVBSlot8>;
pub type SysGeometryChangeSlot9 = SysGeometryChangeIntSlot<AssetDescVBSlot9, AssetKeyVBSlot9>;
pub type SysGeometryChangeSlot10 = SysGeometryChangeIntSlot<AssetDescVBSlot10, AssetKeyVBSlot10>;
pub type SysGeometryChangeSlot11 = SysGeometryChangeIntSlot<AssetDescVBSlot11, AssetKeyVBSlot11>;
pub type SysGeometryChangeSlot12 = SysGeometryChangeIntSlot<AssetDescVBSlot12, AssetKeyVBSlot12>;
pub type SysGeometryChangeSlot13 = SysGeometryChangeIntSlot<AssetDescVBSlot13, AssetKeyVBSlot13>;

pub struct SysGeometryVBUpdateSlot1;
#[setup]
impl SysGeometryVBUpdateSlot1
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   &GeometryDesc, Write<RenderGeometry>, &RenderGeometryEable
                , &AssetDescVBSlot1, &AssetResVBSlot1,
            ),
            Or<(
                Changed<RenderGeometryEable>, 
                Changed<AssetDescVBSlot1>, Changed<AssetResVBSlot1>,
            )>
        >,
    ) {
        items.iter_mut().for_each(|(
            desc, mut geometry, geo_disable
            , key1, res1
        )| {
            if desc.slot_count() == 1 {
                if geo_disable.0 == false {
                    geometry.remove();
                } else {
                    let values = vec![
                        (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                    ];
                    geometry.write(RenderGeometry::create(values));
                }
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot2;
#[setup]
impl SysGeometryVBUpdateSlot2
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   &GeometryDesc, Write<RenderGeometry>, &RenderGeometryEable
                , &AssetDescVBSlot1, &AssetResVBSlot1
                , &AssetDescVBSlot2, &AssetResVBSlot2
            ),
            Or<(
                Changed<RenderGeometryEable>, 
                Changed<AssetDescVBSlot1>, Changed<AssetResVBSlot1>,
                Changed<AssetDescVBSlot2>, Changed<AssetResVBSlot2>,
            )>
        >,
    ) {
        items.iter_mut().for_each(|(
            desc, mut geometry, geo_disable
            , key1, res1
            , key2, res2
        )| {
            if desc.slot_count() == 2 {
                if geo_disable.0 == false {
                    geometry.remove();
                } else {
                    let values = vec![
                        (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                        (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                    ];
                    geometry.write(RenderGeometry::create(values));
                }
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot3;
#[setup]
impl SysGeometryVBUpdateSlot3
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   &GeometryDesc, Write<RenderGeometry>, &RenderGeometryEable
                , &AssetDescVBSlot1, &AssetResVBSlot1
                , &AssetDescVBSlot2, &AssetResVBSlot2
                , &AssetDescVBSlot3, &AssetResVBSlot3
            ),
            Or<(
                Changed<RenderGeometryEable>, 
                Changed<AssetDescVBSlot1>, Changed<AssetResVBSlot1>,
                Changed<AssetDescVBSlot2>, Changed<AssetResVBSlot2>,
                Changed<AssetDescVBSlot3>, Changed<AssetResVBSlot3>,
            )>
        >,
    ) {
        items.iter_mut().for_each(|(
            desc, mut geometry, geo_disable
            , key1, res1
            , key2, res2
            , key3, res3
        )| {
            if desc.slot_count() == 3 {
                if geo_disable.0 == false {
                    geometry.remove();
                } else {
                    let values = vec![
                        (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                        (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                        (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                    ];
                    geometry.write(RenderGeometry::create(values));
                }
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot4;
#[setup]
impl SysGeometryVBUpdateSlot4
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   &GeometryDesc, Write<RenderGeometry>, &RenderGeometryEable
                , &AssetDescVBSlot1, &AssetResVBSlot1
                , &AssetDescVBSlot2, &AssetResVBSlot2
                , &AssetDescVBSlot3, &AssetResVBSlot3
                , &AssetDescVBSlot4, &AssetResVBSlot4
            ),
            Or<(
                Changed<RenderGeometryEable>, 
                Changed<AssetDescVBSlot1>, Changed<AssetResVBSlot1>,
                Changed<AssetDescVBSlot2>, Changed<AssetResVBSlot2>,
                Changed<AssetDescVBSlot3>, Changed<AssetResVBSlot3>,
                Changed<AssetDescVBSlot4>, Changed<AssetResVBSlot4>,
            )>
        >,
    ) {
        items.iter_mut().for_each(|(
            desc, mut geometry, geo_disable
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
        )| {
            if desc.slot_count() == 4 {
                // println!("VBUpdateSlot4");
                if geo_disable.0 == false {
                    geometry.remove();
                } else {
                    let values = vec![
                        (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                        (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                        (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                        (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                    ];
                    geometry.write(RenderGeometry::create(values));
                }
            }
        });
    }
}

pub struct SysGeometryVBUpdateSlot5;
#[setup]
impl SysGeometryVBUpdateSlot5
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject,
            (   &GeometryDesc, Write<RenderGeometry>, &RenderGeometryEable
                , &AssetDescVBSlot1, &AssetResVBSlot1
                , &AssetDescVBSlot2, &AssetResVBSlot2
                , &AssetDescVBSlot3, &AssetResVBSlot3
                , &AssetDescVBSlot4, &AssetResVBSlot4
                , &AssetDescVBSlot5, &AssetResVBSlot5
            ),
            Or<(
                Changed<RenderGeometryEable>, 
                Changed<AssetDescVBSlot1>, Changed<AssetResVBSlot1>,
                Changed<AssetDescVBSlot2>, Changed<AssetResVBSlot2>,
                Changed<AssetDescVBSlot3>, Changed<AssetResVBSlot3>,
                Changed<AssetDescVBSlot4>, Changed<AssetResVBSlot4>,
                Changed<AssetDescVBSlot5>, Changed<AssetResVBSlot5>,
            )>
        >,
    ) {
        items.iter_mut().for_each(|(
            desc, mut geometry, geo_disable
            , key1, res1
            , key2, res2
            , key3, res3
            , key4, res4
            , key5, res5
        )| {
            if desc.slot_count() == 5 {
                println!("VBUpdateSlot5");
                if geo_disable.0 == false {
                    geometry.remove();
                } else {
                    let values = vec![
                        (key1.desc().step_mode(), RenderVertices::create(key1, res1)),
                        (key2.desc().step_mode(), RenderVertices::create(key2, res2)),
                        (key3.desc().step_mode(), RenderVertices::create(key3, res3)),
                        (key4.desc().step_mode(), RenderVertices::create(key4, res4)),
                        (key5.desc().step_mode(), RenderVertices::create(key5, res5)),
                    ];
                    geometry.write(RenderGeometry::create(values));
                }
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
        SysGeometryChangeSlot1::setup(world, stages.command_stage());
        SysGeometryChangeSlot2::setup(world, stages.command_stage());
        SysGeometryChangeSlot3::setup(world, stages.command_stage());
        SysGeometryChangeSlot4::setup(world, stages.command_stage());
        SysGeometryChangeSlot5::setup(world, stages.command_stage());
        SysGeometryChangeSlot6::setup(world, stages.command_stage());
        SysGeometryChangeSlot7::setup(world, stages.command_stage());
        SysGeometryChangeSlot8::setup(world, stages.command_stage());
        SysGeometryChangeSlot9::setup(world, stages.command_stage());
        SysGeometryChangeSlot10::setup(world, stages.command_stage());
        SysGeometryChangeSlot11::setup(world, stages.command_stage());
        SysGeometryChangeSlot12::setup(world, stages.command_stage());
        SysGeometryChangeSlot13::setup(world, stages.command_stage());

        SysGeometryVBUpdateSlot1::setup(world, stages.uniform_update());
        SysGeometryVBUpdateSlot2::setup(world, stages.uniform_update());
        SysGeometryVBUpdateSlot3::setup(world, stages.uniform_update());
        SysGeometryVBUpdateSlot4::setup(world, stages.uniform_update());
        SysGeometryVBUpdateSlot5::setup(world, stages.uniform_update());

        Ok(())
    }
}