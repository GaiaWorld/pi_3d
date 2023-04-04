
use pi_engine_shell::prelude::*;

use crate::{viewer::PluginViewer, animation::command::SingleModifyCommands, };

use self::{base::LightDirection, command::{SysLightModifyCommand, SysLightCreateCommand, SingleLightCreateCommands, SysLightModifyEffectRender}, directional::{system::SysDirectionalShadowModify, DirectionalShadowProjection}, shadow_generator::PluginShadowGenerator};

pub mod base;
pub mod directional;
pub mod point;
pub mod vertex;
pub mod command;
pub mod shadow_generator;
pub mod interface;

pub struct PluginLighting;
impl Plugin for PluginLighting {
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();
    //     world.insert_resource(SingleLightCreateCommands::default());
    //     world.insert_resource(SingleModifyCommands::default());

    //     SysLightCreateCommand::setup(world, stages.query_stage::<SysLightCreateCommand>(ERunStageChap::Initial));
    //     SysLightModifyCommand::setup(world, stages.query_stage::<SysLightModifyCommand>(ERunStageChap::Command));
    //     SysLightModifyEffectRender::setup(world, stages.query_stage::<SysLightModifyEffectRender>(ERunStageChap::Command));

    //     SysDirectionalShadowModify::setup(world, stages.query_stage::<SysDirectionalShadowModify>(ERunStageChap::Command));

    //     PluginShadowGenerator.init(engine, stages);

    //     PluginViewer::<LightDirection, SysLightModifyCommand, DirectionalShadowProjection, SysDirectionalShadowModify>::default().init(engine, stages);

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        app.world.insert_resource(SingleLightCreateCommands::default());
        app.world.insert_resource(SingleModifyCommands::default());

        app.add_system(sys_cmd_light_create.in_set(ERunStageChap::Initial));
        app.add_system(sys_cmd_light_modify.in_set(ERunStageChap::Command));
        app.add_system(sys_light_render_modify.in_set(ERunStageChap::Command));
        app.add_system(sys_directional_light_shadow_modify.in_set(ERunStageChap::Command));

        // SysLightCreateCommand::setup(world, stages.query_stage::<SysLightCreateCommand>(ERunStageChap::Initial));
        // SysLightModifyCommand::setup(world, stages.query_stage::<SysLightModifyCommand>(ERunStageChap::Command));
        // SysLightModifyEffectRender::setup(world, stages.query_stage::<SysLightModifyEffectRender>(ERunStageChap::Command));

        // SysDirectionalShadowModify::setup(world, stages.query_stage::<SysDirectionalShadowModify>(ERunStageChap::Command));

        app.add_plugin(PluginShadowGenerator);
        // PluginShadowGenerator.init(engine, stages);

        PluginViewer::<LightDirection, SysLightModifyCommand, DirectionalShadowProjection, SysDirectionalShadowModify>::default().init(engine, stages);
    }
}