
use pi_scene_shell::{prelude::*, run_stage::should_run_with_lighting};

use crate::{
    transforms::prelude::*,
    object::sys_dispose_ready, scene::StageScene, layer_mask::StageLayerMask, flags::StageEnable,
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
        app.insert_resource(ActionListLightCreate::default());
        app.insert_resource(ActionListLightParam::default());
        app.insert_resource(StateLight::default());

        app.insert_resource(SceneLightLimit(LightLimitInfo { max_direct_light_count: 8, max_point_light_count: 256, max_spot_light_count: 128, max_hemi_light_count: 16 }));
        app.insert_resource(ModelLightLimit(LightLimitInfo { max_direct_light_count: 4, max_point_light_count: 16, max_spot_light_count: 16, max_hemi_light_count: 4 }));

        app.insert_resource(SceneShadowLimit(
            ShadowLimitInfo { max_count: 1, max_width: 1024, max_height: 1024, color_format: ColorFormat::Rgba16Float, depth_stencil_format: DepthStencilFormat::Depth32Float }
        ));
        
        app.configure_sets(
            Update,
            (
                StageLighting::LightCreate.after(StageScene::_Create),
                StageLighting::_LightCreate.after(StageLighting::LightCreate).before(StageLayerMask::Command).before(StageEnable::Command).before(StageTransform::TransformCommand),
                StageLighting::LightingCommand.after(StageLighting::_LightCreate),
                StageLighting::LightingUniform.run_if(should_run_with_lighting).in_set(FrameDataPrepare).after(StageLighting::LightingCommand).after(EStageAnimation::Running).after(StageTransform::TransformCalcMatrix).before(ERunStageChap::Uniform),
            )
        );

        app.add_systems(Update,
            (
                apply_deferred.in_set(StageLighting::_LightCreate),
                sys_create_light.in_set(StageLighting::LightCreate),
                sys_act_light_param.in_set(StageLighting::LightingCommand),
                (
                    sys_direct_light_update,
                    sys_spot_light_update,
                    sys_point_light_update,
                    sys_hemi_light_update,
                ).chain().in_set(StageLighting::LightingUniform),
                sys_dispose_about_light.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );
    }
}
