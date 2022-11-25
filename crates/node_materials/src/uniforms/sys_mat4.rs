use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, ResMut, }, query::{Changed, Write}};
use pi_ecs_macros::{setup};
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::Uniform};
use pi_scene_context::resources::RenderDynUniformBuffer;

use crate::{uniforms::{mat4::{Mat4Slot1, Mat4Slot3, Mat4Slot2, Mat4Slot4, Mat4Slot5, Mat4Slot6, Mat4Slot7, Mat4Slot8}, mat2::UniformMat2, vec4::UniformVec4, vec2::UniformVec2, float::UniformFloat, int::UniformInt, uint::UniformUint}, value::{FromValueUniformStatistics, SlotActiveRequire}};

use super::value_uniform::{ValueUniformStatistics, ValueUniformDynBindOffset};


pub struct SysMaterialChangeMat4Slot<D: FromValueUniformStatistics + SlotActiveRequire>(PhantomData<D>);
#[setup]
impl<D> SysMaterialChangeMat4Slot<D>
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

pub struct SysUpdateMat4Slot<D: FromValueUniformStatistics + Uniform>(PhantomData<D>);
#[setup]
impl<D> SysUpdateMat4Slot<D>
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

pub type SysMaterialChangeMat4Slot1 = SysMaterialChangeMat4Slot<Mat4Slot1>;
pub type SysMaterialChangeMat4Slot2 = SysMaterialChangeMat4Slot<Mat4Slot2>;
pub type SysMaterialChangeMat4Slot3 = SysMaterialChangeMat4Slot<Mat4Slot3>;
pub type SysMaterialChangeMat4Slot4 = SysMaterialChangeMat4Slot<Mat4Slot4>;
pub type SysMaterialChangeMat4Slot5 = SysMaterialChangeMat4Slot<Mat4Slot5>;
pub type SysMaterialChangeMat4Slot6 = SysMaterialChangeMat4Slot<Mat4Slot6>;
pub type SysMaterialChangeMat4Slot7 = SysMaterialChangeMat4Slot<Mat4Slot7>;
pub type SysMaterialChangeMat4Slot8 = SysMaterialChangeMat4Slot<Mat4Slot8>;


pub type SysUpdateMat4Slot1 = SysUpdateMat4Slot<Mat4Slot1>;
pub type SysUpdateMat4Slot2 = SysUpdateMat4Slot<Mat4Slot2>;
pub type SysUpdateMat4Slot3 = SysUpdateMat4Slot<Mat4Slot3>;
pub type SysUpdateMat4Slot4 = SysUpdateMat4Slot<Mat4Slot4>;
pub type SysUpdateMat4Slot5 = SysUpdateMat4Slot<Mat4Slot5>;
pub type SysUpdateMat4Slot6 = SysUpdateMat4Slot<Mat4Slot6>;
pub type SysUpdateMat4Slot7 = SysUpdateMat4Slot<Mat4Slot7>;
pub type SysUpdateMat4Slot8 = SysUpdateMat4Slot<Mat4Slot8>;