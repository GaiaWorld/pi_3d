use std::{mem::replace};

use pi_engine_shell::prelude::*;
use pi_scene_math::Number;

use crate::renderers::{DirtyViewerRenderersInfo, ViewerRenderersInfo};

use super::*;


#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: Number,
    pub y: Number,
    pub w: Number,
    pub h: Number,
}
impl Default for Viewport {
    fn default() -> Self {
        Self { x: 0., y: 0., w: 1., h: 1. }
    }
}

pub struct BundleViewer(
    ViewerViewMatrix,
    ViewerProjectionMatrix,
    ViewerTransformMatrix,
    ViewerGlobalPosition,
    ViewerDirection,
    // ModelList,
    // FlagModelList,
    ModelListAfterCulling,
    // ViewerRenderersInfo,
    DirtyViewerRenderersInfo,
);

pub struct ActionViewer;
impl ActionViewer {
    pub(crate) fn as_viewer(
        commands: &mut EntityCommands,
    ) {
        commands
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
