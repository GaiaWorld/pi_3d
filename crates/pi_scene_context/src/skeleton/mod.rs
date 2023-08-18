
use pi_engine_shell::prelude::*;

use crate::object::sys_dispose_ready;

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

pub struct PluginSkeleton;
impl Plugin for PluginSkeleton {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ActionListSkinCreate::default());
        app.insert_resource(ActionListSkinUse::default());
        app.insert_resource(ActionListBoneCreate::default());
        app.insert_resource(ActionListBonePose::default());
        app.add_systems(
			Update,
            (
                sys_act_skin_create,
            ).chain().in_set(ERunStageChap::Initial)
        );
        app.add_systems(
			Update,
            (
                sys_act_bone_create,
                sys_act_bone_pose
            ).chain().in_set(ERunStageChap::Initial)
        );
        
        app.add_system(
            sys_act_skin_use.in_set(ERunStageChap::SecondInitial)
        );
        app.add_system(
            sys_bones_initial.in_set(ERunStageChap::Command),
        );
        app.add_systems(
			Update,
            (
                sys_skin_dirty_by_bone,
                sys_skin_buffer_update,
            ).chain().in_set(ERunStageChap::Uniform)
        );
        app.add_system(sys_dispose_about_skeleton.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));
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