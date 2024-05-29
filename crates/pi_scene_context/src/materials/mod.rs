
use pi_scene_shell::prelude::*;


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
    fn build(&self, app: &mut App) {
        {
            app.world.insert_single_res(ImageTextureLoader::default());
            app.world.insert_single_res(StateTextureLoader::default());

            app.configure_set(Update, StageTextureLoad::TextureRequest.in_set(FrameDataPrepare));
            app.configure_set(Update, StageTextureLoad::TextureLoading.in_set(FrameDataPrepare).after(StageTextureLoad::TextureRequest));
            app.configure_set(Update, StageTextureLoad::TextureLoaded.in_set(FrameDataPrepare).after(StageTextureLoad::TextureLoading).before(ERunStageChap::Uniform));
            
            app.add_system(Update,sys_image_texture_load_launch.in_set(StageTextureLoad::TextureLoading));
            app.add_system(Update,sys_image_texture_loaded.after(sys_image_texture_load_launch).in_set(StageTextureLoad::TextureLoading));
            
            app.world.insert_single_res(ImageTextureViewLoader2::default());
            app.add_system(
                Update,
                
                    sys_image_texture_view_load_launch2
                .in_set(StageTextureLoad::TextureRequest)
            );
            app.add_system(
                Update,
                
                    sys_image_texture_view_loaded_check2
                .in_set(StageTextureLoad::TextureLoaded)
            );
        }
        if app.world.get_single_res::<ShareAssetMgr<SamplerRes>>().is_none() {
            let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<SamplerRes>();
            app.world.insert_single_res(
                ShareAssetMgr::<SamplerRes>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_single_res::<ShareAssetMgr<TextureRes>>().is_none() {
            let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<TextureRes>();
            app.world.insert_single_res(
                ShareAssetMgr::<TextureRes>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_single_res::<ShareAssetMgr<ImageTexture>>().is_none() {
            let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<ImageTexture>();
            app.world.insert_single_res(
                ShareAssetMgr::<ImageTexture>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };
        if app.world.get_single_res::<ShareAssetMgr<ImageTextureView>>().is_none() {
            let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<ImageTextureView>();
            app.world.insert_single_res(
                ShareAssetMgr::<ImageTextureView>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)
            );
        };

        let defaulttextures = set_up_uniforms(
            &app.world.get_single_res::<ShareAssetMgr<TextureRes>>().unwrap(),
            &app.world.get_single_res::<PiRenderDevice>().unwrap(),
            &app.world.get_single_res::<PiRenderQueue>().unwrap(),
        );
        app.world.insert_single_res(defaulttextures);
        
        // let entity = app.world.spawn_empty().id();
        let i: pi_world::insert::Inserter<()> = app.world.make_inserter();
        let entity = i.insert(());
        let single = SingleIDBaseDefaultMaterial(entity);
        app.world.insert_single_res(single);

        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<ShaderEffectMeta>();
        app.world.insert_single_res(ShareAssetMgr::<ShaderEffectMeta>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout));

        app.world.insert_single_res(ActionListMaterialCreate::default());
        println!("ActionListMaterialUse init");
        app.world.insert_single_res(ActionListMaterialUse::default());
        app.world.insert_single_res(ActionListUniformFloat::default());
        // app.insert_resource(ActionListUniformInt::default());
        app.world.insert_single_res(ActionListUniformUint::default());
        app.world.insert_single_res(ActionListUniformVec2::default());
        app.world.insert_single_res(ActionListUniformVec3::default());
        app.world.insert_single_res(ActionListUniformVec4::default());
        // app.insert_resource(ActionListUniformMat2::default());
        app.world.insert_single_res(ActionListUniformMat4::default());
        app.world.insert_single_res(ActionListUniformTexture::default());
        app.world.insert_single_res(ActionListUniformTextureFromRenderTarget::default());
        app.world.insert_single_res(ActionListTargetAnimationUniform::default());
        app.world.insert_single_res(StateMaterial::default());

        app.configure_set(Update, StageMaterial::Create.after(StageShadowGenerator::Create));
        app.configure_set(Update, StageMaterial::_Init.after(StageMaterial::Create));
        app.configure_set(Update, StageMaterial::Command.after(StageMaterial::_Init).before(StageTextureLoad::TextureRequest).before(EStageAnimation::Create).before(EStageAnimation::Running));
        app.configure_set(Update, StageMaterial::Ready.in_set(FrameDataPrepare).after(StageMaterial::Command).after(StageTextureLoad::TextureLoaded).before(ERunStageChap::Uniform));
        // app.add_system(Update, apply_deferred.in_set(StageMaterial::_Init));

        app.add_system(
			Update,
                sys_create_material
            .in_set(StageMaterial::Create)
        );

        // app.add_system(Update, 
        //     (
        //         sys_act_target_animation_uniform,
        //         sys_act_material_texture_from_target,
        //     ).in_set(StageMaterial::Command)
        // );
        app.add_system(Update,sys_act_target_animation_uniform.in_set(StageMaterial::Command));
        app.add_system(Update,sys_act_material_texture_from_target.in_set(StageMaterial::Command));

        // app.add_system(Update, 
        //     (
        //         sys_act_material_use,
        //         sys_material_textures_modify,
        //     ).chain().in_set(StageMaterial::Command)
        // );

        app.add_system(Update,sys_act_material_use.in_set(StageMaterial::Command));
        app.add_system(Update,sys_material_textures_modify.after(sys_act_material_use).in_set(StageMaterial::Command));

        // app.add_system(
		// 	Update,
        //     (
        //         // sys_act_uniform,
        //         // sys_act_uniform_by_name,
        //         sys_act_material_value.after(sys_act_target_animation_uniform),
        //         // sys_act_material_mat2.run_if(should_run),
        //         // sys_act_material_vec4,
        //         // sys_act_material_vec2,
        //         // sys_act_material_float,
        //         // sys_act_material_int.run_if(should_run),
        //         // sys_act_material_uint,
        //         sys_act_material_texture,
        //     ).before(sys_material_textures_modify).after(sys_act_material_texture_from_target).chain().in_set(StageMaterial::Command)
        //     // .after(sys_sync_load_check_await::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>)
        // );
        app.add_system(Update,sys_act_material_value.after(sys_act_target_animation_uniform).before(sys_material_textures_modify).after(sys_act_material_texture_from_target).in_set(StageMaterial::Command));
        app.add_system(Update,sys_act_material_texture.before(sys_material_textures_modify).after(sys_act_material_texture_from_target).in_set(StageMaterial::Command));

        app.add_system(
			Update,
            
                sys_texture_ready07
            .in_set(StageMaterial::Ready)
        );
        app.add_system(Update, 
                sys_material_uniform_apply.in_set(ERunStageChap::Uniform)
        );

        app.add_system(Update, 
            sys_dispose_about_material.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );
    }
}

pub struct PluginGroupMaterial;
impl Plugin for PluginGroupMaterial {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginMaterial);
    }
    // pub fn add(group: PluginGroupBuilder) -> PluginGroupBuilder {
    //     group.add(PluginMaterial)
    // }
}