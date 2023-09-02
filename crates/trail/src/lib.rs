
use pi_engine_shell::prelude::*;

mod base;
mod command_sys;
mod command;
mod system;

use pi_scene_context::prelude::sys_dispose_ready;

pub use base::*;
pub use command::*;
pub use command_sys::*;
pub use system::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ResTrailBuffer(pub Option<TrailBuffer>);

#[derive(SystemParam)]
pub struct ActionSetTrailRenderer<'w> {
    pub create: ResMut<'w, ActionListTrail>,
    pub trailbuffer: ResMut<'w, ResTrailBuffer>,
    pub age: ResMut<'w, ActionListTrailAge>,
}

pub struct PluginTrail;
impl Plugin for PluginTrail {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<TrailBuffer>();
        let maxcount = cfg.max;
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();

        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let trailbuffer = TrailBuffer::new(maxcount as u32, &mut allocator, &device, &queue);
        app.insert_resource(ResTrailBuffer(trailbuffer));

        app.insert_resource(ActionListTrail::default());
        app.insert_resource(ActionListTrailAge::default());

        app.add_systems(Update, sys_act_trail_mesh_geometry.in_set(ERunStageChap::Initial));
        app.add_systems(Update, sys_act_trail_age.run_if(should_run).in_set(ERunStageChap::Command));
        
        app.add_systems(Update, sys_trail_update.run_if(should_run).in_set(ERunStageChap::Uniform));
        app.add_systems(
			Update,
            (
                sys_dispose_about_trail_linked,
                sys_dispose_about_trail
            ).chain().after(sys_dispose_ready).in_set(ERunStageChap::StateCheck)
        );
    }
}