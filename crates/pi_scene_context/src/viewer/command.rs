use std::{mem::replace};

use pi_engine_shell::prelude::*;
use pi_scene_math::Number;

use crate::renderers::prelude::*;

use super::base::*;


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
