use std::time::{Instant, Duration};

use crate::{prelude::*, run_stage::ERunStageChap};

use crate::engine_shell::EnginShell;

#[derive(Debug, Resource)]
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
        &mut self,
        ms: u64,
    ) -> &Self;
}

impl InterfaceFrameTime for EnginShell {
    fn frame_time(
        &mut self,
        ms: u64,
    ) -> &Self {
        let mut frame = self.world.get_resource_mut::<SingleFrameTimeCommand>().unwrap();
        frame.frame_ms = ms;

        self
    }
}

pub fn sys_frame_time(
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
    
    log::debug!("Frame Time: {:?}", frame.last - last);
}



pub struct PluginFrameTime;
impl Plugin for PluginFrameTime {

    fn build(&self, app: &mut bevy::prelude::App) {
        app.world.insert_resource(SingleFrameTimeCommand::default());

        app.add_system(sys_frame_time.in_set(ERunStageChap::Initial));
    }
}