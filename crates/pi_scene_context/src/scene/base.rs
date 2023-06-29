
use pi_engine_shell::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct SceneAnimationEnable(pub bool);
impl Default for SceneAnimationEnable {
    fn default() -> Self {
        Self(true)
    }
}
