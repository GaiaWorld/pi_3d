use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, ResMut, }, query::{Changed, Write}};
use pi_ecs_macros::{setup};
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::Uniform};
use pi_scene_context::resources::RenderDynUniformBuffer;

use crate::{uniforms::{vec4::{Vec4Slot1, Vec4Slot3, Vec4Slot2, Vec4Slot4, Vec4Slot5, Vec4Slot6, Vec4Slot7, Vec4Slot8}, }, value::{FromValueUniformStatistics, SlotActiveRequire}};

use super::value_uniform::{ValueUniformStatistics, ValueUniformDynBindOffset};


pub struct SysMaterialChangeVec4Slot<D: FromValueUniformStatistics + SlotActiveRequire>(PhantomData<D>);
#[setup]
impl<D> SysMaterialChangeVec4Slot<D>
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

pub struct SysUpdateVec4Slot<D: FromValueUniformStatistics + Uniform>(PhantomData<D>);
#[setup]
impl<D> SysUpdateVec4Slot<D>
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

pub type SysMaterialChangeVec4Slot1 = SysMaterialChangeVec4Slot<Vec4Slot1>;
pub type SysMaterialChangeVec4Slot2 = SysMaterialChangeVec4Slot<Vec4Slot2>;
pub type SysMaterialChangeVec4Slot3 = SysMaterialChangeVec4Slot<Vec4Slot3>;
pub type SysMaterialChangeVec4Slot4 = SysMaterialChangeVec4Slot<Vec4Slot4>;
pub type SysMaterialChangeVec4Slot5 = SysMaterialChangeVec4Slot<Vec4Slot5>;
pub type SysMaterialChangeVec4Slot6 = SysMaterialChangeVec4Slot<Vec4Slot6>;
pub type SysMaterialChangeVec4Slot7 = SysMaterialChangeVec4Slot<Vec4Slot7>;
pub type SysMaterialChangeVec4Slot8 = SysMaterialChangeVec4Slot<Vec4Slot8>;


pub type SysUpdateVec4Slot1 = SysUpdateVec4Slot<Vec4Slot1>;
pub type SysUpdateVec4Slot2 = SysUpdateVec4Slot<Vec4Slot2>;
pub type SysUpdateVec4Slot3 = SysUpdateVec4Slot<Vec4Slot3>;
pub type SysUpdateVec4Slot4 = SysUpdateVec4Slot<Vec4Slot4>;
pub type SysUpdateVec4Slot5 = SysUpdateVec4Slot<Vec4Slot5>;
pub type SysUpdateVec4Slot6 = SysUpdateVec4Slot<Vec4Slot6>;
pub type SysUpdateVec4Slot7 = SysUpdateVec4Slot<Vec4Slot7>;
pub type SysUpdateVec4Slot8 = SysUpdateVec4Slot<Vec4Slot8>;