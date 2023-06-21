

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
