use std::time::{Instant, Duration};

use pi_ecs::{prelude::{StageBuilder, ResMut, Setup}, world::World};
use pi_ecs_macros::setup;

use crate::engine_shell::EnginShell;

#[derive(Debug)]
pub struct SingleFrameTimeCommand {
    pub last: Instant,
    pub frame_ms: u64,
}
impl Default for SingleFrameTimeCommand {
    fn default() -> Self {
        Self {
            last: Instant::now(),
            frame_ms: 2,
        }
    }
}

pub trait InterfaceFrameTime {
    fn frame_time(
        &self,
        ms: u64,
    ) -> &Self;
}

impl InterfaceFrameTime for EnginShell {
    fn frame_time(
        &self,
        ms: u64,
    ) -> &Self {
        let frame = self.world().get_resource_mut::<SingleFrameTimeCommand>().unwrap();
        frame.frame_ms = ms;

        self
    }
}

struct SysFrameTime;
#[setup]
impl SysFrameTime {
    #[system]
    pub fn sys(
        mut frame: ResMut<SingleFrameTimeCommand>,
    ) {
        let last = frame.last;

        let time = Instant::now();

        if time > last {
            let d = time - last;

            let delay = frame.frame_ms;

            let duration = if d > Duration::from_millis(delay) {
                Duration::from_millis(0)
            } else {
                Duration::from_millis(delay) - d
            };
            spin_sleep::sleep(duration);
        }

        frame.last = Instant::now();
        
        log::info!("Frame Time: {:?}", frame.last - last);
    }
}



pub struct PluginFrameTime;
impl PluginFrameTime {
    pub fn init(
        &mut self,
        world: &mut World,
        stage: &mut StageBuilder,
    ) {
        world.insert_resource(SingleFrameTimeCommand::default());
        
        SysFrameTime::setup(world, stage);
    }
}