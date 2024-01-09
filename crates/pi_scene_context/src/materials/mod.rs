
use pi_assets::asset::GarbageEmpty;
use pi_engine_shell::prelude::*;


use crate::{object::sys_dispose_ready, shadow::prelude::*};

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
    fn build(&self, app: &mut bevy::prelude::App) {
        {
            app.insert_resource(ImageTextureLoader::default());
            app.insert_resource(StateTextureLoader::default());

            app.configure_set(Update, StageTextureLoad::TextureRequest);
            app.configure_set(Update, StageTextureLoad::TextureLoading.after(StageTextureLoad::TextureRequest));
            app.configure_set(Update, StageTextureLoad::TextureLoaded.after(StageTextureLoad::TextureLoading).before(ERunStageChap::Uniform));
            app.add_systems(
                Update,
                (
                    sys_image_texture_load_launch,
                    sys_image_texture_loaded
                ).chain().in_set(StageTextureLoad::TextureLoading)
            );
            app.insert_resource(ImageTextureViewLoader::<TextureSlot01>::default());
            app.insert_resource(ImageTextureViewLoader::<TextureSlot02>::default());
            app.insert_resource(ImageTextureViewLoader::<TextureSlot03>::default());
            app.insert_resource(ImageTextureViewLoader::<TextureSlot04>::default());
            app.insert_resource(ImageTextureViewLoader::<TextureSlot05>::default());
            app.insert_resource(ImageTextureViewLoader::<TextureSlot06>::default());
            app.insert_resource(ImageTextureViewLoader::<TextureSlot07>::default());
            app.insert_resource(ImageTextureViewLoader::<TextureSlot08>::default());
            app.add_systems(
                Update,
                (
                    sys_image_texture_view_load_launch::<TextureSlot01, EffectBindTexture2D01Comp>,
                    sys_image_texture_view_load_launch::<TextureSlot02, EffectBindTexture2D02Comp>,
                    sys_image_texture_view_load_launch::<TextureSlot03, EffectBindTexture2D03Comp>,
                    sys_image_texture_view_load_launch::<TextureSlot04, EffectBindTexture2D04Comp>,
                    sys_image_texture_view_load_launch::<TextureSlot05, EffectBindTexture2D05Comp>,
                    sys_image_texture_view_load_launch::<TextureSlot06, EffectBindTexture2D06Comp>,
                    sys_image_texture_view_load_launch::<TextureSlot07, EffectBindTexture2D07Comp>,
                    sys_image_texture_view_load_launch::<TextureSlot08, EffectBindTexture2D08Comp>,
                ).chain().in_set(StageTextureLoad::TextureRequest)
            );
            app.add_systems(
                Update,
                (
                    sys_image_texture_view_loaded_check::<TextureSlot01, EffectBindTexture2D01Comp>,
                    sys_image_texture_view_loaded_check::<TextureSlot02, EffectBindTexture2D02Comp>,
                    sys_image_texture_view_loaded_check::<TextureSlot03, EffectBindTexture2D03Comp>,
                    sys_image_texture_view_loaded_check::<TextureSlot04, EffectBindTexture2D04Comp>,
                    sys_image_texture_view_loaded_check::<TextureSlot05, EffectBindTexture2D05Comp>,
                    sys_image_texture_view_loaded_check::<TextureSlot06, EffectBindTexture2D06Comp>,
                    sys_image_texture_view_loaded_check::<TextureSlot07, EffectBindTexture2D07Comp>,
                    sys_image_texture_view_loaded_check::<TextureSlot08, EffectBindTexture2D08Comp>,
                ).chain().in_set(StageTextureLoad::TextureLoaded)
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
        // app.insert_resource(ActionListUniformInt::default());
        app.insert_resource(ActionListUniformUint::default());
        app.insert_resource(ActionListUniformVec2::default());
        app.insert_resource(ActionListUniformVec3::default());
        app.insert_resource(ActionListUniformVec4::default());
        // app.insert_resource(ActionListUniformMat2::default());
        app.insert_resource(ActionListUniformMat4::default());
        app.insert_resource(ActionListUniformTexture::default());
        app.insert_resource(ActionListUniformTextureFromRenderTarget::default());
        app.insert_resource(ActionListTargetAnimationUniform::default());
        app.insert_resource(StateMaterial::default());

        app.configure_set(Update, StageMaterial::Create.after(StageShadowGenerator::Create));
        app.configure_set(Update, StageMaterial::_Init.after(StageMaterial::Create));
        app.configure_set(Update, StageMaterial::Command.after(StageMaterial::_Init).before(StageTextureLoad::TextureRequest).before(EStageAnimation::Create).before(EStageAnimation::Running));
        app.configure_set(Update, StageMaterial::Ready.after(StageMaterial::Command).after(StageTextureLoad::TextureLoaded).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageMaterial::_Init));

        app.add_systems(
			Update,
            (
                sys_create_material,
            ).in_set(StageMaterial::Create)
        );

        app.add_systems(Update, 
            (
                sys_act_target_animation_uniform,
                sys_act_material_texture_from_target,
            ).in_set(StageMaterial::Command)
        );

        app.add_systems(Update, 
            (
                sys_act_material_use,
                sys_material_textures_modify,
            ).chain().in_set(StageMaterial::Command)
        );

        app.add_systems(
			Update,
            (
                // sys_act_uniform,
                // sys_act_uniform_by_name,
                sys_act_material_value.after(sys_act_target_animation_uniform),
                // sys_act_material_mat2.run_if(should_run),
                // sys_act_material_vec4,
                // sys_act_material_vec2,
                // sys_act_material_float,
                // sys_act_material_int.run_if(should_run),
                // sys_act_material_uint,
                sys_act_material_texture,
            ).before(sys_material_textures_modify).after(sys_act_material_texture_from_target).chain().in_set(StageMaterial::Command)
            // .after(sys_sync_load_check_await::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>)
        );

        app.add_systems(
			Update,
            (
                sys_texture_ready07,
            ).chain().in_set(StageMaterial::Ready)
        );
        app.add_systems(Update, 
                sys_material_uniform_apply.in_set(ERunStageChap::Uniform)
        );

        app.add_systems(Update, 
            sys_dispose_about_material.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );
    }
}

pub struct PluginGroupMaterial;
impl PluginGroupMaterial {
    pub fn add(group: PluginGroupBuilder) -> PluginGroupBuilder {
        group.add(PluginMaterial)
    }
}