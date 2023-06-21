
pub enum ErrorPlugin {
    StageError(&'static str),
    ResourceError(&'static str),
}

pub use bevy::prelude::Plugin;
