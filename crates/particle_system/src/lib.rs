
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

#[cfg(feature = "use_bevy")]
        app.configure_sets(
            Update, 
            (
                StageParticleSystem::ParticleSysCreate.after(StageTrail::_TrailCreate),
                StageParticleSystem::_ParticleSysCreate.after(StageParticleSystem::ParticleSysCreate).before(StageTransform::TransformCommand).before(StageEnable::Command),
                StageParticleSystem::ParticleSysCommand.after(StageParticleSystem::_ParticleSysCreate),
                StageParticleSystem::ParticleSysEmission.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysCommand),
                StageParticleSystem::ParticleSysParamStart.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysEmission),
                StageParticleSystem::ParticleSysParamOverLifetime.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamStart),
                StageParticleSystem::ParticleSysDirection.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamOverLifetime),
                StageParticleSystem::ParticleSysParamBySpeed.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysDirection),
                StageParticleSystem::ParticleSysMatrix.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamBySpeed).after(StageTransform::TransformCalcMatrix),
                StageParticleSystem::ParticleSysUpdate.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysMatrix).after(StageModel::InstanceEffectGeometry).after(StageGeometry::_VertexBufferLoadedApply).before(StageGeometry::GeometryLoaded).before(ERunStageChap::Uniform),
            )
        );
#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update,
            (
                apply_deferred.in_set(StageParticleSystem::_ParticleSysCreate),
                sys_create_particle_calculator.in_set(StageScene::Create),
                sys_create_cpu_partilce_system.in_set(StageParticleSystem::ParticleSysCreate),
                sys_act_partilce_system_state.in_set(StageParticleSystem::ParticleSysCommand),
                (
                    sys_ids                 ,
                    sys_emission            ,
                ).chain().in_set(StageParticleSystem::ParticleSysEmission),
                sys_start.after(sys_emission).in_set(StageParticleSystem::ParticleSysParamStart),
                sys_over_lifetime.in_set(StageParticleSystem::ParticleSysParamOverLifetime),
                sys_direction.in_set(StageParticleSystem::ParticleSysDirection),
                sys_by_speed.in_set(StageParticleSystem::ParticleSysParamBySpeed),
                (
                    sys_particle_active ,
                    sys_emitmatrix      ,
                    sys_prewarm         ,
                ).chain().in_set(StageParticleSystem::ParticleSysMatrix),
                (
                    sys_update_buffer           ,
                    sys_update_buffer_trail     ,
                ).chain().in_set(StageParticleSystem::ParticleSysUpdate),
                sys_dispose_about_particle_system.after(sys_dispose_ready).in_set(ERunStageChap::StateCheck),
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(Update, StageParticleSystem::ParticleSysCreate.after(StageTrail::_TrailCreate))
        .configure_set(Update, StageParticleSystem::_ParticleSysCreate.after(StageParticleSystem::ParticleSysCreate).before(StageTransform::TransformCommand).before(StageEnable::Command))
        .configure_set(Update, StageParticleSystem::ParticleSysCommand.after(StageParticleSystem::_ParticleSysCreate))
        .configure_set(Update, StageParticleSystem::ParticleSysEmission.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysCommand))
        .configure_set(Update, StageParticleSystem::ParticleSysParamStart.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysEmission))
        .configure_set(Update, StageParticleSystem::ParticleSysParamOverLifetime.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamStart))
        .configure_set(Update, StageParticleSystem::ParticleSysDirection.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamOverLifetime))
        .configure_set(Update, StageParticleSystem::ParticleSysParamBySpeed.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysDirection))
        .configure_set(Update, StageParticleSystem::ParticleSysMatrix.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysParamBySpeed).after(StageTransform::TransformCalcMatrix))
        .configure_set(Update, StageParticleSystem::ParticleSysUpdate.in_set(FrameDataPrepare).after(StageParticleSystem::ParticleSysMatrix).after(StageModel::InstanceEffectGeometry).after(StageGeometry::_VertexBufferLoadedApply).before(StageGeometry::GeometryLoaded).before(ERunStageChap::Uniform))
        ;

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(Update, sys_create_particle_calculator.in_set(StageScene::Create))
        .add_systems(Update, sys_create_cpu_partilce_system.in_set(StageParticleSystem::ParticleSysCreate))
        .add_systems(Update, sys_act_partilce_system_state.in_set(StageParticleSystem::ParticleSysCommand))
        .add_systems(Update,sys_ids.in_set(StageParticleSystem::ParticleSysEmission))
        .add_systems(Update,sys_emission.after(sys_ids).in_set(StageParticleSystem::ParticleSysEmission))
        .add_systems(Update, sys_start.after(sys_emission).in_set(StageParticleSystem::ParticleSysParamStart))
        .add_systems(Update, sys_over_lifetime.in_set(StageParticleSystem::ParticleSysParamOverLifetime))
        .add_systems(Update, sys_direction.in_set(StageParticleSystem::ParticleSysDirection))
        .add_systems(Update, sys_by_speed.in_set(StageParticleSystem::ParticleSysParamBySpeed))
        .add_systems(Update, sys_particle_active .in_set(StageParticleSystem::ParticleSysMatrix))
        .add_systems(Update, sys_emitmatrix      .after(sys_particle_active).in_set(StageParticleSystem::ParticleSysMatrix))
        .add_systems(Update, sys_prewarm         .after(sys_emitmatrix).in_set(StageParticleSystem::ParticleSysMatrix))
        .add_systems(Update, sys_update_buffer           .in_set(StageParticleSystem::ParticleSysUpdate))
        .add_systems(Update, sys_update_buffer_trail     .after(sys_update_buffer).in_set(StageParticleSystem::ParticleSysUpdate))
        .add_systems(Update, sys_dispose_about_particle_system.after(sys_dispose_ready).in_set(ERunStageChap::StateCheck))
        ;
    }
}

