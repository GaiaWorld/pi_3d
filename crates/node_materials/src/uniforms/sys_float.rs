use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, ResMut, }, query::{Changed, Write}};
use pi_ecs_macros::{setup};
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::Uniform};
use pi_scene_context::resources::RenderDynUniformBuffer;

use crate::{uniforms::{float::{FloatSlot1, FloatSlot3, FloatSlot2, FloatSlot4, FloatSlot5, FloatSlot6, FloatSlot7, FloatSlot8}}, value::{FromValueUniformStatistics, SlotActiveRequire}};

use super::value_uniform::{ValueUniformStatistics, ValueUniformDynBindOffset};


pub struct SysMaterialChangeFloatSlot<D: FromValueUniformStatistics + SlotActiveRequire>(PhantomData<D>);
#[setup]
impl<D> SysMaterialChangeFloatSlot<D>
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
            if statistics.float_count >= D::ASK_SLOT_COUNT {
                slot.write(D::new(statistics));
            }
        });
    }
}

pub struct SysUpdateFloatSlot<D: FromValueUniformStatistics + Uniform>(PhantomData<D>);
#[setup]
impl<D> SysUpdateFloatSlot<D>
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

pub type SysMaterialChangeFloatSlot1 = SysMaterialChangeFloatSlot<FloatSlot1>;
pub type SysMaterialChangeFloatSlot2 = SysMaterialChangeFloatSlot<FloatSlot2>;
pub type SysMaterialChangeFloatSlot3 = SysMaterialChangeFloatSlot<FloatSlot3>;
pub type SysMaterialChangeFloatSlot4 = SysMaterialChangeFloatSlot<FloatSlot4>;
pub type SysMaterialChangeFloatSlot5 = SysMaterialChangeFloatSlot<FloatSlot5>;
pub type SysMaterialChangeFloatSlot6 = SysMaterialChangeFloatSlot<FloatSlot6>;
pub type SysMaterialChangeFloatSlot7 = SysMaterialChangeFloatSlot<FloatSlot7>;
pub type SysMaterialChangeFloatSlot8 = SysMaterialChangeFloatSlot<FloatSlot8>;


pub type SysUpdateFloatSlot1 = SysUpdateFloatSlot<FloatSlot1>;
pub type SysUpdateFloatSlot2 = SysUpdateFloatSlot<FloatSlot2>;
pub type SysUpdateFloatSlot3 = SysUpdateFloatSlot<FloatSlot3>;
pub type SysUpdateFloatSlot4 = SysUpdateFloatSlot<FloatSlot4>;
pub type SysUpdateFloatSlot5 = SysUpdateFloatSlot<FloatSlot5>;
pub type SysUpdateFloatSlot6 = SysUpdateFloatSlot<FloatSlot6>;
pub type SysUpdateFloatSlot7 = SysUpdateFloatSlot<FloatSlot7>;
pub type SysUpdateFloatSlot8 = SysUpdateFloatSlot<FloatSlot8>;