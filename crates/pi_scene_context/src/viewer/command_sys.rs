

use pi_scene_shell::prelude::*;

use crate::renderers::prelude::*;

use super::{base::*, command::*};

pub fn sys_act_viewer_force_include(
    mut cmds: ResMut<ActionListViewerForceInclude>,
    mut viewers: Query<
        (&mut ForceIncludeModelList, &mut FlagForceIncludeModelList)
    >,
) {
    cmds.drain().drain(..).for_each(|OpsViewerForceInclude(idviewer, idmodel, doadd)| {
        if let Ok((mut list, mut flag)) = viewers.get_mut(idviewer) {
            if doadd {
                if list.0.insert(idmodel) {
                    *flag = FlagForceIncludeModelList;
                }
            } else {
                list.0.remove(&idmodel);
            }
        }
    });
}

pub type ActionViewerBundle = (
    ViewerAspect,
    ViewerViewMatrix,
    ViewerProjectionMatrix,
    ViewerTransformMatrix,
    ViewerGlobalPosition,
    ViewerDirection,
    ModelList,
    FlagModelList,
    ModelListAfterCulling,
    ViewerActive,
    ViewerRenderersInfo,
    DirtyViewerRenderersInfo,
    ForceIncludeModelList,
    FlagForceIncludeModelList
);
pub struct ActionViewer;
impl ActionViewer {
    pub(crate) fn as_viewer(
        entity: Entity,
        commands: &mut Alter<(), (), ActionViewerBundle>,
        active: bool,
    ) {
        commands.alter(entity, 
            // .insert(ViewerSize::default()) // 由具体视口设置 - 相机\阴影生成器
          (
           ViewerAspect::default(),
           ViewerViewMatrix::default(),
           ViewerProjectionMatrix::default(),
           ViewerTransformMatrix::default(),
           ViewerGlobalPosition::default(),
           ViewerDirection::default(),
           ModelList::default(),
           FlagModelList::default(),
           ModelListAfterCulling::default(),
           ViewerActive(active),
           ViewerRenderersInfo::default(),
           DirtyViewerRenderersInfo,
           ForceIncludeModelList::default(),
           FlagForceIncludeModelList,
        ));
    }
}
