
pub enum ErrorPlugin {
    StageError(&'static str),
    ResourceError(&'static str),
}

#[cfg(not(feature = "use_bevy"))]
pub use pi_world::prelude::Plugin;
