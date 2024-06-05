
use pi_scene_shell::prelude::*;

mod base;
mod command_sys;
mod command;
mod system;

use pi_scene_context::prelude::*;

pub use base::*;
pub use command::*;
pub use command_sys::*;
pub use system::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ResTrailBuffer(pub Option<TrailBuffer>);

#[derive(SystemParam)]
pub struct ActionSetTrailRenderer<'w> {
    pub create: ResMut<'w, ActionListTrail>,
    pub age: ResMut<'w, ActionListTrailAge>,
}

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StageTrail {
    TrailCreate,
    _TrailCreate,
    TrailCommand,
    TrailUpdate,
}

#[derive(Resource, Default)]
pub struct StateTrail {
    pub count: u32,
    pub vertexs: u32,
    pub calc_time: u32,
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
        app.insert_resource(StateTrail::default());

        app.configure_sets(
            Update,
            (
                StageTrail::TrailCreate.after(StageSkeleton::_SkinCreate),
                StageTrail::_TrailCreate.after(StageTrail::TrailCreate).before(StageTransform::TransformCommand).before(StageEnable::Command),
                StageTrail::TrailCommand.in_set(FrameDataPrepare).after(StageTrail::_TrailCreate),
                StageTrail::TrailUpdate.in_set(FrameDataPrepare).after(StageTrail::TrailCommand).after(StageGeometry::GeometryLoaded),
            )
        );

        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StageTrail::_TrailCreate),
                sys_create_trail_mesh.in_set(StageTrail::TrailCreate),
                sys_act_trail_age.in_set(StageTrail::TrailCommand),
                sys_trail_update.in_set(StageTrail::TrailUpdate),
                (
                    sys_dispose_about_trail_linked,
                    sys_dispose_about_trail
                ).chain().after(sys_dispose_ready).in_set(ERunStageChap::StateCheck)
            )
        );
    }
}