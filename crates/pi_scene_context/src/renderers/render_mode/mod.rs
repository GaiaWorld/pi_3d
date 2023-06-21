

use pi_engine_shell::prelude::*;

#[derive(Debug, Component)]
pub struct RenderMode(pub ERenderMode);


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum ERenderMode {
    Opaque = 1,
    Skybox = 2,
    AlphaTest = 3,
    Transparent = 4,
}

pub struct ActionRenderMode;
impl ActionRenderMode {
    pub fn modify(
        commands: &mut EntityCommands,
        val: ERenderMode,
    ) {
        commands.insert(val);
    }
}
