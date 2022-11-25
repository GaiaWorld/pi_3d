use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, ResMut, }, query::{Changed, Write}};
use pi_ecs_macros::{setup};
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::Uniform};
use pi_scene_context::resources::RenderDynUniformBuffer;

use crate::{uniforms::{int::{IntSlot1, IntSlot3, IntSlot2, IntSlot4, IntSlot5, IntSlot6, IntSlot7, IntSlot8}, }, value::{FromValueUniformStatistics, SlotActiveRequire}};

use super::value_uniform::{ValueUniformStatistics, ValueUniformDynBindOffset};


pub struct SysMaterialChangeIntSlot<D: FromValueUniformStatistics + SlotActiveRequire>(PhantomData<D>);
#[setup]
impl<D> SysMaterialChangeIntSlot<D>
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

pub struct SysUpdateIntSlot<D: FromValueUniformStatistics + Uniform>(PhantomData<D>);
#[setup]
impl<D> SysUpdateIntSlot<D>
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

pub type SysMaterialChangeIntSlot1 = SysMaterialChangeIntSlot<IntSlot1>;
pub type SysMaterialChangeIntSlot2 = SysMaterialChangeIntSlot<IntSlot2>;
pub type SysMaterialChangeIntSlot3 = SysMaterialChangeIntSlot<IntSlot3>;
pub type SysMaterialChangeIntSlot4 = SysMaterialChangeIntSlot<IntSlot4>;
pub type SysMaterialChangeIntSlot5 = SysMaterialChangeIntSlot<IntSlot5>;
pub type SysMaterialChangeIntSlot6 = SysMaterialChangeIntSlot<IntSlot6>;
pub type SysMaterialChangeIntSlot7 = SysMaterialChangeIntSlot<IntSlot7>;
pub type SysMaterialChangeIntSlot8 = SysMaterialChangeIntSlot<IntSlot8>;


pub type SysUpdateIntSlot1 = SysUpdateIntSlot<IntSlot1>;
pub type SysUpdateIntSlot2 = SysUpdateIntSlot<IntSlot2>;
pub type SysUpdateIntSlot3 = SysUpdateIntSlot<IntSlot3>;
pub type SysUpdateIntSlot4 = SysUpdateIntSlot<IntSlot4>;
pub type SysUpdateIntSlot5 = SysUpdateIntSlot<IntSlot5>;
pub type SysUpdateIntSlot6 = SysUpdateIntSlot<IntSlot6>;
pub type SysUpdateIntSlot7 = SysUpdateIntSlot<IntSlot7>;
pub type SysUpdateIntSlot8 = SysUpdateIntSlot<IntSlot8>;