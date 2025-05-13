
use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};

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

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update, 
            (
                StageSkeleton::SkinCreate.after(StageModel::_InitMesh),
                StageSkeleton::_SkinCreate.after(StageSkeleton::SkinCreate).before(StageTransform::TransformCommand),
                StageSkeleton::SkinCommand.in_set(FrameDataPrepare).after(StageSkeleton::_SkinCreate).before(ERunStageChap::Collect),
                StageSkeleton::SkinCalc.in_set(FrameDataPrepare).after(StageSkeleton::SkinCommand).after(StageTransform::TransformCalcMatrix).before(ERunStageChap::Collect),
            )
        );

        #[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(StageD3, StageSkeleton::SkinCreate    .in_set(ERunStageChap::Create).after(StageModel::_InitMesh))
        .configure_set(StageD3, StageSkeleton::_SkinCreate   .in_set(ERunStageChap::Create).after(StageSkeleton::SkinCreate).before(StageTransform::TransformCommand))
        .configure_set(StageD3, StageSkeleton::SkinCommand   .in_set(ERunStageChap::Modify).after(StageSkeleton::_SkinCreate))
        .configure_set(StageD3, StageSkeleton::SkinCalc      .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageSkeleton::SkinCommand).after(StageTransform::TransformCalcMatrix))
        .configure_set(StageD3, StageSkeleton::SkinDispose   .in_set(ERunStageChap::Dispose).before(StageTransform::TransformDispose))
        ;

        let enginepugins = app.world.get_resource::<EngineCustomPlugins>().unwrap();
        if enginepugins.skeleton {

            #[cfg(feature = "use_bevy")]
            app.add_systems(StageD3, 
                (
                    apply_deferred.in_set(StageSkeleton::_SkinCreate),
                    (
                        sys_create_bone,
                        sys_create_skin.after(sys_create_mesh),
                    ).chain().in_set(StageSkeleton::SkinCreate),
                    (
                        sys_act_skin_use,
                        // sys_act_bone_pose,
                        sys_bones_absolute
                    ).chain().in_set(StageSkeleton::SkinCommand),
                    (
                        sys_skin_dirty_by_bone,
                        sys_skin_buffer_update,
                    ).chain().in_set(StageSkeleton::SkinCalc),
                    sys_dispose_about_skeleton.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
                )
            );


            #[cfg(not(feature = "use_bevy"))]
            app
            .add_systems(StageD3, sys_create_bone.after(sys_create_mesh).in_set(StageSkeleton::SkinCreate))
            .add_systems(StageD3, sys_create_skin.after(sys_create_bone).in_set(StageSkeleton::SkinCreate))
            .add_systems(StageD3, sys_act_skin_use.in_set(StageSkeleton::SkinCommand))
            // .add_systems(StageD3, sys_act_bone_pose.after(sys_act_skin_use).in_set(StageSkeleton::Command))
            .add_systems(StageD3, sys_bones_absolute.after(sys_act_skin_use).in_set(StageSkeleton::SkinCommand))
            .add_systems(StageD3, sys_bones_local_dirty.in_set(StageSkeleton::SkinCalc))
            .add_systems(StageD3, sys_bones_worldmatrix.after(sys_bones_local_dirty).in_set(StageSkeleton::SkinCalc))
            .add_systems(StageD3, sys_skin_dirty_by_bone.after(sys_bones_worldmatrix).in_set(StageSkeleton::SkinCalc))
            .add_systems(StageD3, sys_skin_buffer_update.after(sys_skin_dirty_by_bone).in_set(StageSkeleton::SkinCalc))
            .add_systems(StageD3, sys_dispose_about_skeleton.after(sys_dispose_ready).in_set(StageSkeleton::SkinDispose))
            ;
        }
    }
}