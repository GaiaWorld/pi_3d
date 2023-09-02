
use pi_assets::asset::GarbageEmpty;
use pi_engine_shell::prelude::*;


use crate::object::sys_dispose_ready;

use self::{
    command::*,
    command_sys::*,
    uniforms::{
        sys_texture::*,
        sys_uniform::*,
        sys_pass::*,
        set_up_uniforms
    },
    system::sys_dispose_about_material,
    prelude::{SingleIDBaseDefaultMaterial, StageMaterial, StateMaterial}
};

mod material;
mod material_meta;
mod uniforms;
mod value;
mod shader_effect;
mod command;
pub mod command_sys;
mod interface;
mod system;
mod animation;
pub mod prelude;

pub type MBKK = usize;


// type PluginAssetShaderEffectLoad = PluginAssetSyncLoad::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta, SysMaterailCreateCommands>;

struct PluginMaterial;
impl Plugin for PluginMaterial {
    fn build(&self, app: &mut bevy::prelude::App) {
        if app.world.get_resource::<ShareAssetMgr<SamplerRes>>().is_none() {
            let cfg = asset_capacity::<AssetCfgSamplerRes>(app);
            app.insert_resource(
                ShareAssetMgr::<SamplerRes>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_resource::<ShareAssetMgr<TextureRes>>().is_none() {
            let cfg = asset_capacity::<AssetCfgTextureRes>(app);
            app.insert_resource(
                ShareAssetMgr::<TextureRes>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_resource::<ShareAssetMgr<ImageTexture>>().is_none() {
            let cfg = asset_capacity::<AssetCfgImageTexture>(app);
            app.insert_resource(
                ShareAssetMgr::<ImageTexture>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_resource::<ShareAssetMgr<ImageTextureView>>().is_none() {
            let cfg = asset_capacity::<AssetCfgImageTextureView>(app);
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
        // log::warn!("Default Maerial {:?}", scene);
        let single = SingleIDBaseDefaultMaterial(entity);
        app.insert_resource(single);

        let cfg = asset_capacity::<AssetCfgShaderMeta3D>(app);
        app.insert_resource(ShareAssetMgr::<ShaderEffectMeta>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout));

        // app.insert_resource(AssetSyncWait::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>::default());

        app.insert_resource(ActionListMaterialCreate::default());
        app.insert_resource(ActionListMaterialUse::default());
        // app.insert_resource(ActionListUniform::default());
        // app.insert_resource(ActionListUniformByName::default());
        app.insert_resource(ActionListUniformFloat::default());
        app.insert_resource(ActionListUniformInt::default());
        app.insert_resource(ActionListUniformUint::default());
        app.insert_resource(ActionListUniformVec2::default());
        app.insert_resource(ActionListUniformVec4::default());
        app.insert_resource(ActionListUniformMat2::default());
        app.insert_resource(ActionListUniformMat4::default());
        app.insert_resource(ActionListUniformTexture::default());
        app.insert_resource(StateMaterial::default());

        app.configure_set(Update, StageMaterial::MaterialUse.after(ERunStageChap::_InitialApply));
        app.configure_set(Update, StageMaterial::MaterialUseApply.after(StageMaterial::MaterialUse));
        app.configure_set(Update, StageMaterial::MaterialCommand.after(StageMaterial::MaterialUseApply));
        app.configure_set(Update, StageMaterial::MaterialCommandApply.after(StageMaterial::MaterialCommand).before(StageTextureLoad::TextureRequest));
        app.configure_set(Update, StageMaterial::MaterialReady.after(StageMaterial::MaterialCommandApply).after(StageTextureLoad::TextureLoaded).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageMaterial::MaterialUseApply));
        app.add_systems(Update, apply_deferred.in_set(StageMaterial::MaterialCommandApply));

        app.add_systems(
			Update,
            (
                sys_create_material,
            ).in_set(ERunStageChap::Initial)
        );
        
        app.add_systems(Update, 
            (
                sys_act_material_use,
            ).in_set(StageMaterial::MaterialUse)
        );

        app.add_systems(Update, 
            (
                sys_material_textures_modify,
            ).in_set(StageMaterial::MaterialCommand)
        );

        app.add_systems(
			Update,
            (
                // sys_act_uniform,
                // sys_act_uniform_by_name,
                sys_act_material_mat4,
                // sys_act_material_mat2.run_if(should_run),
                sys_act_material_vec4,
                sys_act_material_vec2,
                sys_act_material_float,
                // sys_act_material_int.run_if(should_run),
                sys_act_material_uint,
                sys_act_material_texture,
            ).before(sys_material_textures_modify).chain().in_set(StageMaterial::MaterialCommand)
            // .after(sys_sync_load_check_await::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>)
        );

        app.add_systems(
			Update,
            (
                sys_texture_ready07,
                sys_effect_bind_to_model_while_mat_modify,
                sys_effect_tex_to_model_while_mat_modify
            ).chain().in_set(StageMaterial::MaterialReady)
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
        group
            .add(PluginTextureSlot01Load::default())
            .add(PluginTextureSlot02Load::default())
            .add(PluginTextureSlot03Load::default())
            .add(PluginTextureSlot04Load::default())
            .add(PluginTextureSlot05Load::default())
            .add(PluginTextureSlot06Load::default())
            .add(PluginMaterial)
    }
}