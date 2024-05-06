
use pi_scene_shell::prelude::*;


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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StageScene {
    Create,
    _Insert,
    Command,
    TextureRequest,
    TextureLoaded,
}

pub struct PluginScene;
impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        let i: pi_world::insert::Inserter<()> = app.world.make_inserter();
        let id = i.insert(());
        // let id = app.world.spawn_empty().id();
        app.world.insert_single_res(SingleEmptyEntity::new(id));

        app.world.insert_single_res(ActionListSceneCreate::default());
        app.world.insert_single_res(ActionListSceneTime::default());
        app.world.insert_single_res(ActionListSceneAmbientColor::default());
        app.world.insert_single_res(ActionListSceneFogParam::default());
        app.world.insert_single_res(ActionListSceneAnimationEnable::default());
        app.world.insert_single_res(ActionListSceneBRDF::default());
        app.world.insert_single_res(ActionListSceneOpaqueTexture::default());
        app.world.insert_single_res(ActionListSceneDepthTexture::default());
        app.world.insert_single_res(ActionListSceneEnvTexture::default());
        app.world.insert_single_res(ActionListSceneShadowMap::default());
        
        app.world.insert_single_res(ImageTextureViewLoader::<BRDFTextureSlot>::default());
        app.world.insert_single_res(ImageTextureViewLoader::<EnvTextureSlot>::default());

        // app.configure_sets(
        //     Update,
        //     (
        //         StageScene::Create.after(ERunStageChap::_InitialApply),
        //         StageScene::_Insert.before(EStageAnimation::Create),
        //         StageScene::Command
        //     ).chain()
        // );

        // app.configure_set(Update, StageScene::TextureRequest.in_set(FrameDataPrepare).after(StageTextureLoad::TextureRequest).before(StageTextureLoad::TextureLoading));
        // app.configure_set(Update, StageScene::TextureLoaded.in_set(FrameDataPrepare).after(StageTextureLoad::TextureLoaded).before(ERunStageChap::Uniform));
        // app.add_system(Update, apply_deferred.in_set(StageScene::_Insert));

        app.add_system(
            Update,
                sys_image_texture_view_load_launch::<BRDFTextureSlot, BRDFTexture>
        );
        app.add_system(
            Update,
                sys_env_texture_load_launch
        );
        app.add_system(
            Update,
                sys_env_texture_loaded_check,
        );
        app.add_system(
            Update,
                sys_image_texture_view_loaded_check::<BRDFTextureSlot, BRDFTexture>,
        );

        app.add_system(Update, 
            sys_create_scene
        );
        
        app.add_system(
			Update,
                sys_act_scene_ambient,
        );
        app.add_system(
			Update,
                sys_act_scene_render,
        );

        app.add_system(
			Update,
                sys_bind_update_scene_ambient,
        );

        app.add_system(Update, sys_dispose_about_scene);
        app.add_system(Update, sys_dispose_ready);
    }
    
}
