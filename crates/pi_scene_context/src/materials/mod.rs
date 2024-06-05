
use pi_scene_shell::prelude::*;


use crate::{object::sys_dispose_ready, prelude::StageModel, shadow::prelude::*};

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
            app.insert_resource(ImageTextureLoader::default());
            app.insert_resource(StateTextureLoader::default());
            app.insert_resource(ImageTextureViewLoader2::default());

            app.configure_sets(
                Update,
                (
                    StageTextureLoad::TextureRequest.in_set(FrameDataPrepare),
                    StageTextureLoad::TextureLoading.in_set(FrameDataPrepare).after(StageTextureLoad::TextureRequest),
                    StageTextureLoad::TextureLoaded.in_set(FrameDataPrepare).after(StageTextureLoad::TextureLoading).before(ERunStageChap::Uniform),
                )
            );

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
        }
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
        if app.world.get_resource::<ShareAssetMgr<ImageTexture>>().is_none() {
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ImageTexture>();
            app.insert_resource(
                ShareAssetMgr::<ImageTexture>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_resource::<ShareAssetMgr<ImageTextureView>>().is_none() {
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ImageTextureView>();
            app.insert_resource(
                ShareAssetMgr::<ImageTextureView>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };

        let defaulttextures = set_up_uniforms(
            &app.world.get_resource::<ShareAssetMgr<TextureRes>>().unwrap(),
            &app.world.get_resource::<PiRenderDevice>().unwrap(),
            &app.world.get_resource::<PiRenderQueue>().unwrap(),
        );
        app.insert_resource(defaulttextures);
        
        let entity = app.world.spawn_empty().id();
        let single = SingleIDBaseDefaultMaterial(entity);
        app.insert_resource(single);

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ShaderEffectMeta>();
        app.insert_resource(ShareAssetMgr::<ShaderEffectMeta>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout));

        app.insert_resource(ActionListMaterialCreate::default());
        app.insert_resource(ActionListMaterialUse::default());
        app.insert_resource(ActionListUniformFloat::default());
        app.insert_resource(ActionListUniformUint::default());
        app.insert_resource(ActionListUniformVec2::default());
        app.insert_resource(ActionListUniformVec3::default());
        app.insert_resource(ActionListUniformVec4::default());
        app.insert_resource(ActionListUniformMat4::default());
        app.insert_resource(ActionListUniformTexture::default());
        app.insert_resource(ActionListUniformTextureFromRenderTarget::default());
        app.insert_resource(ActionListTargetAnimationUniform::default());
        app.insert_resource(StateMaterial::default());

        app.configure_sets(
            Update,
            (
                StageMaterial::Create.after(StageShadowGenerator::_Create).after(StageModel::_InitMesh),
                StageMaterial::_Init.after(StageMaterial::Create),
                StageMaterial::Command.after(StageMaterial::_Init).before(StageTextureLoad::TextureRequest).before(EStageAnimation::Create).before(EStageAnimation::Running),
                StageMaterial::Ready.in_set(FrameDataPrepare).after(StageMaterial::Command).after(StageTextureLoad::TextureLoaded).before(ERunStageChap::Uniform),
            )
        );
        
        app.add_systems(
			Update,
            (
                (
                    sys_create_material,
                ).in_set(StageMaterial::Create),
                apply_deferred.in_set(StageMaterial::_Init),
                (
                    sys_act_material_use,
                    sys_act_target_animation_uniform,
                    sys_act_material_texture_from_target,

                    // sys_act_uniform,
                    // sys_act_uniform_by_name,
                    sys_act_material_value,
                    // sys_act_material_mat2.run_if(should_run),
                    // sys_act_material_vec4,
                    // sys_act_material_vec2,
                    // sys_act_material_float,
                    // sys_act_material_int.run_if(should_run),
                    // sys_act_material_uint,

                    sys_act_material_texture,
                    sys_material_textures_modify,
                ).chain().in_set(StageMaterial::Command),
                (
                    sys_texture_ready07,
                ).chain().in_set(StageMaterial::Ready),
                sys_material_uniform_apply.in_set(ERunStageChap::Uniform),
                sys_dispose_about_material.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );
    }
}

pub struct PluginGroupMaterial;
impl PluginGroupMaterial {
    pub fn add(app: &mut App) -> &mut App {
        app.add_plugins(PluginMaterial)
    }
}