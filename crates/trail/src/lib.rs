use command_sys::sys_act_trail_mesh_geometry;
use pi_engine_shell::prelude::*;

mod base;
mod command_sys;
mod command;

pub use base::*;
pub use command::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ResTrialBuffer(pub Option<TrialBuffer>);

#[derive(SystemParam)]
pub struct ActionSetTrialRenderer<'w> {
    pub trail: ResMut<'w, ActionListTrialMeshGeometry>,
    pub trailbuffer: ResMut<'w, ResTrialBuffer>,
}

pub struct PluginTrail;
impl Plugin for PluginTrail {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<TrialBuffer>();
        let maxcount = cfg.max;
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();

        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let trialbuffer = TrialBuffer::new(maxcount as u32, &mut allocator, &device, &queue);

        app.insert_resource(ResTrialBuffer(trialbuffer));
        app.add_system(sys_act_trail_mesh_geometry.in_set(ERunStageChap::Initial));
    }
}