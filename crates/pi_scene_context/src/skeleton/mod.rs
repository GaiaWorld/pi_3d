
use pi_scene_shell::prelude::*;

use crate::{object::sys_dispose_ready, prelude::{StageTransform, sys_create_mesh, StageModel}};

use self::{sys::*, command::*, command_sys::*, skeleton::*};

mod bone;
mod skeleton;
// pub mod skin_texture;
mod skin_buffer;
mod sys;
mod command;
pub mod command_sys;
mod interface;
pub mod prelude;


pub struct PluginSkeleton;
impl Plugin for PluginSkeleton {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListSkinCreate::default());
        app.insert_resource(ActionListSkinUse::default());
        app.insert_resource(ActionListBoneCreate::default());
        app.insert_resource(ActionListBonePose::default());

        app.configure_sets(Update, 
            (
                StageSkeleton::SkinCreate.after(StageModel::_InitMesh),
                StageSkeleton::_SkinCreate.after(StageSkeleton::SkinCreate).before(StageTransform::TransformCommand),
                StageSkeleton::Command.in_set(FrameDataPrepare).after(StageSkeleton::_SkinCreate).before(ERunStageChap::Uniform),
                StageSkeleton::Calc.in_set(FrameDataPrepare).after(StageSkeleton::Command).after(StageTransform::TransformCalcMatrix).before(ERunStageChap::Uniform),
            )
        );

        app.add_systems(Update, 
            (
                apply_deferred.in_set(StageSkeleton::_SkinCreate),
                (
                    sys_create_bone,
                    sys_create_skin.after(sys_create_mesh),
                ).chain().in_set(StageSkeleton::SkinCreate),
                (
                    sys_act_skin_use,
                    sys_act_bone_pose,
                    sys_bones_initial
                ).chain().in_set(StageSkeleton::Command),
                (
                    sys_skin_dirty_by_bone,
                    sys_skin_buffer_update,
                ).chain().in_set(StageSkeleton::Calc),
                sys_dispose_about_skeleton.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );
    }
}