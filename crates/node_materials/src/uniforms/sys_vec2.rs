use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, ResMut, }, query::{Changed, Write}};
use pi_ecs_macros::{setup};
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::Uniform};
use pi_scene_context::resources::RenderDynUniformBuffer;

use crate::{uniforms::{vec2::{Vec2Slot1, Vec2Slot3, Vec2Slot2, Vec2Slot4, Vec2Slot5, Vec2Slot6, Vec2Slot7, Vec2Slot8}, }, value::{FromValueUniformStatistics, SlotActiveRequire}};

use super::value_uniform::{ValueUniformStatistics, ValueUniformDynBindOffset};


pub struct SysMaterialChangeVec2Slot<D: FromValueUniformStatistics + SlotActiveRequire>(PhantomData<D>);
#[setup]
impl<D> SysMaterialChangeVec2Slot<D>
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

pub struct SysUpdateVec2Slot<D: FromValueUniformStatistics + Uniform>(PhantomData<D>);
#[setup]
impl<D> SysUpdateVec2Slot<D>
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

pub type SysMaterialChangeVec2Slot1 = SysMaterialChangeVec2Slot<Vec2Slot1>;
pub type SysMaterialChangeVec2Slot2 = SysMaterialChangeVec2Slot<Vec2Slot2>;
pub type SysMaterialChangeVec2Slot3 = SysMaterialChangeVec2Slot<Vec2Slot3>;
pub type SysMaterialChangeVec2Slot4 = SysMaterialChangeVec2Slot<Vec2Slot4>;
pub type SysMaterialChangeVec2Slot5 = SysMaterialChangeVec2Slot<Vec2Slot5>;
pub type SysMaterialChangeVec2Slot6 = SysMaterialChangeVec2Slot<Vec2Slot6>;
pub type SysMaterialChangeVec2Slot7 = SysMaterialChangeVec2Slot<Vec2Slot7>;
pub type SysMaterialChangeVec2Slot8 = SysMaterialChangeVec2Slot<Vec2Slot8>;


pub type SysUpdateVec2Slot1 = SysUpdateVec2Slot<Vec2Slot1>;
pub type SysUpdateVec2Slot2 = SysUpdateVec2Slot<Vec2Slot2>;
pub type SysUpdateVec2Slot3 = SysUpdateVec2Slot<Vec2Slot3>;
pub type SysUpdateVec2Slot4 = SysUpdateVec2Slot<Vec2Slot4>;
pub type SysUpdateVec2Slot5 = SysUpdateVec2Slot<Vec2Slot5>;
pub type SysUpdateVec2Slot6 = SysUpdateVec2Slot<Vec2Slot6>;
pub type SysUpdateVec2Slot7 = SysUpdateVec2Slot<Vec2Slot7>;
pub type SysUpdateVec2Slot8 = SysUpdateVec2Slot<Vec2Slot8>;