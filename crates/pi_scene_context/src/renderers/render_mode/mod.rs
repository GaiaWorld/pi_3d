

use pi_scene_shell::prelude::*;


pub struct RenderMode(pub ERenderMode);


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, )]
pub enum ERenderMode {
    Opaque = 1,
    Skybox = 2,
    AlphaTest = 3,
    Transparent = 4,
}

pub struct ActionRenderMode;
impl ActionRenderMode {
    pub fn modify(
        entity: Entity,
        commands: &mut Alter<(), (), (ERenderMode,)>,
        val: ERenderMode,
    ) {
        commands.alter(entity, (val,));
    }
}
