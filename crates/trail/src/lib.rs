use command_sys::sys_act_trail_mesh_geometry;
use pi_engine_shell::prelude::*;

mod base;
mod command_sys;
mod command;
mod system;

pub use base::*;
pub use command::*;
use pi_scene_context::{transforms::transform_node_sys::sys_world_matrix_calc2, prelude::sys_dispose_ready};
pub use system::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ResTrailBuffer(pub Option<TrailBuffer>);

#[derive(SystemParam)]
pub struct ActionSetTrialRenderer<'w> {
    pub trail: ResMut<'w, ActionListTrial>,
    pub trailbuffer: ResMut<'w, ResTrailBuffer>,
}

pub struct PluginTrail;
impl Plugin for PluginTrail {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<TrailBuffer>();
        let maxcount = cfg.max;
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();

        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let trialbuffer = TrailBuffer::new(maxcount as u32, &mut allocator, &device, &queue);

        app.insert_resource(ResTrailBuffer(trialbuffer));
        app.add_system(sys_act_trail_mesh_geometry.in_set(ERunStageChap::Initial));
        
        app.add_system(sys_trail_update.in_set(ERunStageChap::CalcRenderMatrix));
        app.add_systems(
            (
                sys_dispose_about_trail_linked,
                sys_dispose_about_trail
            ).chain().after(sys_dispose_ready).in_set(ERunStageChap::StateCheck)
        );
    }
}