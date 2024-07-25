use pi_scene_shell::prelude::*;

#[derive(Component, Default)]
pub struct Sprite {
    pub atlas: Option<KeyTextureFrameAtlas>,
}
