
use pi_engine_shell::prelude::*;

use crate::{object::sys_dispose_ready, prelude::StageTransform};

use self::{sys::*, command::*, command_sys::*};

mod bone;
mod skeleton;
// pub mod skin_texture;
mod skin_buffer;
mod sys;
mod command;
pub mod command_sys;
mod interface;
pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageSkeleton {
    Command,
    Calc,
}

pub struct PluginSkeleton;
impl Plugin for PluginSkeleton {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ActionListSkinCreate::default());
        app.insert_resource(ActionListSkinUse::default());
        app.insert_resource(ActionListBoneCreate::default());
        app.insert_resource(ActionListBonePose::default());

        app.configure_set(Update, StageSkeleton::Command.after(ERunStageChap::_InitialApply).before(ERunStageChap::Uniform));
        app.configure_set(Update, StageSkeleton::Calc.after(ERunStageChap::Command).after(StageTransform::TransformCalcMatrix).before(ERunStageChap::Uniform));
        // app.configure_set(Update, StageSkeleton::Command.after(ERunStageChap::_InitialApply));

        app.add_systems(
			Update,
            (
                sys_create_bone,
                sys_create_skin,
            ).chain().in_set(ERunStageChap::Initial)
        );
        app.add_systems(
			Update,
            (
                sys_act_skin_use,
                sys_act_bone_pose,
                sys_bones_initial
            ).chain().in_set(StageSkeleton::Command)
        );
        
        // app.add_systems(Update, 
        //     sys_act_skin_use.in_set(ERunStageChap::SecondInitial)
        // );
        // app.add_systems(Update, 
        //     sys_bones_initial.in_set(ERunStageChap::Command),
        // );
        app.add_systems(
			Update,
            (
                sys_skin_dirty_by_bone,
                sys_skin_buffer_update,
            ).chain().in_set(StageSkeleton::Calc)
        );
        app.add_systems(Update, sys_dispose_about_skeleton.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     world.insert_resource(SingleSkinCreateCommands::default());
    //     world.insert_resource(SingleSkinModifyCommands::default());

    //     SysSkinCreateCommand::setup(world, stages.query_stage::<SysSkinCreateCommand>(ERunStageChap::Initial));
    //     SysSkinModifyCommand::setup(world, stages.query_stage::<SysSkinModifyCommand>(ERunStageChap::Initial));

    //     SysSkinDirtyByBonesMatrix::setup(world, stages.query_stage::<SysSkinDirtyByBonesMatrix>(ERunStageChap::Command));
    //     SysSkinTextureUpdate::setup(world, stages.query_stage::<SysSkinTextureUpdate>(ERunStageChap::Command));
        
    //     Ok(())
    // }
}