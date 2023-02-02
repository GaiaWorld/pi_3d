use pi_ecs::prelude::Setup;
use pi_engine_shell::{plugin::Plugin, run_stage::ERunStageChap, object::ObjectID};

use self::{sys::{SysSkinTextureUpdate, SysSkinDirtyByBonesMatrix}, command::{SysSkinCreateCommand, SysSkinModifyCommand, SingleSkinCreateCommands, SingleSkinModifyCommands}};

pub mod bone;
pub mod skeleton;
pub mod skin_texture;
pub mod skin_buffer;
pub mod sys;
pub mod command;
pub mod interface;

pub struct SkeletonID(pub ObjectID);
pub struct SkeletonBonesDirty(pub bool);

pub struct PluginSkeleton;
impl Plugin for PluginSkeleton {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleSkinCreateCommands::default());
        world.insert_resource(SingleSkinModifyCommands::default());

        SysSkinCreateCommand::setup(world, stages.query_stage::<SysSkinCreateCommand>(ERunStageChap::Initial));
        SysSkinModifyCommand::setup(world, stages.query_stage::<SysSkinModifyCommand>(ERunStageChap::Initial));

        SysSkinDirtyByBonesMatrix::setup(world, stages.query_stage::<SysSkinDirtyByBonesMatrix>(ERunStageChap::Command));
        SysSkinTextureUpdate::setup(world, stages.query_stage::<SysSkinTextureUpdate>(ERunStageChap::Command));
        
        Ok(())
    }
}