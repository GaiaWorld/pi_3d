
use pi_engine_shell::prelude::*;

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
use pi_scene_context::prelude::*;
use pi_trail_renderer::{TrailBuffer, sys_create_trail_mesh};
use system::*;

pub struct PluginParticleSystem;
impl Plugin for PluginParticleSystem {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ParticleSystemCalculatorID>();
        app.insert_resource(ShareAssetMgr::<ParticleSystemCalculatorID>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout));
        app.insert_resource(ResParticleCalculatorUninstallQueue::default());
        app.insert_resource(ActionListCPUParticleCalculator::default());
        app.insert_resource(ActionListCPUParticleSystem::default());
        app.insert_resource(ActionListCPUParticleSystemState::default());
        app.insert_resource(ActionListCPUParticleSystemTrailMaterial::default());
        let mut temp = ParticleSystemPerformance::default(); temp.frame_time_ms = 16; temp.update_frame_time_ms = 33;
        app.insert_resource(temp);

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ResParticleTrailBuffer>();
        let cfg2 = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ResParticleCommonBuffer>();
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();
        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let trailbuffer = TrailBuffer::new(cfg.max as u32, &mut allocator, &device, &queue);
        let particlecommonbuffer= ResParticleCommonBuffer::new(cfg2.max as u32, &mut allocator, &device, &queue);
        app.insert_resource(particlecommonbuffer);
        app.insert_resource(ResParticleTrailBuffer(trailbuffer));

        app.configure_set(Update, StageParticleSystem::ParticleSysCommand.after(ERunStageChap::_InitialApply).after(StageTransform::TransformCommand));
        app.configure_set(Update, StageParticleSystem::ParticleSysEmission.after(StageParticleSystem::ParticleSysCommand));
        app.configure_set(Update, StageParticleSystem::ParticleSysParamStart.after(StageParticleSystem::ParticleSysEmission));
        app.configure_set(Update, StageParticleSystem::ParticleSysParamOverLifetime.after(StageParticleSystem::ParticleSysParamStart));
        app.configure_set(Update, StageParticleSystem::ParticleSysDirection.after(StageParticleSystem::ParticleSysParamOverLifetime));
        app.configure_set(Update, StageParticleSystem::ParticleSysParamBySpeed.after(StageParticleSystem::ParticleSysDirection));
        app.configure_set(Update, StageParticleSystem::ParticleSysMatrix.after(StageParticleSystem::ParticleSysParamBySpeed).after(StageTransform::TransformCalcMatrix));
        app.configure_set(Update, StageParticleSystem::ParticleSysUpdate.after(StageParticleSystem::ParticleSysMatrix).after(StageModel::InstanceEffectGeometry).after(StageGeometry::VertexBufferLoadedApply).before(StageGeometry::GeometryLoaded).before(ERunStageChap::Uniform));

        app.add_systems(Update, 
            sys_create_particle_calculator.in_set(ERunStageChap::Initial),
        );
        app.add_systems(Update, 
            sys_create_cpu_partilce_system.after(sys_create_trail_mesh).in_set(ERunStageChap::Initial),
        );
        app.add_systems(
			Update,
            (
                sys_act_particle_system_trail_material,
                sys_act_partilce_system_state,
            ).chain().in_set(StageParticleSystem::ParticleSysCommand),
        );
        app.add_systems(
			Update,
            (
                sys_ids.run_if(should_run),
                sys_emission.run_if(should_run),
            ).chain().in_set(StageParticleSystem::ParticleSysEmission),
        );
        app.add_systems(
			Update,
            (
                sys_emitter.run_if(should_run),
                sys_start_lifetime.run_if(should_run),
                sys_start_size.run_if(should_run),
                sys_start_rotation.run_if(should_run),
                sys_start_color.run_if(should_run),
                sys_start_texture_sheet.run_if(should_run),
            ).after(sys_emission).in_set(StageParticleSystem::ParticleSysParamStart),
        );
        app.add_systems(
			Update,
            (
                sys_size_over_life_time.run_if(should_run),
                sys_color_over_life_time.run_if(should_run),
                sys_rotation_over_life_time.run_if(should_run),
                sys_velocity_over_life_time.run_if(should_run),
                sys_orbit_over_life_time.run_if(should_run),
                sys_speed_modifier_over_life_time.run_if(should_run),
                sys_limit_velocity_over_life_time.run_if(should_run),
                sys_texturesheet.run_if(should_run),
            ).in_set(StageParticleSystem::ParticleSysParamOverLifetime),
        );
        app.add_systems(Update, 
            sys_direction.run_if(should_run).in_set(StageParticleSystem::ParticleSysDirection),
        );
        app.add_systems(
			Update,
            (
                sys_color_by_speed.run_if(should_run),
                sys_size_by_speed.run_if(should_run),
                sys_rotation_by_speed.run_if(should_run),
            ).in_set(StageParticleSystem::ParticleSysParamBySpeed),
        );
        app.add_systems(Update,
            (
                sys_particle_active,
                sys_emitmatrix.run_if(should_run)
            ).chain().in_set(StageParticleSystem::ParticleSysMatrix),
        );
        app.add_systems(Update, 
            (
                sys_update_buffer.run_if(should_run),
                sys_update_buffer_trail.run_if(should_run),
            ).chain().in_set(StageParticleSystem::ParticleSysUpdate),
        );

        app.add_systems(
			Update, 
            sys_dispose_about_particle_system.run_if(should_run).after(sys_dispose_ready).in_set(ERunStageChap::StateCheck),
        );
    }
}

