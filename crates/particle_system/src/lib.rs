
use pi_scene_shell::prelude::*;

mod tools;
mod base;
mod asset;
mod emitter;
mod modifier;
mod interpolation;
mod particle_system_tool;
mod mesh_particle_system;
mod particle;
mod iparticle_system_config;
mod command;
mod command_sys;
mod system;
mod extend;
pub mod prelude;

use base::*;
use command::*;
use command_sys::*;
use pi_scene_context::{prelude::*, scene::StageScene};
use pi_trail_renderer::{TrailBuffer, StageTrail};
use system::*;

pub struct PluginParticleSystem;
impl Plugin for PluginParticleSystem {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<ParticleSystemCalculatorID>();
        app.world.insert_single_res(ShareAssetMgr::<ParticleSystemCalculatorID>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout));
        app.world.insert_single_res(ResParticleCalculatorUninstallQueue::default());
        app.world.insert_single_res(ActionListCPUParticleCalculator::default());
        app.world.insert_single_res(ActionListCPUParticleSystem::default());
        app.world.insert_single_res(ActionListCPUParticleSystemState::default());
        app.world.insert_single_res(ActionListCPUParticleSystemTrailMaterial::default());
        let mut temp = ParticleSystemPerformance::default(); temp.frame_time_ms = 16; temp.update_frame_time_ms = 50;
        app.world.insert_single_res(temp);

        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<ResParticleTrailBuffer>();
        // let cfg2 = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<ResParticleCommonBuffer>();
        let device = app.world.get_single_res::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_single_res::<PiRenderQueue>().unwrap().0.clone();
        let mut allocator = app.world.get_single_res_mut::<VertexBufferAllocator3D>().unwrap();
        let trailbuffer = TrailBuffer::new(cfg.max as u32, &mut allocator, &device, &queue);
        // let particlecommonbuffer= ResParticleCommonBuffer::new(cfg2.max as u32, &mut allocator, &device, &queue);
        // app.world.insert_single_res(particlecommonbuffer);
        app.world.insert_single_res(ResParticleTrailBuffer(trailbuffer));

        // app.configure_set(Update, StageParticleSystem::ParticleSysCreate.after(StageTrail::TrailCreate));
        // app.configure_set(Update, StageParticleSystem::_ParticleSysCreate.after(StageParticleSystem::ParticleSysCreate).before(StageTransform::TransformCommand));
        // app.configure_set(Update, StageParticleSystem::ParticleSysCommand.after(StageParticleSystem::_ParticleSysCreate));
        // app.configure_set(Update, StageParticleSystem::ParticleSysEmission.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysCommand));
        // app.configure_set(Update, StageParticleSystem::ParticleSysParamStart.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysEmission));
        // app.configure_set(Update, StageParticleSystem::ParticleSysParamOverLifetime.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamStart));
        // app.configure_set(Update, StageParticleSystem::ParticleSysDirection.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamOverLifetime));
        // app.configure_set(Update, StageParticleSystem::ParticleSysParamBySpeed.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysDirection));
        // app.configure_set(Update, StageParticleSystem::ParticleSysMatrix.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamBySpeed).after(StageTransform::TransformCalcMatrix));
        // app.configure_set(Update, StageParticleSystem::ParticleSysUpdate.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysMatrix).after(StageModel::InstanceEffectGeometry).after(StageGeometry::_VertexBufferLoadedApply).before(StageGeometry::GeometryLoaded).before(ERunStageChap::Uniform));

        // app.add_system(Update, apply_deferred.in_set(StageParticleSystem::_ParticleSysCreate));

        app.add_system(Update, 
            sys_create_particle_calculator/* .in_set(StageScene::Create) */,
        );
        // app.add_system(Update, 
        //     sys_create_cpu_partilce_system/* .in_set(StageParticleSystem::ParticleSysCreate) */,
        // );
        app.add_system(
			Update,
            // (
                sys_act_particle_system_trail_material,
            //     sys_act_partilce_system_state,
            // ).chain().in_set(StageParticleSystem::ParticleSysCommand),
        );
        app.add_system(
			Update,
            // (
            //     sys_act_particle_system_trail_material,
                sys_act_partilce_system_state,
            // ).chain().in_set(StageParticleSystem::ParticleSysCommand),
        );
        app.add_system(
			Update,
            // (
                sys_ids                 , //.run_if(should_run),
            //     sys_emission            , //.run_if(should_run),
            // ).chain().in_set(StageParticleSystem::ParticleSysEmission),
        );
        app.add_system(
			Update,
            // (
            //     sys_ids                 , //.run_if(should_run),
                sys_emission            , //.run_if(should_run),
            // ).chain().in_set(StageParticleSystem::ParticleSysEmission),
        );
        // app.add_system(
		// 	Update,
        //     (
        //         sys_emitter             , // .run_if(should_run),
        //         sys_start_lifetime      , // .run_if(should_run),
        //         sys_start_size          , // .run_if(should_run),
        //         sys_start_rotation      , // .run_if(should_run),
        //         sys_start_color         , // .run_if(should_run),
        //         sys_start_texture_sheet , // .run_if(should_run),
        //     ).after(sys_emission).in_set(StageParticleSystem::ParticleSysParamStart),
        // );
        app.add_system(Update,sys_emitter);
        app.add_system(Update,sys_start_lifetime);
        app.add_system(Update,sys_start_size,);
        app.add_system(Update,sys_start_rotation);
        app.add_system(Update,sys_start_color);
        app.add_system(Update,sys_start_texture_sheet);
        app.add_system(Update,sys_emission);
        // app.add_system(
		// 	Update,
        //     (
        //         sys_gravity                         , // .run_if(should_run),
        //         sys_size_over_life_time             , // .run_if(should_run),
        //         sys_color_over_life_time            , // .run_if(should_run),
        //         sys_rotation_over_life_time         , // .run_if(should_run),
        //         sys_force_over_life_time            , // .run_if(should_run),
        //         sys_velocity_over_life_time         , // .run_if(should_run),
        //         sys_orbit_over_life_time            , // .run_if(should_run),
        //         sys_speed_modifier_over_life_time   , // .run_if(should_run),
        //         sys_limit_velocity_over_life_time   , // .run_if(should_run),
        //         sys_texturesheet                    , // .run_if(should_run),
        //     ).in_set(StageParticleSystem::ParticleSysParamOverLifetime),
        // );
        app.add_system(Update,sys_gravity);
        app.add_system(Update,sys_size_over_life_time);
        app.add_system(Update,sys_color_over_life_time);
        app.add_system(Update,sys_force_over_life_time);
        app.add_system(Update,sys_velocity_over_life_time);
        app.add_system(Update,sys_orbit_over_life_time);
        app.add_system(Update,sys_rotation_over_life_time);
        app.add_system(Update,sys_speed_modifier_over_life_time);
        app.add_system(Update,sys_limit_velocity_over_life_time);
        app.add_system(Update,sys_texturesheet);

        app.add_system(Update, 
            sys_direction                   //.run_if(should_run)
            // .in_set(StageParticleSystem::ParticleSysDirection),
        );
        // app.add_system(
		// 	Update,
        //     (
        //         sys_color_by_speed              , // .run_if(should_run),
        //         sys_size_by_speed               , // .run_if(should_run),
        //         sys_rotation_by_speed           , // .run_if(should_run),
        //     ).in_set(StageParticleSystem::ParticleSysParamBySpeed),
        // );
        app.add_system(Update,sys_color_by_speed);
        app.add_system(Update,sys_size_by_speed);
        app.add_system(Update,sys_rotation_by_speed);
        // app.add_system(Update,
        //     (
        //         sys_particle_active , //.run_if(should_run),
        //         sys_emitmatrix      , //.run_if(should_run),
        //         sys_prewarm         , //.run_if(should_run),
        //     ).chain().in_set(StageParticleSystem::ParticleSysMatrix),
        // );
        app.add_system(Update,sys_particle_active);
        app.add_system(Update,sys_emitmatrix);
        app.add_system(Update,sys_prewarm);

        // app.add_system(Update, 
        //     (
        //         sys_update_buffer           , //.run_if(should_run),
        //         sys_update_buffer_trail     , //.run_if(should_run),
        //     ).chain().in_set(StageParticleSystem::ParticleSysUpdate),
        // );

        app.add_system(Update,sys_update_buffer);
        app.add_system(Update,sys_update_buffer_trail);
        // app.add_system(Update,sys_prewarm);

        // app.add_system(
		// 	Update, 
        //     sys_dispose_about_particle_system    // .run_if(should_run)
        //         .after(sys_dispose_ready)
        //         .in_set(ERunStageChap::StateCheck),
        // );
        app.add_system(Update, sys_dispose_about_particle_system);
        app.add_system(Update, sys_dispose_ready);
    }
}

