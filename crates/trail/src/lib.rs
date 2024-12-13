
use pi_scene_shell::prelude::*;

mod base;
mod command_sys;
mod command;
mod system;

use pi_scene_context::{prelude::*, scene::StageScene};

pub use base::*;
pub use command::*;
pub use command_sys::*;
pub use system::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ResTrailBuffer(pub Option<TrailBuffer>);
impl MemSize for ResTrailBuffer {
    fn memsize(&self) -> usize {
        if let Some(item) = &self.0 {
            item.vertices.capacity() * 4
            + 64
        } else {
            64
        }
    }
}

#[derive(Resource)]
pub struct ArgTrailBufferSize(pub usize);

#[derive(SystemParam)]
pub struct ActionSetTrailRenderer<'w> {
    pub create: ResMut<'w, ActionListTrail>,
    pub age: ResMut<'w, ActionListTrailAge>,
}
impl<'w> MemSize for ActionSetTrailRenderer<'w> {
    fn memsize(&self) -> usize {
        self.create.memsize()
        + self.age.memsize()
    }
}

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StageTrail {
    TrailCreate,
    _TrailCreate,
    TrailCommand,
    TrailUpdate,
    TrailDispose,
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
        let maxcount = if let Some(arg) = app.world.get_resource::<ArgTrailBufferSize>() {
            arg.0 as u32
        } else {  4 * 1024 * 1024 };

        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();

        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let trailbuffer = TrailBuffer::new(maxcount as u32, &mut allocator, &device, &queue);
        app.insert_resource(ResTrailBuffer(trailbuffer));

        app.insert_resource(ActionListTrail::default());
        app.insert_resource(ActionListTrailAge::default());
        app.insert_resource(StateTrail::default());
#[cfg(target_feature = "use_bevy")]
        app.configure_sets(
            Update,
            (
                StageTrail::TrailCreate.after(StageSkeleton::_SkinCreate),
                StageTrail::_TrailCreate.after(StageTrail::TrailCreate).before(StageTransform::TransformCommand).before(StageEnable::Command),
                StageTrail::TrailCommand.in_set(FrameDataPrepare).after(StageTrail::_TrailCreate),
                StageTrail::TrailUpdate.in_set(FrameDataPrepare).after(StageTrail::TrailCommand).after(StageGeometry::GeometryLoaded),
            )
        );

#[cfg(target_feature = "use_bevy")]
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
        
#[cfg(not(target_feature = "use_bevy"))]
    app
        .configure_set( Update, StageTrail::TrailCreate .in_set(ERunStageChap::Create).after(StageSkeleton::_SkinCreate))
        .configure_set( Update, StageTrail::_TrailCreate.in_set(ERunStageChap::Create).after(StageTrail::TrailCreate).before(StageTransform::TransformCommand).before(StageEnable::Command))
        .configure_set( Update, StageTrail::TrailCommand.in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageTrail::_TrailCreate))
        .configure_set( Update, StageTrail::TrailUpdate .in_set(ERunStageChap::Culled).in_set(FrameDataPrepare).after(StageTrail::TrailCommand).after(StageGeometry::GeometryLoaded))
        .configure_set( Update, StageTrail::TrailDispose.in_set(ERunStageChap::Dispose).before(StageTransform::TransformDispose))
        ;

#[cfg(not(target_feature = "use_bevy"))]
        app
        .add_systems(Update, sys_create_trail_mesh       .in_set(StageTrail::TrailCreate))
        .add_systems(Update, sys_act_trail_age           .in_set(StageTrail::TrailCommand))
        .add_systems(Update, sys_trail_update            .in_set(StageTrail::TrailUpdate))
        .add_systems(Update, sys_dispose_about_trail_linked      .after(sys_dispose_ready)      .before(sys_dispose_can)         .in_set(StageTrail::TrailDispose))
        .add_systems(Update, sys_dispose_about_trail             .after(sys_dispose_about_trail_linked).before(sys_dispose_can)  .in_set(StageTrail::TrailDispose))
        ;
    }
}