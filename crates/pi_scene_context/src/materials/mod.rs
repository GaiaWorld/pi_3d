
use pi_assets::asset::GarbageEmpty;
use pi_engine_shell::{prelude::*};


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
    shader_effect::*, system::sys_dispose_about_material, prelude::SingleIDBaseDefaultMaterial
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

        app.insert_resource(AssetSyncWait::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>::default());

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

        app.add_systems(
            (
                sys_create_material.run_if(should_run),
                // sys_sync_load_create::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>,
                // sys_sync_load_check_await::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>,
            ).chain().in_set(ERunStageChap::Initial)
        );
        
        app.add_system(
            sys_act_material_use.run_if(should_run).in_set(ERunStageChap::SecondInitial)
        );

        app.add_systems(
            (
                // sys_act_uniform,
                // sys_act_uniform_by_name,
                sys_act_material_mat4.run_if(should_run),
                // sys_act_material_mat2.run_if(should_run),
                sys_act_material_vec4.run_if(should_run),
                sys_act_material_vec2.run_if(should_run),
                sys_act_material_float.run_if(should_run),
                // sys_act_material_int.run_if(should_run),
                sys_act_material_uint.run_if(should_run),
                sys_act_material_texture.run_if(should_run),
            ).in_set(ERunStageChap::SecondInitial)
            // .after(sys_sync_load_check_await::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>)
        );
        
        app.add_systems(
            (
                // sys_material_init,
                sys_material_textures_modify,
            ).in_set(ERunStageChap::Command)
        );
        app.add_systems(
            (
                sys_texture_ready01.run_if(should_run),
                sys_texture_ready02.run_if(should_run),
                sys_texture_ready03.run_if(should_run),
                sys_texture_ready04.run_if(should_run),
                sys_texture_ready05.run_if(should_run),
                sys_texture_ready06.run_if(should_run),
                sys_texture_ready07.run_if(should_run),
                sys_texture_ready08.run_if(should_run),
            ).in_set(ERunStageChap::Uniform)
        );
        app.add_systems(
            (
                sys_effect_bind_to_model_while_mat_modify.run_if(should_run), // ::<PassID01>,
                // sys_effect_bind_to_model_while_mat_modify::<PassID02>,
                // sys_effect_bind_to_model_while_mat_modify::<PassID03>,
                // sys_effect_bind_to_model_while_mat_modify::<PassID04>,
                // sys_effect_bind_to_model_while_mat_modify::<PassID05>,
                // sys_effect_bind_to_model_while_mat_modify::<PassID06>,
                // sys_effect_bind_to_model_while_mat_modify::<PassID07>,
                // sys_effect_bind_to_model_while_mat_modify::<PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );
        app.add_systems(
            (
                sys_effect_tex_to_model_while_mat_modify.run_if(should_run), //::<PassID01>,
                // sys_effect_tex_to_model_while_mat_modify::<PassID02>,
                // sys_effect_tex_to_model_while_mat_modify::<PassID03>,
                // sys_effect_tex_to_model_while_mat_modify::<PassID04>,
                // sys_effect_tex_to_model_while_mat_modify::<PassID05>,
                // sys_effect_tex_to_model_while_mat_modify::<PassID06>,
                // sys_effect_tex_to_model_while_mat_modify::<PassID07>,
                // sys_effect_tex_to_model_while_mat_modify::<PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );
        app.add_system(
            sys_material_uniform_apply.run_if(should_run).in_set(ERunStageChap::Uniform)
        );

        app.add_system(
            sys_dispose_about_material.run_if(should_run).after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );
        // PluginMaterialUniforms.build(app);
        // app.add_plugin(PluginMaterialUniforms);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();
    //     world.insert_resource(SingleMatCreateCommands::default());
    //     world.insert_resource(SingleMaterialIDCommandList::default());

    //     SysMaterailCreateCommands::setup(world, stages.query_stage::<SysMaterailCreateCommands>(ERunStageChap::Initial));

    //     PluginAssetSyncNotNeedLoad::<KeyShader3D, Shader3D>::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);
    //     PluginAssetShaderEffectLoad::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);

    //     let world = engine.world_mut();
    //     SysMaterialIDCommand::setup(world, stages.query_stage::<SysMaterialIDCommand>(ERunStageChap::Initial));

    //     PluginMaterialUniforms.init(engine, stages);

    //     Ok(())
    // }
}

pub struct PluginGroupMaterial;
impl PluginGroupMaterial {
    pub fn add(group: PluginGroupBuilder) -> PluginGroupBuilder {
        group
            .add(PluginMaterial)
            .add(PluginTextureSlot01Load::default())
            .add(PluginTextureSlot02Load::default())
            .add(PluginTextureSlot03Load::default())
            .add(PluginTextureSlot04Load::default())
            .add(PluginTextureSlot05Load::default())
            .add(PluginTextureSlot06Load::default())
    }
}