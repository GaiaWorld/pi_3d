

use pi_scene_shell::prelude::*;

#[derive(Component, Default)]
pub struct RenderMode(pub ERenderMode);


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Default)]
pub enum ERenderMode {
    #[default]
    Opaque = 1,
    Skybox = 2,
    AlphaTest = 3,
    Transparent = 4,
}
