

use pi_scene_shell::prelude::{pi_world::editor::EntityEditor, *};

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
        editor: &mut EntityEditor,
        active: bool,
    ) {
        let components = [
            editor.init_component::<ViewerAspect>(),
            editor.init_component::<ViewerViewMatrix>(),
            editor.init_component::<ViewerProjectionMatrix>(),
            editor.init_component::<ViewerTransformMatrix>(),
            editor.init_component::<ViewerGlobalPosition>(),
            editor.init_component::<ViewerDirection>(),
            editor.init_component::<ModelList>(),
            editor.init_component::<FlagModelList>(),
            editor.init_component::<ModelListAfterCulling>(),
            editor.init_component::<ViewerActive>(),
            editor.init_component::<ViewerRenderersInfo>(),
            editor.init_component::<DirtyViewerRenderersInfo>(),
            editor.init_component::<ForceIncludeModelList>(),
            editor.init_component::<FlagForceIncludeModelList>(),
        ];

        editor.add_components(entity, &components).unwrap();
            // .insert(ViewerSize::default()) // 由具体视口设置 - 相机\阴影生成器
          
        // *editor.get_component_unchecked_mut_by_id(entity, components[0]) =  ViewerAspect::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[1]) =   ViewerViewMatrix::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[2]) =   ViewerProjectionMatrix::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[3]) =   ViewerTransformMatrix::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[4]) =   ViewerGlobalPosition::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[5]) =   ViewerDirection::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[6]) =   ModelList::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[7]) =   FlagModelList::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[8]) =   ModelListAfterCulling::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[9]) =   ViewerActive(active);
        // *editor.get_component_unchecked_mut_by_id(entity, components[10]) =   ViewerRenderersInfo::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[11]) =   DirtyViewerRenderersInfo;
        // *editor.get_component_unchecked_mut_by_id(entity, components[12]) =   ForceIncludeModelList::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[13]) =   FlagForceIncludeModelList;
      
    }
}
