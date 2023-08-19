
use crate::{prelude::*, run_stage::ERunStageChap};

use crate::engine_shell::EnginShell;

#[derive(Debug, Resource)]
pub struct SingleFrameTimeCommand {
    pub last: pi_time::Instant,
    pub frame_ms: u64,
    pub curr: pi_time::Instant,
}
impl Default for SingleFrameTimeCommand {
    fn default() -> Self {
        Self {
            last: pi_time::Instant::now(),
            frame_ms: 8,
            curr: pi_time::Instant::now(),
        }
    }
}
impl SingleFrameTimeCommand {
    pub fn delta_ms(&self) -> u64 {
        if self.last < self.curr {
            (self.curr - self.last).as_millis() as u64
        } else {
            // log::warn!("Curr {:?}, Last: {:?}", self.curr, self.last);
            0
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
#[cfg(not(target_arch = "wasm32"))]
pub fn sys_frame_time(
    mut frame: ResMut<SingleFrameTimeCommand>,
) {
    let last = frame.last;
    frame.last = frame.curr;

    let time = pi_time::Instant::now();

    if time > last {
        let d: std::time::Duration = time - last;

        let delay = frame.frame_ms;

        let duration = if d > std::time::Duration::from_millis(delay) {
            std::time::Duration::from_millis(0)
        } else {
            std::time::Duration::from_millis(delay) - d
        };
        spin_sleep::sleep(duration);
    }

    frame.curr = pi_time::Instant::now();

    // if frame.curr < frame.last {
    //     log::warn!("Time Error: Last {:}");
    // }
    
    // log::debug!("Frame Time: {:?}", frame.last - last);
}



pub struct PluginFrameTime;
impl Plugin for PluginFrameTime {

    fn build(&self, app: &mut bevy::prelude::App) {
        app.world.insert_resource(SingleFrameTimeCommand::default());

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(sys_frame_time.in_set(ERunStageChap::Initial));
    }
}