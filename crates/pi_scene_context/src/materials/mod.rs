
use pi_bevy_render_plugin::GraphBuild;
use pi_scene_shell::prelude::*;


use crate::{object::sys_dispose_ready, prelude::StageModel, scene::StageScene, shadow::prelude::*};

use self::{
    command::*,
    command_sys::*,
    uniforms::set_up_uniforms,
    system::*,
    prelude::*
};

mod material;
mod uniforms;
mod value;
mod shader_effect;
mod command;
pub mod command_sys;
mod interface;
mod system;
pub mod prelude;

pub type MBKK = usize;


// type PluginAssetShaderEffectLoad = PluginAssetSyncLoad::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta, SysMaterailCreateCommands>;

struct PluginMaterial;
impl Plugin for PluginMaterial {
    fn build(&self, app: &mut App) {
        {
            app.insert_resource(TextureCombineAtlas2DMgr::default());
            app.insert_resource(ImageTextureLoader::default());
            app.insert_resource(StateTextureLoader::default());
            app.insert_resource(ImageTextureViewLoader2::default());

            app.configure_set(StageD3, StageTextureLoad::TextureRequest.in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageMaterial::MatCommand));
            app.configure_set(StageD3, StageTextureLoad::TextureLoading.in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageTextureLoad::TextureRequest));
            app.configure_set(StageD3, StageTextureLoad::TextureLoaded .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageTextureLoad::TextureLoading));

#[cfg(feature = "use_bevy")]
            app.add_systems(
                Update,
                (
                    (
                        sys_image_texture_load_launch,
                        sys_image_texture_loaded
                    ).chain().in_set(StageTextureLoad::TextureLoading),
                    (
                        sys_image_texture_view_load_launch2
                    ).chain().in_set(StageTextureLoad::TextureRequest),
                    (
                        sys_image_texture_view_loaded_check2
                    ).chain().in_set(StageTextureLoad::TextureLoaded),
                )
            );

#[cfg(not(feature = "use_bevy"))]
            app
                .add_systems(Update, sys_texture_combine                .in_set(StageTextureLoad::TextureLoading))
                .add_systems(Update, sys_image_texture_load_launch                                                   .in_set(StageTextureLoad::TextureLoading))
                .add_systems(Update, sys_image_texture_loaded        .after(sys_image_texture_load_launch)   .in_set(StageTextureLoad::TextureLoading))
                .add_systems(Update, sys_image_texture_view_load_launch2
                    // .run_if(runif_changes::<TextureKeyList>)         
                    .in_set(StageTextureLoad::TextureRequest))
                .add_systems(Update, sys_image_texture_view_loaded_check2        .in_set(StageTextureLoad::TextureLoaded))
                ;
        }

        if app.world.get_resource::<TextureCombineCmds>().is_none() {
            app.insert_resource(TextureCombineCmds::default());
        };
        if app.world.get_resource::<ShareAssetMgr<SamplerRes>>().is_none() {
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<SamplerRes>();
            app.insert_resource(
                ShareAssetMgr::<SamplerRes>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_resource::<ShareAssetMgr<TextureRes>>().is_none() {
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<TextureRes>();
            app.insert_resource(
                ShareAssetMgr::<TextureRes>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_resource::<ShareAssetMgr<ResImageTexture>>().is_none() {
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ResImageTexture>();
            app.insert_resource(
                ShareAssetMgr::<ResImageTexture>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
            app.insert_resource(
                ShareAssetMgr::<ImageTextureFrame>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_resource::<ShareAssetMgr<ImageTextureView>>().is_none() {
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ImageTextureView>();
            app.insert_resource(
                ShareAssetMgr::<ImageTextureView>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
            app.insert_resource(
                ShareAssetMgr::<ImageTextureViewFrame>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };

        let defaulttextures = set_up_uniforms(
            &app.world.get_resource::<ShareAssetMgr<TextureRes>>().unwrap(),
            &app.world.get_resource::<PiRenderDevice>().unwrap(),
            &app.world.get_resource::<PiRenderQueue>().unwrap(),
        );
        app.insert_resource(defaulttextures);
        
        let entity = app.world.spawn_empty_id();
        let single = SingleIDBaseDefaultMaterial(entity);
        app.insert_resource(single);

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ShaderEffectMeta>();
        app.insert_resource(ShareAssetMgr::<ShaderEffectMeta>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout));

        app.insert_resource(ActionListMaterialCreate::default());
        app.insert_resource(ActionListMaterialUse::default());
        app.insert_resource(ActionListUniformVal::default());
        app.insert_resource(ActionListUniformValB::default());
        app.insert_resource(StateMaterial::default());

        app.configure_set(StageD3, StageMaterial::MatCreate     .in_set(ERunStageChap::Create).after(StageShadowGenerator::_ShadowCreate).after(StageModel::_InitMesh));
        app.configure_set(StageD3, StageMaterial::_MatCreate    .in_set(ERunStageChap::Create).after(StageMaterial::MatCreate).before(ERunStageChap::Dispose));
        app.configure_set(StageD3, StageMaterial::MatUse         .in_set(ERunStageChap::Modify).before(StageMaterial::MatCommand));
        app.configure_set(StageD3, StageMaterial::MatCommand    .in_set(ERunStageChap::Modify).before(StageTextureLoad::TextureRequest).before(EStageAnimation::Create).before(EStageAnimation::Running).after(GraphBuild));
        app.configure_set(StageD3, StageMaterial::MatReady      .in_set(ERunStageChap::Culled).in_set(FrameDataPrepare).after(StageTextureLoad::TextureLoaded));
        app.configure_set(StageD3, StageMaterial::MatDispose     .in_set(ERunStageChap::Dispose).before(StageScene::SceneDispose));

#[cfg(feature = "use_bevy")]
        app.add_systems(
			Update,
            (
                (
                    sys_create_material,
                ).in_set(StageMaterial::MatCreate),
                apply_deferred.in_set(StageMaterial::_MatCreate),
                (
                    sys_act_material_use,
                    sys_act_material_value,
                    sys_material_textures_modify,
                ).chain().in_set(StageMaterial::MatCommand),
                (
                    sys_texture_ready,
                ).chain().in_set(StageMaterial::MatReady),
                sys_material_uniform_apply.in_set(ERunStageChap::Collect),
                sys_dispose_about_material.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
            .add_systems(Update, sys_create_material
                // .run_if(runif_acts::<OpsMaterialCreate>)             
                .in_set(StageMaterial::MatCreate) )
            .add_systems(Update, sys_act_material_use
                // .run_if(runif_acts::<OpsMaterialUse>)                               
                .in_set(StageMaterial::MatUse) )
            .add_systems(Update, sys_act_material_value                  .after(sys_act_material_use)   .in_set(StageMaterial::MatCommand) )
            .add_systems(Update, sys_material_textures_modify
                // .run_if(runif_comp::<UniformTextureWithSamplerParamsDirty>)
                .after(sys_act_material_value)                .in_set(StageMaterial::MatCommand) )
            .add_systems(Update, sys_texture_ready
                // .run_if(runif_comp::<EffectBindTexture2DList>)
                .in_set(StageMaterial::MatReady) )
            .add_systems(Update, sys_material_uniform_apply
                // .run_if(runif_comp::<TargetAnimatorableIsRunning>)
                .in_set(ERunStageChap::Collect) )
            .add_systems(Update, sys_dispose_about_material          .after(sys_dispose_ready)   .in_set(StageMaterial::MatDispose) )
            ;

    }
}

pub struct PluginGroupMaterial;
impl PluginGroupMaterial {
    pub fn add(app: &mut App) -> &mut App {
        app.add_plugins(PluginMaterial)
    }
}