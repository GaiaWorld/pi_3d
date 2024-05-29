
use pi_bevy_render_plugin::param::Assign;
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
        app.world.insert_single_res(ActionListSkinCreate::default());
        app.world.insert_single_res(ActionListSkinUse::default());
        app.world.insert_single_res(ActionListBoneCreate::default());
        app.world.insert_single_res(ActionListBonePose::default());

        app.configure_set(Update, StageSkeleton::SkinCreate.after(StageModel::CreateMesh));
        app.configure_set(Update, StageSkeleton::_SkinCreate.after(StageSkeleton::SkinCreate).before(StageTransform::TransformCommand));
        app.configure_set(Update, StageSkeleton::Command.in_set(FrameDataPrepare).after(StageSkeleton::_SkinCreate).before(ERunStageChap::Uniform));
        app.configure_set(Update, StageSkeleton::Calc.in_set(FrameDataPrepare).after(StageSkeleton::Command).after(StageTransform::TransformCalcMatrix).before(ERunStageChap::Uniform));
        // app.add_system(Update, apply_deferred.in_set(StageSkeleton::_SkinCreate));

        // app.add_system(
		// 	Update,
        //     (
        //         sys_create_skin.after(sys_create_mesh),
        //         sys_create_bone,
        //     ).chain().in_set(StageSkeleton::SkinCreate)
        // );
        app.add_system(Update,sys_create_skin.after(sys_create_mesh).in_set(StageSkeleton::SkinCreate));
        app.add_system(Update,sys_create_bone.after(sys_create_skin).in_set(StageSkeleton::SkinCreate));

        // app.add_system(
		// 	Update,
        //     (
        //         sys_act_skin_use,
        //         sys_act_bone_pose,
        //         sys_bones_initial
        //     ).chain().in_set(StageSkeleton::Command)
        // );
        app.add_system(Update,sys_act_skin_use.in_set(StageSkeleton::Command));
        app.add_system(Update,sys_act_bone_pose.after(sys_act_skin_use).in_set(StageSkeleton::Command));
        app.add_system(Update,sys_bones_initial.after(sys_act_bone_pose).in_set(StageSkeleton::Command));

        // app.add_system(
		// 	Update,
        //     (
        //         sys_skin_dirty_by_bone,
        //         sys_skin_buffer_update,
        //     ).chain().in_set(StageSkeleton::Calc)
        // );
        app.add_system(Update,sys_skin_dirty_by_bone.in_set(StageSkeleton::Calc));
        app.add_system(Update,sys_skin_buffer_update.after(sys_skin_dirty_by_bone).in_set(StageSkeleton::Calc));

        app.add_system(Update, sys_dispose_about_skeleton.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));
    }
}