
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
use pi_scene_context::transforms::transform_node_sys::sys_world_matrix_calc;
use pi_trail_renderer::TrailBuffer;
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
        
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<ResParticleTrailBuffer>();
        let maxbytes = cfg.max;
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();
        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let trailbuffer = TrailBuffer::new(maxbytes as u32, &mut allocator, &device, &queue);
        app.insert_resource(ResParticleTrailBuffer(trailbuffer));

        app.add_systems(Update, 
            sys_act_particle_calculator.in_set(ERunStageChap::Initial),
        );
        app.add_systems(Update, 
            sys_act_create_cpu_partilce_system.in_set(ERunStageChap::SecondInitial),
        );
        app.add_systems(
			Update,
            (
                sys_act_particle_system_trail_material,
                sys_act_partilce_system_state.run_if(should_run),
                sys_ids.run_if(should_run),
                sys_emission.run_if(should_run)
            ).chain().in_set(ERunStageChap::Command),
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
            ).after(sys_emission).in_set(ERunStageChap::Command),
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
            ).after(sys_emitter).in_set(ERunStageChap::Command),
        );
        app.add_systems(Update, 
            sys_direction.run_if(should_run).after(sys_velocity_over_life_time).in_set(ERunStageChap::Command),
        );
        app.add_systems(
			Update,
            (
                sys_color_by_speed.run_if(should_run),
                sys_size_by_speed.run_if(should_run),
                sys_rotation_by_speed.run_if(should_run),
            ).after(sys_direction).in_set(ERunStageChap::Command),
        );
        app.add_systems(Update, 
            sys_emitmatrix.run_if(should_run).after(sys_world_matrix_calc).in_set(ERunStageChap::CalcWorldMatrix),
        );
        app.add_systems(Update, 
            sys_update_buffer.run_if(should_run).in_set(ERunStageChap::Uniform),
        );
        app.add_systems(Update, 
            sys_update_buffer_trail.run_if(should_run).after(sys_update_buffer).in_set(ERunStageChap::Uniform),
        );
    }
}

