use std::{mem::replace};

use pi_engine_shell::prelude::*;
use pi_scene_math::Number;

use crate::renderers::prelude::*;

use super::base::*;

pub struct ActionViewer;
impl ActionViewer {
    pub(crate) fn as_viewer(
        commands: &mut EntityCommands,
    ) {
        commands
            // .insert(ViewerSize::default()) // 由具体视口设置 - 相机\阴影生成器
            .insert(ViewerAspect::default())
            .insert(ViewerViewMatrix::default())
            .insert(ViewerProjectionMatrix::default())
            .insert(ViewerTransformMatrix::default())
            .insert(ViewerGlobalPosition::default())
            .insert(ViewerDirection::default())
            .insert(ModelList::default())
            .insert(FlagModelList::default())
            .insert(ModelListAfterCulling::default())
            .insert(ViewerActive(false))
            .insert(ViewerRenderersInfo::default())
            .insert(DirtyViewerRenderersInfo);
    }
}
