
use pi_engine_shell::prelude::*;

use crate::{
    transforms::prelude::*,
    object::sys_dispose_ready,
};

use self::{
    command::*,
    command_sys::*, system::*, prelude::*,
};

mod base;
mod point;
mod spot;
mod hemisphere;
mod vertex;
mod command;
mod command_sys;
mod interface;
mod system;
pub mod prelude;

pub struct PluginLighting;
impl Plugin for PluginLighting {
    fn build(&self, app: &mut App) {
        // app.world.insert_resource(SingleLightCreateCommands::default());
        app.insert_resource(ActionListLightCreate::default());
        app.insert_resource(ActionListLightParam::default());
        app.insert_resource(ActionListLightColor::default());
        app.insert_resource(ActionListSpotLightAngle::default());
        app.insert_resource(ActionListLightStrength::default());
        app.insert_resource(ActionListLightRadius::default());
        app.insert_resource(StateLight::default());
        
        app.configure_set(Update, StageLighting::LightingCommand.after(ERunStageChap::_InitialApply));
        app.configure_set(Update, StageLighting::LightingCommandApply.after(StageLighting::LightingCommand));
        app.configure_set(Update, StageLighting::LightingUniform.after(StageLighting::LightingCommandApply).after(StageTransform::TransformCalcMatrix).before(ERunStageChap::Uniform));
        // app.configure_set(Update, StageLighting::LightingCalcMatrix.after(StageLighting::LightingCommandApply).after(StageTransform::TransformCalcMatrix));
        // app.configure_set(Update, StageLighting::LightingCulling.after(StageLighting::LightingCalcMatrix).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageLighting::LightingCommandApply));

        app.insert_resource(SceneLightLimit(LightLimitInfo { max_direct_light_count: 8, max_point_light_count: 256, max_spot_light_count: 128, max_hemi_light_count: 16 }));
        app.insert_resource(ModelLightLimit(LightLimitInfo { max_direct_light_count: 4, max_point_light_count: 16, max_spot_light_count: 16, max_hemi_light_count: 4 }));
    
        app.insert_resource(SceneShadowLimit(
            ShadowLimitInfo { max_count: 1, max_width: 1024, max_height: 1024, color_format: ColorFormat::Rgba16Float, depth_stencil_format: DepthStencilFormat::Depth32Float }
        ));

        // app.add_systems(Update, sys_cmd_light_create.in_set(ERunStageChap::Initial));
        // app.add_systems(Update, sys_cmd_light_modify.in_set(ERunStageChap::Command));
        // app.add_systems(Update, sys_light_render_modify.in_set(ERunStageChap::Command));
        app.add_systems(
			Update,
            (
                sys_create_light,
            ).chain().in_set(ERunStageChap::Initial)
        );
        app.add_systems(
			Update,
            (
                sys_light_index_create,
                sys_act_light_param,
                sys_act_light_color,
                sys_act_spot_light_angle,
                sys_act_light_radius,
                sys_act_light_strength,
            ).chain().in_set(StageLighting::LightingCommand)
        );
        app.add_systems(
			Update,
            (
                sys_direct_light_update,
                sys_spot_light_update,
                sys_point_light_update,
                sys_hemi_light_update,
            ).chain().in_set(StageLighting::LightingUniform)
        );
        

        app.add_systems(Update, sys_dispose_about_light.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));

        // app.add_systems(Startup, setup);
    }
}
