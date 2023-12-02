

use pi_engine_shell::prelude::*;

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

pub struct ActionViewer;
impl ActionViewer {
    pub(crate) fn as_viewer(
        commands: &mut EntityCommands,
        active: bool,
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
            .insert(ViewerActive(active))
            .insert(ViewerRenderersInfo::default())
            .insert(DirtyViewerRenderersInfo)
            .insert(ForceIncludeModelList::default())
            .insert(FlagForceIncludeModelList)
        ;
    }
}
