use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, ResMut, }, query::{Changed, Write}};
use pi_ecs_macros::{setup};
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::Uniform};
use pi_scene_context::resources::RenderDynUniformBuffer;

use crate::{uniforms::{mat2::{Mat2Slot1, Mat2Slot3, Mat2Slot2, Mat2Slot4, Mat2Slot5, Mat2Slot6, Mat2Slot7, Mat2Slot8}, }, value::{FromValueUniformStatistics, SlotActiveRequire}};

use super::value_uniform::{ValueUniformStatistics, ValueUniformDynBindOffset};


pub struct SysMaterialChangeMat2Slot<D: FromValueUniformStatistics + SlotActiveRequire>(PhantomData<D>);
#[setup]
impl<D> SysMaterialChangeMat2Slot<D>
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

pub struct SysUpdateMat2Slot<D: FromValueUniformStatistics + Uniform>(PhantomData<D>);
#[setup]
impl<D> SysUpdateMat2Slot<D>
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

pub type SysMaterialChangeMat2Slot1 = SysMaterialChangeMat2Slot<Mat2Slot1>;
pub type SysMaterialChangeMat2Slot2 = SysMaterialChangeMat2Slot<Mat2Slot2>;
pub type SysMaterialChangeMat2Slot3 = SysMaterialChangeMat2Slot<Mat2Slot3>;
pub type SysMaterialChangeMat2Slot4 = SysMaterialChangeMat2Slot<Mat2Slot4>;
pub type SysMaterialChangeMat2Slot5 = SysMaterialChangeMat2Slot<Mat2Slot5>;
pub type SysMaterialChangeMat2Slot6 = SysMaterialChangeMat2Slot<Mat2Slot6>;
pub type SysMaterialChangeMat2Slot7 = SysMaterialChangeMat2Slot<Mat2Slot7>;
pub type SysMaterialChangeMat2Slot8 = SysMaterialChangeMat2Slot<Mat2Slot8>;


pub type SysUpdateMat2Slot1 = SysUpdateMat2Slot<Mat2Slot1>;
pub type SysUpdateMat2Slot2 = SysUpdateMat2Slot<Mat2Slot2>;
pub type SysUpdateMat2Slot3 = SysUpdateMat2Slot<Mat2Slot3>;
pub type SysUpdateMat2Slot4 = SysUpdateMat2Slot<Mat2Slot4>;
pub type SysUpdateMat2Slot5 = SysUpdateMat2Slot<Mat2Slot5>;
pub type SysUpdateMat2Slot6 = SysUpdateMat2Slot<Mat2Slot6>;
pub type SysUpdateMat2Slot7 = SysUpdateMat2Slot<Mat2Slot7>;
pub type SysUpdateMat2Slot8 = SysUpdateMat2Slot<Mat2Slot8>;