
mod ecs;
mod ecs_bevy;

#[cfg(feature = "use_bevy")]
pub use ecs_bevy::*;
#[cfg(not(feature = "use_bevy"))]
pub use ecs::*;