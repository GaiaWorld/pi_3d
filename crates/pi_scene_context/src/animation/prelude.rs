
use pi_engine_shell::prelude::*;

pub use super::base::*;
pub use super::command::*;

#[derive(SystemParam)]
pub struct ActionSetAnimationGroup<'w> {
    pub create: ResMut<'w, ActionListAnimeGroupCreate>,
    pub add_target_anime: ResMut<'w, ActionListAddTargetAnime>,
    pub start: ResMut<'w, ActionListAnimeGroupStart>,
    pub pause: ResMut<'w, ActionListAnimeGroupPause>,
    pub scene_ctxs: ResMut<'w, SceneAnimationContextMap>,
}
