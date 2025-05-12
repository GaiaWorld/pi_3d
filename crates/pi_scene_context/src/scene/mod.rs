
use pi_scene_shell::prelude::*;


use crate::object::{sys_dispose_ready, TmpCommonVec, TmpSortDrawOpaqueVec, TmpSortDrawTransparentVec};

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
    _SceneCreate,
    SceneCommand,
    SceneDispose,
    SceneTextureRequest,
    SceneTextureLoaded,
}

pub struct PluginScene;
impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "use_bevy")]
        {
            let id = app.world.spawn_empty_id();
            app.insert_resource(SingleEmptyEntity::new(id));
        }
        #[cfg(not(feature = "use_bevy"))]
        app.insert_resource(SingleEmptyEntity::new(Entity::default()));

        app.insert_resource(ActionListSceneCreate::default());
        app.insert_resource(ActionListSceneOption::default());
        
        app.insert_resource(ImageTextureViewLoader::<BRDFTextureSlot>::default());
        app.insert_resource(ImageTextureViewLoader::<EnvTextureSlot>::default());
        app.insert_resource(TmpCommonVec::default());
        app.insert_resource(TmpSortDrawOpaqueVec::default());
        app.insert_resource(TmpSortDrawTransparentVec::default());

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update,
            (
                StageScene::SceneCreate.after(ERunStageChap::Modify),
                StageScene::_SceneCreate.before(EStageAnimation::Create).after(StageScene::SceneCreate),
                StageScene::SceneCommand.after(StageScene::_SceneCreate),
                StageScene::SceneTextureRequest.in_set(FrameDataPrepare).after(StageTextureLoad::TextureRequest).before(StageTextureLoad::TextureLoading),
                StageScene::SceneTextureLoaded.in_set(FrameDataPrepare).after(StageTextureLoad::TextureLoaded).before(ERunStageChap::Collect),
            )
        );

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update,
            (
                apply_deferred.in_set(StageScene::_SceneCreate),
                (
                    sys_env_texture_load_launch,
                    sys_image_texture_view_load_launch::<BRDFTextureSlot, BRDFTexture>
                ).in_set(StageScene::SceneTextureRequest),
                (
                    sys_env_texture_loaded_check,
                    sys_image_texture_view_loaded_check::<BRDFTextureSlot, BRDFTexture>,
                ).in_set(StageScene::SceneTextureLoaded),
                sys_create_scene.in_set(StageScene::SceneCreate),
                (
                    sys_act_scene_ambient,
                ).in_set(StageScene::SceneCommand),
                (
                    sys_bind_update_scene_ambient,
                ).in_set(ERunStageChap::Collect),
                sys_dispose_about_scene.after(sys_dispose_ready).in_set(ERunStageChap::Dispose),
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
            .configure_set(StageD3, StageScene::SceneCreate           .in_set(ERunStageChap::Create))
            .configure_set(StageD3, StageScene::_SceneCreate          .in_set(ERunStageChap::Create).after(StageScene::SceneCreate))
            .configure_set(StageD3, StageScene::SceneCommand          .in_set(ERunStageChap::Modify))
            .configure_set(StageD3, StageScene::SceneTextureRequest   .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageScene::SceneCommand).after(StageTextureLoad::TextureRequest).before(StageTextureLoad::TextureLoading))
            .configure_set(StageD3, StageScene::SceneTextureLoaded    .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageTextureLoad::TextureLoaded))
            .configure_set(StageD3, StageScene::SceneDispose           .in_set(ERunStageChap::Dispose))
            ;

#[cfg(not(feature = "use_bevy"))]
        app
            .add_systems(Update, sys_env_texture_load_launch                                         .in_set(StageScene::SceneTextureRequest))
            .add_systems(Update, sys_image_texture_view_load_launch::<BRDFTextureSlot, BRDFTexture>  .in_set(StageScene::SceneTextureRequest))
            .add_systems(Update, sys_env_texture_loaded_check                                        .in_set(StageScene::SceneTextureLoaded))
            .add_systems(Update, sys_image_texture_view_loaded_check::<BRDFTextureSlot, BRDFTexture> .in_set(StageScene::SceneTextureLoaded))
            .add_systems(Update, sys_create_scene
                // .run_if(runif_acts::<OpsSceneCreation>)        
                .in_set(StageScene::SceneCreate))
            .add_systems(Update, sys_act_scene_ambient
                // .run_if(runif_acts::<OpsSceneOption>)           
                .in_set(StageScene::SceneCommand))
            .add_systems(Update, sys_bind_update_scene_ambient   .in_set(ERunStageChap::Collect))
            .add_systems(Update, sys_dispose_about_scene             .after(sys_dispose_ready)       .in_set(StageScene::SceneDispose))
            ;
    }
    
}
