
pub enum ErrorPlugin {
    StageError(&'static str),
    ResourceError(&'static str),
}

#[cfg(feature = "use_bevy")]
pub use bevy_app::prelude::Plugin;
#[cfg(not(feature = "use_bevy"))]
pub use pi_world::prelude::Plugin;
