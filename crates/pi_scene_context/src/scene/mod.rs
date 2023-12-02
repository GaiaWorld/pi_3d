
use pi_engine_shell::prelude::*;


use crate::object::sys_dispose_ready;

use self::{
    environment::{sys::*, brdf::*, environment_texture::*},
    command_sys::*,
    prelude::*,
    system::*,
};

pub mod coordinate_system;
pub mod command;
pub mod command_sys;
pub mod interface;
pub mod environment;
pub mod light;
pub mod passes_cfg;
mod base;
mod system;
mod pass_render_target;
pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageScene {
    SceneCreate,
    SceneCreateAplly,
    SceneCommand,
    SceneTextureRequest,
    SceneTextureLoaded,
}

pub struct PluginScene;
impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        let id = app.world.spawn_empty().id();
        app.insert_resource(SingleEmptyEntity::new(id));

        app.insert_resource(ActionListSceneCreate::default());
        app.insert_resource(ActionListSceneTime::default());
        app.insert_resource(ActionListSceneAmbientColor::default());
        app.insert_resource(ActionListSceneAmbientIntensity::default());
        app.insert_resource(ActionListSceneFogColor::default());
        app.insert_resource(ActionListSceneFogParam::default());
        app.insert_resource(ActionListSceneAnimationEnable::default());
        app.insert_resource(ActionListSceneBRDF::default());
        app.insert_resource(ActionListSceneOpaqueTexture::default());
        app.insert_resource(ActionListSceneDepthTexture::default());
        app.insert_resource(ActionListSceneEnvTexture::default());
        app.insert_resource(ActionListSceneShadowMap::default());
        
        app.insert_resource(ImageTextureViewLoader::<BRDFTextureSlot>::default());
        app.insert_resource(ImageTextureViewLoader::<EnvTextureSlot>::default());

        app.configure_sets(
            Update,
            (
                StageScene::SceneCreate,
                StageScene::SceneCreateAplly,
                StageScene::SceneCommand.after(ERunStageChap::_InitialApply)
            ).chain()
        );

        app.configure_set(Update, StageScene::SceneTextureRequest.after(StageTextureLoad::TextureRequest).before(StageTextureLoad::TextureLoading));
        app.configure_set(Update, StageScene::SceneTextureLoaded.after(StageTextureLoad::TextureLoaded).before(ERunStageChap::Uniform));
        app.add_systems(
            Update,
            (
                sys_env_texture_load_launch,
                sys_image_texture_view_load_launch::<BRDFTextureSlot, BRDFTexture>
            ).in_set(StageScene::SceneTextureRequest)
        );
        app.add_systems(
            Update,
            (
                sys_env_texture_loaded_check,
                sys_image_texture_view_loaded_check::<BRDFTextureSlot, BRDFTexture>,
            ).in_set(StageScene::SceneTextureLoaded)
        );

        app.add_systems(Update, 
            sys_create_scene.in_set(ERunStageChap::Initial)
        );
        
        app.add_systems(
			Update,
            (
                sys_act_scene_time,
                sys_act_scene_ambientcolor,
                sys_act_scene_ambientintensity,
                sys_act_scene_fogcolor,
                sys_act_scene_fogparam,
                sys_act_scene_animation_enable,
                sys_act_scene_brdf,
                sys_act_scene_env_texture,
                sys_act_scene_opaque_target,
                sys_act_scene_depth_target,
                sys_act_scene_shadowmap,
            ).in_set(StageScene::SceneCommand)
        );

        app.add_systems(
			Update,
            (
                sys_bind_update_scene_ambient,
                sys_bind_update_scene_fog,
                sys_bind_update_scene_time
            ).in_set(ERunStageChap::Uniform)
        );

        app.add_systems(Update, sys_dispose_about_scene.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));
    }
    
}
