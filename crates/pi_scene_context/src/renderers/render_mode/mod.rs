

use pi_scene_shell::{add_component, prelude::{pi_world::editor::EntityEditor, *}};


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
        entity: Entity,
        editor: &mut EntityEditor,
        val: ERenderMode,
    ) {
        add_component(editor, entity, val);
        // commands.alter(entity, (val,));
    }
}
