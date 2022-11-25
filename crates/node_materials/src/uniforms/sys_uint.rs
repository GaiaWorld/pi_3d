use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, ResMut, }, query::{Changed, Write}};
use pi_ecs_macros::{setup};
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::Uniform};
use pi_scene_context::resources::RenderDynUniformBuffer;

use crate::{uniforms::{uint::{UintSlot1, UintSlot3, UintSlot2, UintSlot4, UintSlot5, UintSlot6, UintSlot7, UintSlot8}, }, value::{FromValueUniformStatistics, SlotActiveRequire}};

use super::value_uniform::{ValueUniformStatistics, ValueUniformDynBindOffset};


pub struct SysMaterialChangeUintSlot<D: FromValueUniformStatistics + SlotActiveRequire>(PhantomData<D>);
#[setup]
impl<D> SysMaterialChangeUintSlot<D>
where
    D: FromValueUniformStatistics + SlotActiveRequire,
{
    #[system]
    pub fn material_change(
        mut items: Query<
            GameObject,
            (&ValueUniformStatistics, Write<D>),
            (Changed<ValueUniformStatistics>),
        >,
    ) {
        items.iter_mut().for_each(|(statistics, mut slot)| {
            if statistics.mat4_count >= D::ASK_SLOT_COUNT {
                slot.write(D::new(statistics));
            }
        });
    }
}

pub struct SysUpdateUintSlot<D: FromValueUniformStatistics + Uniform>(PhantomData<D>);
#[setup]
impl<D> SysUpdateUintSlot<D>
where
    D: FromValueUniformStatistics + Uniform,
{
    #[system]
    pub fn slot_change(
        mut items: Query<
            GameObject, 
            (&ValueUniformDynBindOffset, &D), 
            (Changed<D>)
        >,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        items.iter_mut().for_each(|(bindoffset, slot)| {
            dynbuffer.as_mut().set_uniform(&bindoffset.bind_offset, slot);
        });
    }
}

pub type SysMaterialChangeUintSlot1 = SysMaterialChangeUintSlot<UintSlot1>;
pub type SysMaterialChangeUintSlot2 = SysMaterialChangeUintSlot<UintSlot2>;
pub type SysMaterialChangeUintSlot3 = SysMaterialChangeUintSlot<UintSlot3>;
pub type SysMaterialChangeUintSlot4 = SysMaterialChangeUintSlot<UintSlot4>;
pub type SysMaterialChangeUintSlot5 = SysMaterialChangeUintSlot<UintSlot5>;
pub type SysMaterialChangeUintSlot6 = SysMaterialChangeUintSlot<UintSlot6>;
pub type SysMaterialChangeUintSlot7 = SysMaterialChangeUintSlot<UintSlot7>;
pub type SysMaterialChangeUintSlot8 = SysMaterialChangeUintSlot<UintSlot8>;


pub type SysUpdateUintSlot1 = SysUpdateUintSlot<UintSlot1>;
pub type SysUpdateUintSlot2 = SysUpdateUintSlot<UintSlot2>;
pub type SysUpdateUintSlot3 = SysUpdateUintSlot<UintSlot3>;
pub type SysUpdateUintSlot4 = SysUpdateUintSlot<UintSlot4>;
pub type SysUpdateUintSlot5 = SysUpdateUintSlot<UintSlot5>;
pub type SysUpdateUintSlot6 = SysUpdateUintSlot<UintSlot6>;
pub type SysUpdateUintSlot7 = SysUpdateUintSlot<UintSlot7>;
pub type SysUpdateUintSlot8 = SysUpdateUintSlot<UintSlot8>;