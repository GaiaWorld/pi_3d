#[cfg(feature = "use_bevy")]
mod graphic_bevy;
#[cfg(feature = "use_bevy")]
pub use graphic_bevy::*;
#[cfg(not(feature = "use_bevy"))]
mod graphic;
#[cfg(not(feature = "use_bevy"))]
pub use graphic::*;