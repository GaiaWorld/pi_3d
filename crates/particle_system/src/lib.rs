
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
        let cfgparticlecalc = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ParticleSystemCalculatorID>();
        let mut temp = ParticleSystemPerformance::default(); temp.frame_time_ms = 16; temp.update_frame_time_ms = 50;

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ResParticleTrailBuffer>();
        // let cfg2 = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ResParticleCommonBuffer>();
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();
        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let trailbuffer = TrailBuffer::new(cfg.max as u32, &mut allocator, &device, &queue);
        // let particlecommonbuffer= ResParticleCommonBuffer::new(cfg2.max as u32, &mut allocator, &device, &queue);
        // app.insert_resource(particlecommonbuffer);
        
        app.insert_resource(ShareAssetMgr::<ParticleSystemCalculatorID>::new(GarbageEmpty(), cfgparticlecalc.flag, cfgparticlecalc.min, cfgparticlecalc.timeout));
        app.insert_resource(ResParticleCalculatorUninstallQueue::default());
        app.insert_resource(ActionListCPUParticleCalculator::default());
        app.insert_resource(ActionListCPUParticleSystem::default());
        app.insert_resource(ActionListCPUParticleSystemState::default());
        app.insert_resource(ActionListCPUParticleSystemTrailMaterial::default());
        app.insert_resource(temp);
        app.insert_resource(ResParticleTrailBuffer(trailbuffer));

        app.configure_set(Update, StageParticleSystem::ParticleSysCreate.after(StageTrail::_TrailCreate));
        app.configure_set(Update, StageParticleSystem::_ParticleSysCreate.after(StageParticleSystem::ParticleSysCreate).before(StageTransform::TransformCommand).before(StageEnable::Command));
        app.configure_set(Update, StageParticleSystem::ParticleSysCommand.after(StageParticleSystem::_ParticleSysCreate));
        app.configure_set(Update, StageParticleSystem::ParticleSysEmission.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysCommand));
        app.configure_set(Update, StageParticleSystem::ParticleSysParamStart.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysEmission));
        app.configure_set(Update, StageParticleSystem::ParticleSysParamOverLifetime.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamStart));
        app.configure_set(Update, StageParticleSystem::ParticleSysDirection.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamOverLifetime));
        app.configure_set(Update, StageParticleSystem::ParticleSysParamBySpeed.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysDirection));
        app.configure_set(Update, StageParticleSystem::ParticleSysMatrix.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamBySpeed).after(StageTransform::TransformCalcMatrix));
        app.configure_set(Update, StageParticleSystem::ParticleSysUpdate.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysMatrix).after(StageModel::InstanceEffectGeometry).after(StageGeometry::_VertexBufferLoadedApply).before(StageGeometry::GeometryLoaded).before(ERunStageChap::Uniform));

        app.add_systems(Update, apply_deferred.in_set(StageParticleSystem::_ParticleSysCreate));

        app.add_systems(Update, 
            sys_create_particle_calculator.in_set(StageScene::Create),
        );
        app.add_systems(Update, 
            sys_create_cpu_partilce_system.in_set(StageParticleSystem::ParticleSysCreate),
        );
        app.add_systems(
			Update,
            (
                // sys_act_particle_system_trail_material,
                sys_act_partilce_system_state,
            ).chain().in_set(StageParticleSystem::ParticleSysCommand),
        );
        app.add_systems(
			Update,
            (
                sys_ids                 , //.run_if(should_run),
                sys_emission            , //.run_if(should_run),
            ).chain().in_set(StageParticleSystem::ParticleSysEmission),
        );
        app.add_systems(
			Update,
            (
                sys_start
                // sys_emitter             , // .run_if(should_run),
                // sys_start_lifetime      , // .run_if(should_run),
                // sys_start_size          , // .run_if(should_run),
                // sys_start_rotation      , // .run_if(should_run),
                // sys_start_color         , // .run_if(should_run),
                // sys_start_texture_sheet , // .run_if(should_run),
            ).after(sys_emission).in_set(StageParticleSystem::ParticleSysParamStart),
        );
        app.add_systems(
			Update,
            (
                sys_over_lifetime,
                // sys_gravity                         , // .run_if(should_run),
                // sys_size_over_life_time             , // .run_if(should_run),
                // sys_color_over_life_time            , // .run_if(should_run),
                // sys_rotation_over_life_time         , // .run_if(should_run),
                // sys_force_over_life_time            , // .run_if(should_run),
                // sys_velocity_over_life_time         , // .run_if(should_run),
                // sys_orbit_over_life_time            , // .run_if(should_run),
                // sys_speed_modifier_over_life_time   , // .run_if(should_run),
                // sys_limit_velocity_over_life_time   , // .run_if(should_run),
                // sys_texturesheet                    , // .run_if(should_run),
            ).in_set(StageParticleSystem::ParticleSysParamOverLifetime),
        );
        app.add_systems(Update, 
            sys_direction                   //.run_if(should_run)
            .in_set(StageParticleSystem::ParticleSysDirection),
        );
        app.add_systems(
			Update,
            (
                sys_by_speed,
                // sys_color_by_speed              , // .run_if(should_run),
                // sys_size_by_speed               , // .run_if(should_run),
                // sys_rotation_by_speed           , // .run_if(should_run),
            ).in_set(StageParticleSystem::ParticleSysParamBySpeed),
        );
        app.add_systems(Update,
            (
                sys_particle_active , //.run_if(should_run),
                sys_emitmatrix      , //.run_if(should_run),
                sys_prewarm         , //.run_if(should_run),
            ).chain().in_set(StageParticleSystem::ParticleSysMatrix),
        );
        app.add_systems(Update, 
            (
                sys_update_buffer           , //.run_if(should_run),
                sys_update_buffer_trail     , //.run_if(should_run),
            ).chain().in_set(StageParticleSystem::ParticleSysUpdate),
        );

        app.add_systems(
			Update, 
            sys_dispose_about_particle_system    // .run_if(should_run)
                .after(sys_dispose_ready)
                .in_set(ERunStageChap::StateCheck),
        );
    }
}

